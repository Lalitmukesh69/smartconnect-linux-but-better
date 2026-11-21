//! Cross Control, at the top of the Display tab: the three modes the Windows app offers, side by
//! side, and where the phone sits next to this PC's screen. The display settings follow below them
//! (`display.rs`); `window.rs` puts the parts together.
//!
//! - **Shared mouse and keyboard** runs `smartconnect-crosscontrol` (svc 26).
//! - **Duplicate display** / **Extend display** run a phone-as-display session (svc 202) through the
//!   Display page (`start_mode("mirror" | "extend")`), so its settings apply.
//!
//! Only one mode runs at a time: picking one stops the other. The phone's position is drawn as a
//! phone next to a screen; dragging it snaps it to the nearest side and stores the side plus the
//! part of that edge it touches in the shared `crosscontrol.json`, which `smartconnect-crosscontrol`
//! reads at startup (also when the daemon starts it for the phone's tile).

use std::cell::{Cell, RefCell};
use std::os::fd::AsRawFd;
use std::path::PathBuf;
use std::rc::{Rc, Weak};
use std::time::Duration;

use adw::prelude::*;
use gtk::{cairo, gdk, glib};
use serde_json::{Value, json};

use crate::components::*;
use crate::display::DisplayPage;
use crate::mirror::{Exit, Mirror};
use crate::theme::{IconKind, ModernIcon};

/// The phone's place next to the screen: a side, and the centre of the phone along that side
/// (0..1 from the top or the left).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Layout {
    pub edge: Edge,
    pub center: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    Left,
    Right,
    Top,
    Bottom,
}

impl Edge {
    fn name(self) -> &'static str {
        match self {
            Edge::Left => "left",
            Edge::Right => "right",
            Edge::Top => "top",
            Edge::Bottom => "bottom",
        }
    }

    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "left" => Edge::Left,
            "right" => Edge::Right,
            "top" => Edge::Top,
            "bottom" => Edge::Bottom,
            _ => return None,
        })
    }

    fn vertical(self) -> bool {
        matches!(self, Edge::Left | Edge::Right)
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self { edge: Edge::Right, center: 0.5 }
    }
}

/// `$XDG_CONFIG_HOME/smartconnect-linux/crosscontrol.json`, shared with `smartconnect-crosscontrol`.
pub fn config_path() -> PathBuf {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .unwrap_or_else(|| PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".config"))
        .join("smartconnect-linux/crosscontrol.json")
}

/// Drawing proportions: screen height as a share of the canvas, phone height as a share of the
/// screen's, phone width as a share of its height (a 20:9 phone in portrait).
const SCREEN_H: f64 = 0.42;
const PHONE_H: f64 = 0.7;
const PHONE_ASPECT: f64 = 9.0 / 20.0;
const GAP: f64 = 6.0;

/// The screens as drawn, in canvas pixels.
#[derive(Clone, Copy, Debug)]
struct Geometry {
    /// Screen: x, y, w, h.
    pc: (f64, f64, f64, f64),
    /// Phone size: w, h.
    phone: (f64, f64),
}

impl Geometry {
    /// `landscape`: draw the phone on its side (how it is held while it shows a display session).
    fn new(w: f64, h: f64, screen_aspect: f64, landscape: bool) -> Self {
        let mut ph = h * SCREEN_H;
        let mut pw = ph * screen_aspect;
        if pw > w * 0.5 {
            pw = w * 0.5;
            ph = pw / screen_aspect;
        }
        let phone_h = ph * PHONE_H;
        let phone = if landscape { (phone_h, phone_h * PHONE_ASPECT) } else { (phone_h * PHONE_ASPECT, phone_h) };
        Self { pc: ((w - pw) / 2.0, (h - ph) / 2.0, pw, ph), phone }
    }

    /// Length of the PC edge the phone covers, as a fraction of that edge.
    fn span_len(&self, edge: Edge) -> f64 {
        let (_, _, w, h) = self.pc;
        if edge.vertical() { (self.phone.1 / h).min(1.0) } else { (self.phone.0 / w).min(1.0) }
    }

