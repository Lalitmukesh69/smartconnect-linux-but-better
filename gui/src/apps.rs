//! Apps page: stream single phone apps into their own windows through the `smartconnect-mirror
//! --host` process.
//!
//! The app list comes from `smartconnect-mirror --list-apps --json` (asks the phone over svc 17;
//! the last list is cached in `~/.cache/smartconnect/apps.json` and shown immediately). Launch,
//! session list and close talk JSON lines to the host's control socket
//! (`$XDG_RUNTIME_DIR/smartconnect-mirror.sock`) directly; the host is started detached when it
//! is not running yet.

use std::cell::{Cell, RefCell};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::time::{Duration, Instant};

use adw::prelude::*;
use gtk::{gio, glib};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::components::*;
use crate::icons::{self, IconSlot};
use crate::mirror::binary;
use crate::prefs;
use crate::theme::{IconKind, ModernIcon};

const MIRROR_BIN: &str = "smartconnect-mirror";
/// Session list refresh period.
const POLL: Duration = Duration::from_secs(3);
/// How long a freshly spawned host gets to open its control socket.
const HOST_START: Duration = Duration::from_secs(8);
/// How many recently opened apps to remember for the Recent row.
const RECENT_MAX: usize = 8;

// ── Backend (blocking; run through gio::spawn_blocking) ─────────────────────

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct App {
    pub pkg: String,
    pub name: String,
    pub cmp: Option<String>,
    /// Android user (0 = owner; a work profile is usually 10). Null/missing on older lists.
    pub user_id: Option<i64>,
    /// The phone's icon file name (`<pkg>_<cmp>.png`, see `icons.rs`). Missing on older lists.
    pub icon_name: Option<String>,
    pub work: bool,
}