    /// Top-left corner of the phone for a layout.
    fn phone_at(&self, l: Layout) -> (f64, f64) {
        let (x, y, w, h) = self.pc;
        let (pw, ph) = self.phone;
        match l.edge {
            Edge::Right => (x + w + GAP, y + l.center * h - ph / 2.0),
            Edge::Left => (x - GAP - pw, y + l.center * h - ph / 2.0),
            Edge::Top => (x + l.center * w - pw / 2.0, y - GAP - ph),
            Edge::Bottom => (x + l.center * w - pw / 2.0, y + h + GAP),
        }
    }

    /// The layout nearest to a phone whose top-left corner is at `(px, py)`.
    fn snap(&self, px: f64, py: f64) -> Layout {
        let (x, y, w, h) = self.pc;
        let (pw, ph) = self.phone;
        let (cx, cy) = (px + pw / 2.0, py + ph / 2.0);
        let nx = (cx - (x + w / 2.0)) / ((w + pw) / 2.0);
        let ny = (cy - (y + h / 2.0)) / ((h + ph) / 2.0);
        let (edge, center) = if nx.abs() >= ny.abs() {
            (if nx >= 0.0 { Edge::Right } else { Edge::Left }, (cy - y) / h)
        } else {
            (if ny >= 0.0 { Edge::Bottom } else { Edge::Top }, (cx - x) / w)
        };
        self.clamped(Layout { edge, center })
    }

    fn clamped(&self, l: Layout) -> Layout {
        let half = self.span_len(l.edge) / 2.0;
        Layout { edge: l.edge, center: l.center.clamp(half, 1.0 - half) }
    }
}

/// Reads `crosscontrol.json`, if there is one.
fn load_layout() -> Option<(Edge, f64, f64)> {
    let v: Value = serde_json::from_str(&std::fs::read_to_string(config_path()).ok()?).ok()?;
    let edge = Edge::parse(v.get("edge")?.as_str()?)?;
    let start = v.get("start").and_then(Value::as_f64).unwrap_or(0.0);
    let end = v.get("end").and_then(Value::as_f64).unwrap_or(1.0);
    (0.0..=1.0).contains(&start).then_some(())?;
    (start < end && end <= 1.0).then_some((edge, start, end))
}

fn save_layout(edge: Edge, start: f64, end: f64) -> std::io::Result<()> {
    let path = config_path();
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let round = |f: f64| (f * 1000.0).round() / 1000.0;
    let text = serde_json::to_string_pretty(&json!({ "edge": edge.name(), "start": round(start), "end": round(end) }))?;
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, text + "\n")?;
    std::fs::rename(tmp, path)
}

fn crosscontrol_lock_path() -> PathBuf {
    std::env::var_os("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir)
        .join("smartconnect-crosscontrol.lock")
}

/// The pid of a running `smartconnect-crosscontrol` (ours, the daemon's or a terminal's), from its
/// lock file: `Some` only while the lock is held.
fn running_crosscontrol() -> Option<libc::pid_t> {
    let path = crosscontrol_lock_path();
    let file = std::fs::File::open(&path).ok()?;
    // SAFETY: flock(2) on a file we own for the duration of the call.
    let free = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) } == 0;
    if free {
        return None; // dropped below, which releases our probe lock
    }
    std::fs::read_to_string(&path).ok()?.trim().parse().ok()
}

/// Which mode is running right now.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Shared,
    Duplicate,
    Extend,
}

pub struct CrossControlPage {
    toasts: adw::ToastOverlay,
    display: Rc<DisplayPage>,
    cards: Vec<(Mode, gtk::ToggleButton, ModernIcon, gtk::Image)>,
    notice: gtk::Box,
    status: gtk::Label,
    canvas: gtk::DrawingArea,
    position: gtk::Label,
    cross: RefCell<Mirror>,
    layout: Cell<Layout>,
    /// The phone is drawn on its side: a display session runs with the phone held landscape.
    landscape: Cell<bool>,
    /// The phone's top-left corner while it is being dragged, and where in the phone it was grabbed.
    drag: Cell<Option<((f64, f64), (f64, f64))>>,
    /// Our client was stopped to apply a new position: start it again once it has exited.
    restart_after_exit: Cell<bool>,
    connected: Cell<bool>,
    syncing: Cell<bool>,
    polling: Cell<bool>,
    on_state: RefCell<Option<Box<dyn Fn()>>>,
}