impl App {
    /// What the icon resolver needs to know about this app.
    pub fn query(&self) -> icons::Query {
        icons::Query {
            pkg: self.pkg.clone(),
            name: self.name.clone(),
            cmp: self.cmp.clone(),
            user_id: self.user_id.unwrap_or(0),
            icon_name: self.icon_name.clone(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct Session {
    pub session_id: i64,
    pub pkg: String,
    pub display_id: Option<i64>,
    pub title: Option<String>,
}

fn runtime_dir() -> PathBuf {
    std::env::var_os("XDG_RUNTIME_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir)
}

fn host_socket() -> PathBuf {
    runtime_dir().join("smartconnect-mirror.sock")
}

fn cache_dir() -> PathBuf {
    std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".cache"))
        .join("smartconnect")
}

/// One request/reply on the host's control socket; `None` when no host answers.
fn host_call(req: &Value) -> Option<Value> {
    let mut s = UnixStream::connect(host_socket()).ok()?;
    s.set_read_timeout(Some(Duration::from_secs(8))).ok()?;
    s.write_all(format!("{req}\n").as_bytes()).ok()?;
    let mut line = String::new();
    BufReader::new(s).read_line(&mut line).ok()?;
    serde_json::from_str(&line).ok()
}

fn host_running() -> bool {
    UnixStream::connect(host_socket()).is_ok()
}

/// Starts `smartconnect-mirror --host` detached (own process group, log in the cache dir) unless
/// one already serves the control socket, and waits for the socket to come up.
fn ensure_host() -> Result<(), String> {
    if host_running() {
        return Ok(());
    }
    let bin = binary(MIRROR_BIN);
    let log = std::fs::create_dir_all(cache_dir())
        .and_then(|()| std::fs::OpenOptions::new().create(true).append(true).open(cache_dir().join("mirror-host.log")));
    let (out, err) = match log.and_then(|f| Ok((f.try_clone()?, f))) {
        Ok((a, b)) => (Stdio::from(a), Stdio::from(b)),
        Err(_) => (Stdio::null(), Stdio::null()),
    };
    let mut child = Command::new(&bin)
        .arg("--host")
        .stdin(Stdio::null())
        .stdout(out)
        .stderr(err)
        .process_group(0) // survives the GUI and its Ctrl+C
        .spawn()
        .map_err(|e| format!("Could not start {}: {e}", bin.display()))?;
    let deadline = Instant::now() + HOST_START;
    while Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(200));
        if host_running() {
            // Reap it whenever it exits so it never lingers as a zombie of the GUI.
            std::thread::spawn(move || child.wait());
            return Ok(());
        }
        if let Ok(Some(status)) = child.try_wait() {
            // Lost a race with another starter (daemon auto-start) → that host is fine too.
            if host_running() {
                return Ok(());
            }
            return Err(format!("The stream host exited ({status}); see {}", cache_dir().join("mirror-host.log").display()));
        }
    }
    std::thread::spawn(move || child.wait());
    Err("The stream host did not start in time".into())
}

fn reply_result(reply: Option<Value>) -> Result<String, String> {
    let r = reply.ok_or("The stream host did not answer")?;
    let msg = r.get("message").and_then(Value::as_str).unwrap_or_default().to_owned();
    if r.get("ok").and_then(Value::as_bool) == Some(true) {
        Ok(msg)
    } else {
        Err(if msg.is_empty() { "The stream host refused the request".into() } else { msg })
    }
}

fn launch(pkg: &str, cmp: Option<&str>) -> Result<String, String> {
    ensure_host()?;
    let mut req = json!({ "cmd": "app", "pkg": pkg });
    if let Some(cmp) = cmp {
        req["cmp"] = json!(cmp);
    }
    reply_result(host_call(&req))
}

/// Whole-screen mirror through the host (which owns the mirror session while it runs).
pub fn start_mirror() -> Result<String, String> {
    ensure_host()?;
    reply_result(host_call(&json!({ "cmd": "mirror" })))
}

pub fn close(sid: i64) -> Result<String, String> {
    reply_result(host_call(&json!({ "cmd": "close", "sessionId": sid })))
}

/// Open sessions; empty when no host runs.
fn sessions() -> Vec<Session> {
    host_call(&json!({ "cmd": "sessions" }))
        .and_then(|r| r.get("sessions").cloned())
        .and_then(|s| serde_json::from_value(s).ok())
        .unwrap_or_default()
}

fn parse_apps(text: &str) -> Option<Vec<App>> {
    let mut apps: Vec<App> = serde_json::from_str(text.trim()).ok()?;
    apps.retain(|a| !a.pkg.is_empty());
    for a in &mut apps {
        if a.name.is_empty() {
            a.name = a.pkg.clone();
        }
        a.cmp = a.cmp.take().filter(|c| !c.is_empty());
    }
    apps.sort_by_cached_key(|a| (a.name.to_lowercase(), a.work));
    Some(apps)
}

fn cached_apps() -> Vec<App> {
    std::fs::read_to_string(cache_dir().join("apps.json")).ok().and_then(|t| parse_apps(&t)).unwrap_or_default()
}

/// Asks the phone for its app list (up to ~15 s).
fn fetch_apps() -> Result<Vec<App>, String> {
    let bin = binary(MIRROR_BIN);
    let out = Command::new(&bin)
        .args(["--list-apps", "--json"])
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("Could not run {}: {e}", bin.display()))?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        let line = err.lines().rev().find(|l| !l.trim().is_empty()).unwrap_or("could not get the app list");
        return Err(line.trim_start_matches("Error: ").to_owned());
    }
    // stdout is the JSON array on its last non-empty line.
    let text = String::from_utf8_lossy(&out.stdout);
    text.lines()
        .rev()
        .find(|l| l.trim_start().starts_with('['))
        .and_then(parse_apps)
        .ok_or_else(|| "Unexpected --list-apps output".into())
}

/// How long to wait after a change in the icon cache dir before re-resolving (icons arrive in bursts).
const ICON_RESCAN_DELAY: Duration = Duration::from_millis(700);

// ── Page ────────────────────────────────────────────────────────────────────