fn mode_card(kind: IconKind, title: &str, subtitle: &str) -> (gtk::ToggleButton, ModernIcon, gtk::Image) {
    let icon = ModernIcon::new(kind, 24);
    icon.add_css_class("sc-card-icon");

    let title_lbl = gtk::Label::builder()
        .label(title)
        .xalign(0.0)
        .css_classes(["sc-card-title"])
        .wrap(true)
        .build();

    let sub = gtk::Label::builder()
        .label(subtitle)
        .xalign(0.0)
        .css_classes(["sc-card-desc"])
        .wrap(true)
        .build();

    let check = gtk::Image::builder()
        .icon_name("emblem-ok-symbolic")
        .css_classes(["sc-highlight"])
        .visible(false)
        .build();

    let top_row = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(crate::tokens::SPACE_2)
        .build();
    top_row.append(icon.widget());
    let spacer = gtk::Box::builder().hexpand(true).build();
    top_row.append(&spacer);
    top_row.append(&check);

    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(crate::tokens::SPACE_3)
        .valign(gtk::Align::Center)
        .build();
    content.append(&top_row);
    content.append(&title_lbl);
    content.append(&sub);

    let button = gtk::ToggleButton::builder()
        .child(&content)
        .css_classes(["sc-option-card"])
        .hexpand(true)
        .build();
    (button, icon, check)
}

impl CrossControlPage {
    pub fn build(toasts: &adw::ToastOverlay, display: Rc<DisplayPage>) -> (Rc<Self>, gtk::Widget, gtk::Widget) {
        let (header, _, _) = page_header("Display", "Cross Control Studio", None::<&gtk::Widget>);

        let notice = notice_banner(
            IconKind::Smartphone,
            "Phone display is available when connected. Connect from the sidebar to start.",
        );

        let modes = gtk::Box::builder()
            .spacing(crate::tokens::SPACE_4)
            .homogeneous(true)
            .build();
        let mut cards = Vec::new();
        for (mode, kind, name, sub) in [
            (
                Mode::Shared,
                IconKind::Mouse,
                "Shared mouse and keyboard",
                "Move the pointer past the screen edge onto the phone",
            ),
            (
                Mode::Duplicate,
                IconKind::Monitor,
                "Duplicate display",
                "The phone shows this PC's screen",
            ),
            (
                Mode::Extend,
                IconKind::Monitor,
                "Extend display",
                "The phone becomes a second screen",
            ),
        ] {
            let (button, icon, check) = mode_card(kind, name, sub);
            modes.append(&button);
            cards.push((mode, button, icon, check));
        }
        let status = gtk::Label::builder()
            .xalign(0.0)
            .wrap(true)
            .css_classes(["sc-row-helper"])
            .build();

        let mode_col = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(crate::tokens::SPACE_3)
            .build();
        mode_col.append(&modes);
        mode_col.append(&status);

        let mode_section = section("Mode", Some("Select how your phone works with this PC"), &mode_col);

        let top = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(crate::tokens::SPACE_4)
            .build();
        top.append(&header);
        top.append(&notice);
        top.append(&mode_section);

        let canvas = gtk::DrawingArea::builder()
            .content_height(180)
            .hexpand(true)
            .focusable(true)
            .cursor(&gdk::Cursor::from_name("grab", None).expect("cursor"))
            .build();

        let position = gtk::Label::builder()
            .xalign(0.0)
            .wrap(true)
            .css_classes(["sc-row-title"])
            .build();

        let shortcuts = gtk::Label::builder()
            .label("Use arrow keys or drag to reposition · Push pointer back or press Ctrl+Alt+Esc to return to PC")
            .xalign(0.0)
            .wrap(true)
            .css_classes(["sc-row-helper"])
            .build();

        let info_col = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(crate::tokens::SPACE_1)
            .css_classes(["sc-row"])
            .build();
        info_col.append(&position);
        info_col.append(&shortcuts);

        let arrange_surf = surface();
        arrange_surf.append(&canvas);
        arrange_surf.append(&hairline());
        arrange_surf.append(&info_col);

        let arrange = section(
            "Arrangement",
            Some("Drag or use arrow keys to position the phone relative to your PC screen"),
            &arrange_surf,
        );

        let layout = match load_layout() {
            Some((edge, start, end)) => Layout { edge, center: (start + end) / 2.0 },
            None => Layout::default(),
        };
        let page = Rc::new(Self {
            toasts: toasts.clone(),
            display,
            cards,
            notice,
            status,
            canvas,
            position,
            cross: RefCell::new(Mirror::cross_control()),
            layout: Cell::new(layout),
            landscape: Cell::new(false),
            drag: Cell::new(None),
            restart_after_exit: Cell::new(false),
            connected: Cell::new(false),
            syncing: Cell::new(false),
            polling: Cell::new(false),
            on_state: RefCell::default(),
        });
        page.connect();
        page.update_position_label();
        page.sync();
        (page, top.upcast(), arrange.upcast())
    }