pub struct AppsPage {
    toasts: adw::ToastOverlay,
    stack: gtk::Stack,
    _empty: gtk::Box,
    notice: gtk::Box,
    search: gtk::SearchEntry,
    refresh: gtk::Button,
    spinner: adw::Spinner,
    count_lbl: gtk::Label,
    _mirror_btn: gtk::Button,
    empty_search: gtk::Box,
    running: adw::PreferencesGroup,
    recent_group: gtk::Box,
    recent_flow: gtk::Box,
    flow: gtk::FlowBox,
    state: RefCell<AppsState>,
    polling: Cell<bool>,
    on_sessions: RefCell<Option<Box<dyn Fn()>>>,
    on_apps: RefCell<Option<Box<dyn Fn()>>>,
    /// Watches the icon cache dir so icons written by the stream host / icon download show up.
    _icon_monitor: Option<gio::FileMonitor>,
    /// A debounced icon re-scan is scheduled.
    icon_rescan: Cell<bool>,
}

/// One "All apps" grid tile.
struct Tile {
    /// The FlowBox wrapper (what is hidden when filtering / removed on rebuild).
    child: gtk::FlowBoxChild,
    /// Lowercase `name pkg` search haystack.
    hay: String,
    icon: IconSlot,
    query: icons::Query,
}

#[derive(Default)]
struct AppsState {
    apps: Vec<App>,
    rows: Vec<Tile>,
    /// Packages opened most recently, newest first.
    recent: Vec<String>,
    sessions: Vec<Session>,
    session_rows: Vec<adw::ActionRow>,
    fetched: bool,
    fetching: bool,
    connected: bool,
}

impl AppsPage {
    pub fn build(toasts: &adw::ToastOverlay) -> (Rc<Self>, gtk::Widget) {
        let count_lbl = gtk::Label::builder()
            .label("0 apps")
            .css_classes(["sc-stat-caption", "tabular-nums"])
            .valign(gtk::Align::Center)
            .build();

        let mirror_btn = ghost_button("Mirror phone display");
        mirror_btn.set_tooltip_text(Some("Stream entire phone display in a window"));

        let search = gtk::SearchEntry::builder()
            .placeholder_text("Search apps…")
            .width_request(220)
            .build();

        let refresh_ic = ModernIcon::new(IconKind::Refresh, 16);
        let refresh = gtk::Button::builder()
            .child(refresh_ic.widget())
            .css_classes(["sc-btn-icon"])
            .tooltip_text("Reload apps from phone")
            .valign(gtk::Align::Center)
            .build();

        let spinner = adw::Spinner::builder().visible(false).build();

        let header_actions = gtk::Box::builder()
            .spacing(crate::tokens::SPACE_2)
            .valign(gtk::Align::Center)
            .build();
        header_actions.append(&search);
        header_actions.append(&count_lbl);
        header_actions.append(&spinner);
        header_actions.append(&refresh);
        header_actions.append(&mirror_btn);

        let (header, _, _) = page_header("Apps", "Launchpad and mirroring", Some(&header_actions));

        let notice = notice_banner(
            IconKind::Smartphone,
            "App streaming is available when connected. Connect from the sidebar to launch apps.",
        );

        let running = adw::PreferencesGroup::builder()
            .title("Running apps")
            .description("Phone apps open in their own windows")
            .css_classes(["sc-pref-group"])
            .visible(false)
            .build();

        // Recent apps inside ONE Surface
        let recent_flow = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["sc-surface"])
            .build();
        let recent_group = section("Recent", Some("Recently streamed applications"), &recent_flow);
        recent_group.set_visible(false);

        // Empty search state
        let empty_search = empty_state(
            IconKind::Search,
            "No apps matching your query · Check spelling or search by package name",
        );
        empty_search.set_visible(false);

        // All apps inside ONE Surface with hairline-separated rows
        let flow = gtk::FlowBox::builder()
            .selection_mode(gtk::SelectionMode::None)
            .homogeneous(true)
            .min_children_per_line(1)
            .max_children_per_line(1)
            .row_spacing(0)
            .column_spacing(0)
            .css_classes(["sc-surface"])
            .hexpand(true)
            .build();

        let all_col = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(crate::tokens::SPACE_2)
            .build();
        all_col.append(&empty_search);
        all_col.append(&flow);

        let all_group = section("All apps", None, &all_col);

        let column = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(crate::tokens::SPACE_10)
            .margin_top(crate::tokens::CONTENT_PAD_TOP)
            .margin_bottom(crate::tokens::CONTENT_PAD_BOTTOM)
            .margin_start(crate::tokens::CONTENT_PAD_HORIZ)
            .margin_end(crate::tokens::CONTENT_PAD_HORIZ)
            .build();
        column.append(&header);
        column.append(&notice);
        column.append(&running);
        column.append(&recent_group);
        column.append(&all_group);

        let scroller = page_scroller(&column, crate::tokens::CONTENT_MAX_WIDTH);

        let load = primary_button("Load apps");
        let empty_box = empty_state(
            IconKind::AppsGrid,
            "No apps loaded yet · Connect your phone to stream apps onto this PC",
        );
        let empty = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(crate::tokens::SPACE_4)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .vexpand(true)
            .build();
        empty.append(&empty_box);
        empty.append(&load);

        let stack = gtk::Stack::new();
        stack.add_named(&empty, Some("empty"));
        stack.add_named(&scroller, Some("list"));

        let icon_dir = icons::cache_dir();
        let _ = std::fs::create_dir_all(&icon_dir);
        let icon_monitor = gio::File::for_path(&icon_dir).monitor_directory(gio::FileMonitorFlags::NONE, gio::Cancellable::NONE).ok();

        let page = Rc::new(Self {
            toasts: toasts.clone(),
            stack: stack.clone(),
            _empty: empty,
            notice,
            search,
            refresh,
            spinner,
            count_lbl,
            _mirror_btn: mirror_btn.clone(),
            empty_search,
            running,
            recent_group,
            recent_flow,
            flow,
            state: RefCell::new(AppsState { recent: prefs::load().apps.recent, ..AppsState::default() }),
            polling: Cell::new(false),
            on_sessions: RefCell::default(),
            on_apps: RefCell::default(),
            _icon_monitor: icon_monitor,
            icon_rescan: Cell::new(false),
        });

        let weak_mirror = Rc::downgrade(&page);
        mirror_btn.connect_clicked(move |_| {
            if let Some(p) = weak_mirror.upgrade() {
                p.toggle_mirror();
            }
        });

        if let Some(monitor) = &page._icon_monitor {
            let weak = Rc::downgrade(&page);
            monitor.connect_changed(move |_, _, _, _| {
                if let Some(p) = weak.upgrade() {
                    p.schedule_icon_rescan();
                }
            });
        }

        let weak = Rc::downgrade(&page);
        page.search.connect_search_changed(move |_| {
            if let Some(p) = weak.upgrade() {
                p.apply_filter();
            }
        });
        for button in [&page.refresh, &load] {
            let weak = Rc::downgrade(&page);
            button.connect_clicked(move |_| {
                if let Some(p) = weak.upgrade() {
                    p.fetch();
                }
            });
        }