    fn connect(self: &Rc<Self>) {
        for (mode, button, _, _) in &self.cards {
            let weak = Rc::downgrade(self);
            let mode = *mode;
            button.connect_toggled(move |_| {
                if let Some(page) = weak.upgrade() {
                    if !page.syncing.get() {
                        page.pick(mode);
                    }
                }
            });
        }

        // Arrow keys reposition phone
        let key_ctrl = gtk::EventControllerKey::new();
        let weak_key = Rc::downgrade(self);
        key_ctrl.connect_key_pressed(move |_, key, _, _| {
            let Some(page) = weak_key.upgrade() else { return glib::Propagation::Proceed };
            let edge = match key {
                gdk::Key::Left => Edge::Left,
                gdk::Key::Right => Edge::Right,
                gdk::Key::Up => Edge::Top,
                gdk::Key::Down => Edge::Bottom,
                _ => return glib::Propagation::Proceed,
            };
            let mut l = page.layout.get();
            l.edge = edge;
            page.set_layout(l);
            page.canvas.queue_draw();
            glib::Propagation::Stop
        });
        self.canvas.add_controller(key_ctrl);

        let weak: Weak<Self> = Rc::downgrade(self);
        self.canvas.set_draw_func(move |area, cr, w, h| {
            if let Some(page) = weak.upgrade() {
                page.draw(area, cr, w as f64, h as f64);
            }
        });

        let drag = gtk::GestureDrag::new();
        let weak = Rc::downgrade(self);
        drag.connect_drag_begin(move |_, x, y| {
            let Some(page) = weak.upgrade() else { return };
            let g = page.geometry();
            let (px, py) = g.phone_at(page.layout.get());
            let (pw, ph) = g.phone;
            if x >= px && x <= px + pw && y >= py && y <= py + ph {
                page.drag.set(Some(((px, py), (x - px, y - py))));
                page.canvas.set_cursor_from_name(Some("grabbing"));
            }
        });
        let weak = Rc::downgrade(self);
        drag.connect_drag_update(move |gesture, dx, dy| {
            let Some(page) = weak.upgrade() else { return };
            let (Some((_, grab)), Some((sx, sy))) = (page.drag.get(), gesture.start_point()) else { return };
            page.drag.set(Some(((sx + dx - grab.0, sy + dy - grab.1), grab)));
            page.canvas.queue_draw();
        });
        let weak = Rc::downgrade(self);
        drag.connect_drag_end(move |_, _, _| {
            let Some(page) = weak.upgrade() else { return };
            let Some(((px, py), _)) = page.drag.take() else { return };
            page.canvas.set_cursor_from_name(Some("grab"));
            let layout = page.geometry().snap(px, py);
            page.set_layout(layout);
        });
        self.canvas.add_controller(drag);

        // Keep the drawing in step with the theme's accent colour.
        let weak = Rc::downgrade(self);
        adw::StyleManager::default().connect_accent_color_rgba_notify(move |_| {
            if let Some(page) = weak.upgrade() {
                page.canvas.queue_draw();
            }
        });
    }

    /// The phone link is up: modes can start.
    pub fn set_connected(self: &Rc<Self>, connected: bool) {
        self.connected.set(connected);
        self.sync();
    }

    pub fn connect_state_changed(&self, f: impl Fn() + 'static) {
        *self.on_state.borrow_mut() = Some(Box::new(f));
    }

    pub fn edge_label(&self) -> &'static str {
        match self.layout.get().edge {
            Edge::Left => "Left",
            Edge::Right => "Right",
            Edge::Top => "Top",
            Edge::Bottom => "Bottom",
        }
    }

    pub fn active_mode_id(&self) -> Option<&'static str> {
        match self.running() {
            Some(Mode::Shared) => Some("mouse"),
            Some(Mode::Duplicate) => Some("duplicate"),
            Some(Mode::Extend) => Some("extend"),
            None => None,
        }
    }

    /// Toggles a Cross Control mode programmatically (e.g. from the Command Palette).
    pub fn toggle_mode_id(self: &Rc<Self>, id: &str) {
        let mode = match id {
            "mouse" => Mode::Shared,
            "duplicate" => Mode::Duplicate,
            "extend" => Mode::Extend,
            _ => return,
        };
        self.pick(mode);
    }

    /// Called when the page is shown or the phone asked for Cross Control: refresh, and poll while
    /// anything runs.
    pub fn refresh(self: &Rc<Self>) {
        self.sync();
        self.poll_soon();
    }

    /// Stops our own Cross Control client when the app quits (a display session keeps running, as
    /// the Display page does).
    pub fn shutdown(&self) {
        self.cross.borrow_mut().stop();
    }

    fn running(&self) -> Option<Mode> {
        let cross = self.cross.borrow();
        if (cross.running() && !cross.stopping()) || (!cross.running() && running_crosscontrol().is_some()) {
            return Some(Mode::Shared);
        }
        match self.display.running_mode().as_deref() {
            Some("mirror") => Some(Mode::Duplicate),
            Some("extend") => Some(Mode::Extend),
            _ => None,
        }
    }

    /// Starts `mode`, or stops it when it is the one running; any other running mode is stopped.
    fn pick(self: &Rc<Self>, mode: Mode) {
        let running = self.running();
        if running == Some(mode) {
            match mode {
                Mode::Shared => self.stop_shared(),
                _ => self.display.stop_session(),
            }
            self.after_change();
            return;
        }
        if !self.connected.get() {
            self.toasts.add_toast(adw::Toast::new("Connect the phone first"));
            self.after_change();
            return;
        }
        match running {
            Some(Mode::Shared) => self.stop_shared(),
            Some(_) if mode == Mode::Shared => self.display.stop_session(),
            _ => {}
        }
        match mode {
            Mode::Shared => self.start_shared(),
            Mode::Duplicate => self.display.start_mode("mirror"),
            Mode::Extend => self.display.start_mode("extend"),
        }
        self.after_change();
    }

    fn start_shared(&self) {
        // The client reads crosscontrol.json itself; a running one (e.g. the daemon's, for the
        // phone's tile) is taken over.
        if let Err(e) = self.cross.borrow_mut().start(&[]) {
            self.toasts.add_toast(adw::Toast::new(&e));
        }
    }

    fn stop_shared(&self) {
        let mut cross = self.cross.borrow_mut();
        if cross.running() {
            cross.stop();
        } else if let Some(pid) = running_crosscontrol() {
            // Started by the daemon (phone tile) or a terminal: SIGINT tells the phone to stop.
            // SAFETY: plain kill(2) on the pid from the client's own lock file.
            unsafe { libc::kill(pid, libc::SIGINT) };
        }
    }

    fn after_change(self: &Rc<Self>) {
        self.sync();
        self.poll_soon();
    }

    /// Polls every 500 ms while something runs or is winding down, to follow helpers that exit
    /// and sessions started elsewhere.
    fn poll_soon(self: &Rc<Self>) {
        if self.polling.replace(true) {
            return;
        }
        let weak = Rc::downgrade(self);
        glib::timeout_add_local(Duration::from_millis(500), move || {
            let Some(page) = weak.upgrade() else { return glib::ControlFlow::Break };
            let exit = page.cross.borrow_mut().poll();
            if let Some(Exit::Failed(msg)) = &exit {
                page.toasts.add_toast(adw::Toast::new(msg));
            }
            if exit.is_some() && page.restart_after_exit.take() {
                page.start_shared();
            }
            page.sync();
            let busy = page.cross.borrow().running() || page.running().is_some();
            if busy {
                glib::ControlFlow::Continue
            } else {
                page.polling.set(false);
                glib::ControlFlow::Break
            }
        });
    }

    fn sync(&self) {
        let running = self.running();
        self.notice.set_visible(!self.connected.get());
        self.syncing.set(true);
        for (mode, button, _, check) in &self.cards {
            let on = running == Some(*mode);
            button.set_active(on);
            check.set_visible(on);
            button.set_sensitive(self.connected.get() || on);
        }
        self.syncing.set(false);
        let text = match (running, self.connected.get()) {
            (Some(Mode::Shared), _) => "Shared mouse and keyboard is on: push the pointer into the highlighted edge.",
            (Some(Mode::Duplicate), _) => "The phone is showing this PC's screen.",
            (Some(Mode::Extend), _) => "The phone is a second screen for this PC.",
            (None, true) => "Pick a mode to start. Pick it again to stop.",
            (None, false) => "Connect the phone to use Cross Control.",
        };
        self.status.set_label(text);
        let landscape = self.display.phone_landscape().unwrap_or(false);
        if self.landscape.replace(landscape) != landscape {
            self.canvas.queue_draw();
        }
        if let Some(f) = self.on_state.borrow().as_ref() {
            f();
        }
    }

    // ── Phone position ──────────────────────────────────────────────────────

    fn screen_aspect(&self) -> f64 {
        let monitor = gdk::Display::default()
            .and_then(|d| d.monitors().item(0))
            .and_then(|m| m.downcast::<gdk::Monitor>().ok());
        match monitor {
            Some(m) if m.geometry().height() > 0 => m.geometry().width() as f64 / m.geometry().height() as f64,
            _ => 16.0 / 10.0,
        }
    }

    fn geometry(&self) -> Geometry {
        Geometry::new(self.canvas.width() as f64, self.canvas.height() as f64, self.screen_aspect(), self.landscape.get())
    }

    fn set_layout(self: &Rc<Self>, layout: Layout) {
        self.layout.set(layout);
        let half = self.geometry().span_len(layout.edge) / 2.0;
        if let Err(e) = save_layout(layout.edge, layout.center - half, layout.center + half) {
            self.toasts.add_toast(adw::Toast::new(&format!("Could not save the phone position: {e}")));
        }
        self.update_position_label();
        self.canvas.queue_draw();
        if let Some(f) = self.on_state.borrow().as_ref() {
            f();
        }
        // A running client only reads the position at startup: restart ours to apply it.
        if self.running() == Some(Mode::Shared) {
            let mut cross = self.cross.borrow_mut();
            if cross.running() {
                cross.stop();
                self.restart_after_exit.set(true);
            } else {
                drop(cross);
                self.start_shared(); // takes over the daemon's client
            }
            self.poll_soon();
        }
    }

    fn update_position_label(&self) {
        let l = self.layout.get();
        let side = match l.edge {
            Edge::Left => "left of",
            Edge::Right => "right of",
            Edge::Top => "above",
            Edge::Bottom => "below",
        };
        self.position.set_label(&format!("The phone is {side} this PC's screen."));
    }

    fn draw(&self, _area: &gtk::DrawingArea, cr: &cairo::Context, w: f64, h: f64) {
        let g = Geometry::new(w, h, self.screen_aspect(), self.landscape.get());
        let layout = g.clamped(self.layout.get());

        // This PC's screen: 1.5px soft outline with label "This PC"
        let (x, y, sw, sh) = g.pc;
        rounded(cr, x, y, sw, sh, 8.0);
        cr.set_source_rgba(1.0, 1.0, 1.0, 0.04);
        let _ = cr.fill_preserve();
        cr.set_source_rgba(0.88, 0.90, 0.95, 0.40);
        cr.set_line_width(1.5);
        let _ = cr.stroke();
        cr.set_source_rgba(1.0, 1.0, 1.0, 0.90);
        label(cr, "This PC", x + sw / 2.0, y + sh / 2.0, 13.0);

        // Active edge bar: 3px accent indicator (#1F7A8C)
        if self.drag.get().is_none() {
            let half = g.span_len(layout.edge) / 2.0;
            let (a, b) = (layout.center - half, layout.center + half);
            let (x1, y1, x2, y2) = match layout.edge {
                Edge::Right => (x + sw, y + a * sh, x + sw, y + b * sh),
                Edge::Left => (x, y + a * sh, x, y + b * sh),
                Edge::Top => (x + a * sw, y, x + b * sw, y),
                Edge::Bottom => (x + a * sw, y + sh, x + b * sw, y + sh),
            };
            cr.set_source_rgba(0.12, 0.48, 0.55, 1.0);
            cr.set_line_width(3.0);
            cr.set_line_cap(cairo::LineCap::Round);
            cr.move_to(x1, y1);
            cr.line_to(x2, y2);
            let _ = cr.stroke();
        }

        // The phone: 1.5px #BFDBF7 outline with accent-tint fill
        let (px, py) = self.drag.get().map(|(p, _)| p).unwrap_or_else(|| g.phone_at(layout));
        let (pw, ph) = g.phone;
        rounded(cr, px, py, pw, ph, 7.0);
        cr.set_source_rgba(0.12, 0.48, 0.55, 0.18);
        let _ = cr.fill_preserve();
        cr.set_source_rgba(0.75, 0.86, 0.97, 1.0);
        cr.set_line_width(1.5);
        let _ = cr.stroke();
        // Speaker slot
        cr.set_source_rgba(0.75, 0.86, 0.97, 0.80);
        rounded(cr, px + pw * 0.35, py + 4.0, pw * 0.3, 2.0, 1.0);
        let _ = cr.fill();
        cr.set_source_rgba(1.0, 1.0, 1.0, 0.90);
        label(cr, "Phone", px + pw / 2.0, py + ph / 2.0, 11.0);
    }
}