        page.set_apps(cached_apps());
        page.start_polling();
        (page, stack.upcast())
    }

    /// Called whenever the session list changes (the Phone page's mirror button follows it).
    pub fn connect_sessions_changed(&self, f: impl Fn() + 'static) {
        *self.on_sessions.borrow_mut() = Some(Box::new(f));
    }

    /// Called whenever the app list or most-recent list changes (sidebar count badge and Phone summary).
    pub fn connect_apps_changed(&self, f: impl Fn() + 'static) {
        *self.on_apps.borrow_mut() = Some(Box::new(f));
    }

    pub fn app_count(&self) -> usize {
        self.state.borrow().apps.len()
    }

    pub fn all_apps(&self) -> Vec<App> {
        self.state.borrow().apps.clone()
    }

    /// Most recently streamed phone app present in the current list.
    pub fn recent_app(&self) -> Option<App> {
        let st = self.state.borrow();
        st.recent
            .iter()
            .find_map(|pkg| st.apps.iter().find(|a| &a.pkg == pkg).cloned())
    }

    pub fn launch_by_pkg(self: &Rc<Self>, pkg: &str) {
        let app = self.state.borrow().apps.iter().find(|a| a.pkg == pkg).cloned();
        if let Some(app) = app {
            if !self.state.borrow().connected {
                self.toast("The phone is not connected");
                return;
            }
            self.remember_recent(&app.pkg);
            self.toast(&format!("Opening {}…", app.name));
            let (pkg, cmp, name) = (app.pkg.clone(), app.cmp.clone(), app.name.clone());
            let weak = Rc::downgrade(self);
            glib::spawn_future_local(async move {
                let result = gio::spawn_blocking(move || launch(&pkg, cmp.as_deref()))
                    .await
                    .unwrap_or_else(|_| Err("launch thread failed".into()));
                let Some(p) = weak.upgrade() else { return };
                if let Err(e) = result {
                    p.toast(&format!("Could not open {name}: {e}"));
                }
                p.poll_soon();
            });
        }
    }

    /// A phone app's name from the (cached) app list.
    pub fn app_name(&self, pkg: &str) -> Option<String> {
        self.state.borrow().apps.iter().find(|a| a.pkg == pkg).map(|a| a.name.clone())
    }

    /// The icon query for a package other pages know only by package and name (notifications,
    /// sessions): enriched with `cmp`/`userId`/`iconName` from the app list when the app is in it
    /// (the owner's entry first).
    pub fn icon_query(&self, pkg: &str, name: &str) -> icons::Query {
        let st = self.state.borrow();
        let known = st.apps.iter().filter(|a| a.pkg == pkg).min_by_key(|a| (a.work, a.user_id.unwrap_or(0)));
        match known {
            Some(app) => icons::Query { name: if name.is_empty() { app.name.clone() } else { name.to_owned() }, ..app.query() },
            None => icons::Query::new(pkg, name),
        }
    }

    /// Re-resolves every icon after the cache dir changed, debounced (icons arrive in bursts).
    fn schedule_icon_rescan(self: &Rc<Self>) {
        if self.icon_rescan.replace(true) {
            return;
        }
        let weak = Rc::downgrade(self);
        glib::timeout_add_local_once(ICON_RESCAN_DELAY, move || {
            if let Some(p) = weak.upgrade() {
                p.icon_rescan.set(false);
                p.refresh_icons();
            }
        });
    }

    /// Resolves every tile's and chip's icon again (new files, a changed theme or new desktop apps).
    fn refresh_icons(self: &Rc<Self>) {
        icons::rescan();
        for tile in &self.state.borrow().rows {
            tile.icon.set(&tile.query);
        }
        self.rebuild_recent();
        if let Some(f) = self.on_apps.borrow().as_ref() {
            f();
        }
    }

    /// The host's whole-screen mirror session, if one is open.
    pub fn toggle_mirror(self: &Rc<Self>) {
        if let Some(sid) = self.mirror_session() {
            self.close_session(sid);
        } else {
            let weak = Rc::downgrade(self);
            glib::spawn_future_local(async move {
                let result = gio::spawn_blocking(start_mirror)
                    .await
                    .unwrap_or_else(|_| Err("mirror thread failed".into()));
                let Some(p) = weak.upgrade() else { return };
                if let Err(e) = result {
                    p.toast(&e);
                }
                p.poll_soon();
            });
        }
    }

    pub fn mirror_session(&self) -> Option<i64> {
        self.state.borrow().sessions.iter().find(|s| s.pkg == "mirror").map(|s| s.session_id)
    }

    pub fn set_connected(self: &Rc<Self>, connected: bool) {
        let mut st = self.state.borrow_mut();
        st.connected = connected;
        drop(st);
        self.update_controls();
    }

    /// Focuses the app search entry (e.g. from Ctrl+F).
    pub fn focus_search(&self) {
        self.search.grab_focus();
    }

    /// The page was shown: load the phone's list once per run.
    pub fn shown(self: &Rc<Self>) {
        let st = self.state.borrow();
        let want = st.connected && !st.fetched && !st.fetching;
        drop(st);
        if want {
            self.fetch();
        }
        self.poll_now();
    }

    fn toast(&self, message: &str) {
        self.toasts.add_toast(adw::Toast::new(message));
    }

    fn update_controls(&self) {
        let st = self.state.borrow();
        self.refresh.set_sensitive(st.connected && !st.fetching);
        self.spinner.set_visible(st.fetching);
        self.stack.set_visible_child_name(if st.apps.is_empty() { "empty" } else { "list" });
        let count = st.apps.len();
        self.count_lbl.set_label(&format!("{count} apps"));
        self.notice.set_visible(!st.connected);
    }

    fn fetch(self: &Rc<Self>) {
        {
            let mut st = self.state.borrow_mut();
            if st.fetching {
                return;
            }
            st.fetching = true;
        }
        self.update_controls();
        let weak = Rc::downgrade(self);
        glib::spawn_future_local(async move {
            let result = gio::spawn_blocking(fetch_apps).await.unwrap_or_else(|_| Err("app list thread failed".into()));
            let Some(p) = weak.upgrade() else { return };
            {
                let mut st = p.state.borrow_mut();
                st.fetching = false;
                st.fetched = true;
            }
            match result {
                Ok(apps) => {
                    let unchanged = p.state.borrow().apps == apps;
                    p.set_apps(apps);
                    // Same list: the tiles stay, but icons may have arrived since — look again.
                    if unchanged {
                        p.refresh_icons();
                    }
                }
                Err(e) => p.toast(&e),
            }
            p.update_controls();
        });
    }

    fn set_apps(self: &Rc<Self>, apps: Vec<App>) {
        let mut st = self.state.borrow_mut();
        if st.apps == apps && !st.rows.is_empty() {
            return;
        }
        for tile in st.rows.drain(..) {
            self.flow.remove(&tile.child);
        }
        icons::rescan();
        for app in &apps {
            let (button, icon) = self.app_row(app);
            self.flow.append(&button);
            // FlowBox wraps each child in a FlowBoxChild; that is what we hide when filtering.
            let child = button.parent().and_downcast::<gtk::FlowBoxChild>().expect("flowbox child");
            st.rows.push(Tile { child, hay: format!("{} {}", app.name, app.pkg).to_lowercase(), icon, query: app.query() });
        }
        st.apps = apps;
        drop(st);
        self.apply_filter();
        self.rebuild_recent();
        self.update_controls();
        if let Some(f) = self.on_apps.borrow().as_ref() {
            f();
        }
    }

    fn app_row(self: &Rc<Self>, app: &App) -> (gtk::Widget, IconSlot) {
        let icon = IconSlot::new(40);
        icon.set(&app.query());

        let title_lbl = gtk::Label::builder()
            .label(&app.name)
            .xalign(0.0)
            .css_classes(["sc-row-title"])
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .build();

        let sub_lbl = gtk::Label::builder()
            .label(if app.work { format!("{} · Work profile", app.pkg) } else { app.pkg.clone() })
            .xalign(0.0)
            .css_classes(["sc-row-helper"])
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .build();

        let text_col = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(2)
            .hexpand(true)
            .valign(gtk::Align::Center)
            .build();
        text_col.append(&title_lbl);
        text_col.append(&sub_lbl);

        let launch_btn = row_action_button("Launch", false);

        let row = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(crate::tokens::SPACE_3)
            .css_classes(["sc-row", "sc-row-interactive"])
            .build();
        row.append(icon.widget());
        row.append(&text_col);
        row.append(&launch_btn);

        let weak = Rc::downgrade(self);
        let app_clone = app.clone();
        let btn = launch_btn.downgrade();
        launch_btn.connect_clicked(move |_| {
            if let (Some(p), Some(b)) = (weak.upgrade(), btn.upgrade()) {
                p.launch(&app_clone, &b);
            }
        });

        (row.upcast(), icon)
    }

    fn recent_row(self: &Rc<Self>, app: &App) -> gtk::Widget {
        let icon = IconSlot::new(40);
        icon.set(&app.query());

        let title_lbl = gtk::Label::builder()
            .label(&app.name)
            .xalign(0.0)
            .css_classes(["sc-row-title"])
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .build();

        let sub_lbl = gtk::Label::builder()
            .label("Recently streamed")
            .xalign(0.0)
            .css_classes(["sc-row-helper"])
            .build();

        let text_col = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(2)
            .hexpand(true)
            .valign(gtk::Align::Center)
            .build();
        text_col.append(&title_lbl);
        text_col.append(&sub_lbl);

        let launch_btn = row_action_button("Launch", false);

        let row = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(crate::tokens::SPACE_3)
            .css_classes(["sc-row", "sc-row-interactive"])
            .build();
        row.append(icon.widget());
        row.append(&text_col);
        row.append(&launch_btn);

        let weak = Rc::downgrade(self);
        let app_clone = app.clone();
        let btn = launch_btn.downgrade();
        launch_btn.connect_clicked(move |_| {
            if let (Some(p), Some(b)) = (weak.upgrade(), btn.upgrade()) {
                p.launch(&app_clone, &b);
            }
        });

        row.upcast()
    }

    /// Rebuilds the Recent chips from the remembered packages that are in the current app list.
    fn rebuild_recent(self: &Rc<Self>) {
        while let Some(child) = self.recent_flow.first_child() {
            self.recent_flow.remove(&child);
        }
        let apps: Vec<App> = {
            let st = self.state.borrow();
            st.recent.iter().filter_map(|pkg| st.apps.iter().find(|a| &a.pkg == pkg).cloned()).collect()
        };
        for (i, app) in apps.iter().enumerate() {
            if i > 0 {
                self.recent_flow.append(&hairline());
            }
            let row = self.recent_row(app);
            self.recent_flow.append(&row);
        }
        self.recent_group.set_visible(!apps.is_empty());
    }

    /// Moves `pkg` to the front of the remembered recents and persists them.
    fn remember_recent(self: &Rc<Self>, pkg: &str) {
        {
            let mut st = self.state.borrow_mut();
            st.recent.retain(|p| p != pkg);
            st.recent.insert(0, pkg.to_owned());
            st.recent.truncate(RECENT_MAX);
        }
        let recent = self.state.borrow().recent.clone();
        let mut prefs = prefs::load();
        if prefs.apps.recent != recent {
            prefs.apps.recent = recent;
            if let Err(e) = prefs::save(&prefs) {
                eprintln!("smartconnect-gui: could not save the recent apps: {e}");
            }
        }
        self.rebuild_recent();
        if let Some(f) = self.on_apps.borrow().as_ref() {
            f();
        }
    }

    fn apply_filter(&self) {
        let query = self.search.text().trim().to_lowercase();
        let mut matches = 0;
        for tile in &self.state.borrow().rows {
            let m = query.split_whitespace().all(|w| tile.hay.contains(w));
            tile.child.set_visible(m);
            if m {
                matches += 1;
            }
        }
        self.empty_search.set_visible(!query.is_empty() && matches == 0);
        self.flow.set_visible(query.is_empty() || matches > 0);
    }

    fn launch(self: &Rc<Self>, app: &App, button: &gtk::Button) {
        if !self.state.borrow().connected {
            self.toast("The phone is not connected");
            return;
        }
        self.remember_recent(&app.pkg);
        button.set_sensitive(false);
        self.toast(&format!("Opening {}…", app.name));
        let (pkg, cmp, name) = (app.pkg.clone(), app.cmp.clone(), app.name.clone());
        let weak = Rc::downgrade(self);
        let button = button.downgrade();
        glib::spawn_future_local(async move {
            let result = gio::spawn_blocking(move || launch(&pkg, cmp.as_deref()))
                .await
                .unwrap_or_else(|_| Err("launch thread failed".into()));
            if let Some(b) = button.upgrade() {
                b.set_sensitive(true);
            }
            let Some(p) = weak.upgrade() else { return };
            if let Err(e) = result {
                p.toast(&format!("Could not open {name}: {e}"));
            }
            p.poll_soon();
        });
    }

    fn close_session(self: &Rc<Self>, sid: i64) {
        let weak = Rc::downgrade(self);
        glib::spawn_future_local(async move {
            let result = gio::spawn_blocking(move || close(sid)).await.unwrap_or_else(|_| Err("close thread failed".into()));
            let Some(p) = weak.upgrade() else { return };
            if let Err(e) = result {
                p.toast(&e);
            }
            p.poll_now();
        });
    }

    // ── Running sessions ────────────────────────────────────────────────────

    fn start_polling(self: &Rc<Self>) {
        self.poll_now();
        let weak = Rc::downgrade(self);
        glib::timeout_add_local(POLL, move || {
            let Some(p) = weak.upgrade() else { return glib::ControlFlow::Break };
            p.poll_now();
            glib::ControlFlow::Continue
        });
    }

    /// Re-polls a couple of times after an action (a stream takes a few seconds to appear).
    pub fn poll_soon(self: &Rc<Self>) {
        for delay in [700, 2500] {
            let weak = Rc::downgrade(self);
            glib::timeout_add_local_once(Duration::from_millis(delay), move || {
                if let Some(p) = weak.upgrade() {
                    p.poll_now();
                }
            });
        }
    }

    fn poll_now(self: &Rc<Self>) {
        if self.polling.replace(true) {
            return;
        }
        let weak = Rc::downgrade(self);
        glib::spawn_future_local(async move {
            let list = gio::spawn_blocking(sessions).await.unwrap_or_default();
            let Some(p) = weak.upgrade() else { return };
            p.polling.set(false);
            p.set_sessions(list);
        });
    }

    fn set_sessions(self: &Rc<Self>, mut list: Vec<Session>) {
        list.sort_by_key(|s| s.session_id);
        let mut st = self.state.borrow_mut();
        if st.sessions == list {
            return;
        }
        for row in st.session_rows.drain(..) {
            self.running.remove(&row);
        }
        for s in &list {
            let name = st.apps.iter().find(|a| a.pkg == s.pkg).map(|a| a.name.clone());
            let title = match (s.pkg.as_str(), s.title.as_deref().filter(|t| !t.is_empty())) {
                ("mirror", _) => "Phone screen".to_owned(),
                (_, Some(t)) => t.trim_end_matches(" — Phone").to_owned(),
                _ => name.unwrap_or_else(|| s.pkg.clone()),
            };
            let row = adw::ActionRow::builder()
                .title(&title)
                .subtitle(if s.pkg == "mirror" { "Screen mirror".to_owned() } else { s.pkg.clone() })
                .use_markup(false)
                .build();
            if s.pkg == "mirror" {
                row.add_prefix(&gtk::Image::builder().icon_name("phone-symbolic").pixel_size(32).build());
            } else {
                // `st` is borrowed here, so build the query from the list directly (not icon_query).
                let query = match st.apps.iter().find(|a| a.pkg == s.pkg) {
                    Some(app) => icons::Query { name: title.clone(), ..app.query() },
                    None => icons::Query::new(&s.pkg, &title),
                };
                let icon = IconSlot::new(32);
                icon.set(&query);
                row.add_prefix(icon.widget());
            }
            let close_btn = gtk::Button::builder()
                .icon_name("window-close-symbolic")
                .tooltip_text("Close this window")
                .valign(gtk::Align::Center)
                .css_classes(["flat", "circular"])
                .build();
            let weak = Rc::downgrade(self);
            let sid = s.session_id;
            close_btn.connect_clicked(move |b| {
                b.set_sensitive(false);
                if let Some(p) = weak.upgrade() {
                    p.close_session(sid);
                }
            });
            row.add_suffix(&close_btn);
            self.running.add(&row);
            st.session_rows.push(row);
        }
        self.running.set_visible(!list.is_empty());
        st.sessions = list;
        drop(st);
        if let Some(f) = self.on_sessions.borrow().as_ref() {
            f();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_list_keeps_icon_fields() {
        // `smartconnect-mirror --list-apps --json` output (and the apps.json cache it writes).
        let text = r#"[
            {"pkg":"com.example.notes","name":"Notes","cmp":"com.example.notes.Main","userId":0,
             "iconName":"com.example.notes_com.example.notes.Main.png","work":false},
            {"pkg":"com.example.notes","name":"Notes","cmp":"com.example.notes.Main","userId":10,"work":true},
            {"pkg":"com.example.old","name":"Old","cmp":null,"userId":null}
        ]"#;
        let apps = parse_apps(text).unwrap();
        let q: Vec<Option<String>> = apps.iter().map(|a| a.query().icon_file()).collect();
        assert_eq!(
            q,
            vec![
                Some("com.example.notes_com.example.notes.Main.png".to_owned()),
                Some("10_com.example.notes_com.example.notes.Main.png".to_owned()),
                None,
            ]
        );
        // An older cache without the new fields still parses.
        let old = parse_apps(r#"[{"pkg":"com.example.a","name":"A","cmp":"com.example.a.Main"}]"#).unwrap();
        assert_eq!(old[0].query().icon_file().as_deref(), Some("com.example.a_com.example.a.Main.png"));
    }
}