fn rounded(cr: &cairo::Context, x: f64, y: f64, w: f64, h: f64, r: f64) {
    let r = r.min(w / 2.0).min(h / 2.0);
    cr.new_sub_path();
    cr.arc(x + w - r, y + r, r, -std::f64::consts::FRAC_PI_2, 0.0);
    cr.arc(x + w - r, y + h - r, r, 0.0, std::f64::consts::FRAC_PI_2);
    cr.arc(x + r, y + h - r, r, std::f64::consts::FRAC_PI_2, std::f64::consts::PI);
    cr.arc(x + r, y + r, r, std::f64::consts::PI, 1.5 * std::f64::consts::PI);
    cr.close_path();
}

fn label(cr: &cairo::Context, text: &str, cx: f64, cy: f64, size: f64) {
    cr.select_font_face("Helvetica", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
    cr.set_font_size(size);
    if let Ok(ext) = cr.text_extents(text) {
        cr.move_to(cx - ext.width() / 2.0 - ext.x_bearing(), cy - ext.height() / 2.0 - ext.y_bearing());
        let _ = cr.show_text(text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn geo() -> Geometry {
        Geometry::new(800.0, 260.0, 16.0 / 10.0, false)
    }

    #[test]
    fn snaps_to_the_nearest_side() {
        let g = geo();
        let (x, y, w, h) = g.pc;
        let (pw, ph) = g.phone;
        assert_eq!(g.snap(x + w + 40.0, y + h / 2.0 - ph / 2.0).edge, Edge::Right);
        assert_eq!(g.snap(x - pw - 40.0, y).edge, Edge::Left);
        assert_eq!(g.snap(x + w / 2.0 - pw / 2.0, y - ph - 30.0).edge, Edge::Top);
        assert_eq!(g.snap(x + w / 2.0 - pw / 2.0, y + h + 30.0).edge, Edge::Bottom);
    }

    #[test]
    fn the_phone_stays_on_its_edge() {
        let g = geo();
        let half = g.span_len(Edge::Right) / 2.0;
        // Right of the screen, dragged down until the phone's top is level with the screen's bottom.
        let far_down = g.snap(g.pc.0 + g.pc.2 + 20.0, g.pc.1 + g.pc.3);
        assert_eq!(far_down.edge, Edge::Right);
        assert!((far_down.center - (1.0 - half)).abs() < 1e-9);
        // A saved layout round-trips through the drawing.
        let l = Layout { edge: Edge::Left, center: 0.4 };
        let (px, py) = g.phone_at(l);
        let back = g.snap(px, py);
        assert_eq!(back.edge, Edge::Left);
        assert!((back.center - 0.4).abs() < 1e-9);
    }
}
