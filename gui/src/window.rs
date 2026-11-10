//! The main window: an Apple Pro Dark Monochrome 272px sidebar split-view with sticky 56px
//! workspace header, Phone / Apps / Display / Files / Notifications views, `Ctrl+K` Command Palette,
//! and Endpoint Telemetry sheet, driven by `smartconnectd` status events.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::rc::{Rc, Weak};
use std::time::Duration;

use adw::prelude::*;
use gtk::{gdk, gio, glib};

use crate::apps::{self, AppsPage};
use crate::crosscontrol::CrossControlPage;
use crate::daemon::{self, Event, Notification, Reply, Status};
use crate::display::DisplayPage;
use crate::files::FilesPage;
use crate::icons::{self, IconSlot, Query};
use crate::lastdevice::{self, LastDevice};
use crate::lifecycle::{self, Daemon};
use crate::mirror::{Exit, Mirror};
use crate::palette::{self, CommandEntry};
use crate::prefs;
use crate::theme::{brand_mark, format_display_path, overline, set_dot_online, status_dot};
use crate::tray;

const TABS: [&str; 5] = ["phone", "apps", "display", "files", "notifications"];

const SIGNAL_WORDS: [&str; 6] = ["None", "Very weak", "Weak", "Fair", "Good", "Excellent"];
/// Adwaita's `network-{wireless,cellular}-signal-<x>-symbolic` names for levels 0–5.
const SIGNAL_ICONS: [&str; 6] = ["none", "weak", "weak", "ok", "good", "excellent"];

fn signal_level(level: Option<i64>) -> Option<usize> {
    level.map(|l| l.clamp(0, 5) as usize)
}

fn signal_text(level: Option<i64>) -> &'static str {
    signal_level(level).map_or("—", |l| SIGNAL_WORDS[l])
}

fn wifi_summary_word(v: &LastDevice) -> &'static str {
    if !v.wifi {
        return "Off";
    }
    match signal_level(v.wifi_signal) {
        Some(5) => "Optimal",
        Some(4) => "Good",
        Some(3) => "Fair",
        Some(1 | 2) => "Weak",
        Some(0) => "None",
        Some(_) => "On",
        None => "On",
    }
}

fn cellular_bars(level: Option<i64>) -> usize {
    match signal_level(level) {
        None | Some(0) => 0,
        Some(1) => 1,
        Some(2 | 3) => 2,
        Some(4) => 3,
        Some(_) => 4,
    }
}

fn view_title(name: &str) -> &'static str {
    match name {
        "apps" => "Apps",
        "display" => "Display",
        "files" => "Files",
        "notifications" => "Notifications",
        _ => "Phone",
    }
}

pub struct Ui {
    window: adw::ApplicationWindow,
    toasts: adw::ToastOverlay,
    split_view: adw::OverlaySplitView,
    sidebar_toggle: gtk::Button,
    root: gtk::Stack,
    /// Label under the spinner on the "connecting" page ("Connecting…" / "Starting smartconnectd…").
    connecting_label: gtk::Label,
    /// The five-view stack (phone, apps, display, files, notifications).
    views: gtk::Stack,
    sidebar: SidebarNav,
    header: TopHeader,
    /// The "service isn't running" page (with the last device and a Start button).
    down: DownPage,
    phone: PhonePage,
    notifs: NotifPage,
    pair: PairDialog,
    apps: Rc<AppsPage>,
    display: Rc<DisplayPage>,
    cross: Rc<CrossControlPage>,
    files: Rc<FilesPage>,
    /// The system-tray item, when the desktop has a StatusNotifierHost.
    tray: RefCell<Option<tray::Tray>>,
    /// Tracks how the daemon is running, so Quit stops the right thing.
    daemon_ctl: RefCell<Daemon>,
    state: RefCell<State>,
}

#[derive(Default)]
struct State {
    status: Option<Status>,
    wallpaper: Option<String>,
    busy: bool,
    /// The daemon was just launched; keep the "Starting…" spinner until the socket answers.
    starting: bool,
    mirror: Option<Mirror>,
    /// A whole-screen mirror request to the stream host is in flight.
    mirror_pending: bool,
    show_qr: bool,
    qr_path: Option<String>,
    rows: HashMap<String, NotifRow>,
    /// Notifications cleared from this PC's list (Dismiss / Clear all).
    dismissed: HashSet<String>,
    logged_first: bool,
    /// Packages whose desktop popups the daemon suppresses (`notif_mute`); `None` until asked.
    muted: Option<Vec<String>>,
    muted_loading: bool,
    muted_rows: Vec<adw::ActionRow>,
    /// App names seen in notifications, by package (for the muted list).
    app_names: HashMap<String, String>,
    /// The last connected phone (persisted; shown while it is away or the daemon is down).
    last: LastDevice,
    /// The daemon's wallpaper value (path + `?md5`) last copied into the cache.
    copied_wallpaper: Option<String>,
    /// Connected at the previous status, to stamp and save the cache on a disconnect.
    was_connected: bool,
    /// Unix seconds of the last cache save (re-saved at most every few minutes while connected).
    last_saved: u64,
    /// Wallpaper shown on the "service isn't running" page.
    down_wallpaper: Option<String>,
    /// "Start service" was pressed and `Daemon::ensure_running` is in flight.
    service_starting: bool,
}

/// What the Phone page shows: the live phone, the last one while it is away, or the pairing prompt.
#[derive(Clone, Copy, PartialEq, Eq)]
enum PhoneState {
    Connected,
    Disconnected,
    Unpaired,
}

/// Device switcher card in the sidebar and its popover (Paired Endpoint, Rename, Telemetry, Pair, Preferences, Connect).
struct DeviceMenu {
    button: gtk::MenuButton,
    popover: gtk::Popover,
    dot: gtk::Box,
    label: gtk::Label,
    sub_label: gtk::Label,
    pop_active: gtk::Button,
    pop_active_name: gtk::Label,
    pop_active_check: gtk::Image,
    pop_sub: gtk::Label,
    rename: gtk::Button,
    telemetry: gtk::Button,
    pair: gtk::Button,
    prefs: gtk::Button,
    /// Connect / Disconnect, or "Start service" while the daemon is down.
    connect: gtk::Button,
}

/// Fixed 272px Apple Pro Monochrome left sidebar.
struct SidebarNav {
    root: gtk::Box,
    cmd_trigger: gtk::Button,
    device: DeviceMenu,
    nav_buttons: Vec<(&'static str, gtk::Button)>,
    apps_badge: gtk::Label,
    notif_badge: gtk::Label,
    foot_bat_icon: gtk::Image,
    foot_bat_val: gtk::Label,
    foot_wifi_icon: gtk::Image,
    foot_wifi_val: gtk::Label,
    foot_bt_icon: gtk::Image,
    foot_bt_val: gtk::Label,
    foot_cta: gtk::Button,
    foot_cta_spinner: adw::Spinner,
    foot_cta_dot: gtk::Box,
    foot_cta_label: gtk::Label,
}

/// Sticky 56px workspace header bar with breadcrumb, status pill, Quick Command trigger, Telemetry and CTA.
struct TopHeader {
    crumb_device: gtk::Label,
    crumb_view: gtk::Label,
    status_pill: gtk::Label,
    quick_cmd: gtk::Button,
    telemetry: gtk::Button,
    connect: gtk::Button,
    connect_spinner: adw::Spinner,
    connect_label: gtk::Label,
}

struct PhonePage {
    hero_top: gtk::Box,
    /// Either a cached phone wallpaper thumbnail OR the crisp 56×56 monochrome device tile.
    thumb_box: gtk::Widget,
    thumb_stack: gtk::Stack,
    wallpaper: gtk::Picture,
    status_dot: gtk::Box,
    status_pill: gtk::Label,
    model_pill: gtk::Label,
    name_stack: gtk::Stack,
    name: gtk::Label,
    rename_btn: gtk::Button,
    rename_entry: gtk::Entry,
    rename_save: gtk::Button,
    rename_cancel: gtk::Button,
    subtitle: gtk::Label,
    mirror: gtk::Button,
    telemetry: gtk::Button,
    connect: gtk::Button,
    connect_spinner: adw::Spinner,
    connect_label: gtk::Label,
    pair: gtk::Button,
    unread_card: gtk::Button,
    unread_label: gtk::Label,
    /// 4-column Bento telemetry wrap inside the hero card.
    stats_wrap: gtk::Box,
    bat_icon: gtk::Image,
    bat_val: gtk::Label,
    bat_segments: [gtk::Box; 5],
    bat_sub: gtk::Label,
    wifi_icon: gtk::Image,
    wifi_val: gtk::Label,
    wifi_sub: gtk::Label,
    cell_icon: gtk::Image,
    cell_val: gtk::Label,
    cell_bars: [gtk::Box; 4],
    cell_sub: gtk::Label,
    bt_icon: gtk::Image,
    bt_val: gtk::Label,
    bt_sub: gtk::Label,
    /// Cross Control quick-launch section.
    cross_section: gtk::Box,
    cross_btn: gtk::Button,
    cross_edge: gtk::Label,
    cross_pills: [gtk::Label; 3],
    /// System Continuity 3-column diagnostics row.
    continuity_section: gtk::Box,
    recent_icon_slot: IconSlot,
    recent_icon_stack: gtk::Stack,
    recent_name: gtk::Label,
    recent_sub: gtk::Label,
    recent_btn: gtk::Button,
    recent_pkg: RefCell<Option<String>>,
    mount_path: gtk::Label,
    mount_status: gtk::Label,
    mount_btn: gtk::Button,
    share_path: gtk::Label,
    share_status: gtk::Label,
    share_btn: gtk::Button,
}

/// The root "Smart Connect service isn't running" page.
struct DownPage {
    card: gtk::Box,
    wallpaper: gtk::Picture,
    wallpaper_stack: gtk::Stack,
    name: gtk::Label,
    sub: gtk::Label,
    start: gtk::Button,
}

struct NotifPage {
    empty: adw::StatusPage,
    today: gtk::Box,
    clear_all: gtk::Button,
    list: gtk::ListBox,
    muted_group: adw::PreferencesGroup,
    muted: adw::ExpanderRow,
}

struct PairDialog {
    dialog: adw::Dialog,
    qr: gtk::Picture,
    qr_box: gtk::Box,
    status: gtk::Label,
    spinner: adw::Spinner,
    button: gtk::Button,
    #[allow(dead_code)]
    firewall: std::rc::Rc<crate::setup::FirewallHint>,
}

struct NotifRow {
    row: gtk::ListBoxRow,
    icon: IconSlot,
    header: gtk::Label,
    sub: gtk::Label,
    body: gtk::Label,
    reply_box: gtk::Box,
    entry: gtk::Entry,
    send: gtk::Button,
    menu: gtk::MenuButton,
    open_item: gtk::Button,
    mute_item: gtk::Button,
    data: Notification,
}

fn page_scroller(child: &impl IsA<gtk::Widget>, max: i32) -> gtk::ScrolledWindow {
    let clamp = adw::Clamp::builder().maximum_size(max).tightening_threshold(max).child(child).build();
    gtk::ScrolledWindow::builder().hscrollbar_policy(gtk::PolicyType::Never).vexpand(true).child(&clamp).build()
}

/// Shows the image at `path` (a `?md5` suffix is ignored) or the icon placeholder.
fn show_wallpaper(picture: &gtk::Picture, stack: &gtk::Stack, path: Option<&str>) {
    match path.map(|p| p.split('?').next().unwrap_or(p)).map(gdk::Texture::from_filename) {
        Some(Ok(texture)) => {
            picture.set_paintable(Some(&texture));
            stack.set_visible_child_name("picture");
        }
        _ => {
            picture.set_paintable(None::<&gdk::Paintable>);
            stack.set_visible_child_name("placeholder");
        }
    }
}

fn wallpaper_thumb(w: i32, h: i32) -> (gtk::Widget, gtk::Picture, gtk::Stack) {
    let picture = gtk::Picture::builder()
        .content_fit(gtk::ContentFit::Cover)
        .can_shrink(true)
        .width_request(w)
        .height_request(h)
        .css_classes(["wallpaper"])
        .overflow(gtk::Overflow::Hidden)
        .build();
    let placeholder = gtk::Image::builder()
        .icon_name("phone-symbolic")
        .pixel_size(24)
        .width_request(56)
        .height_request(56)
        .css_classes(["sc-icon-box"])
        .build();
    let stack = gtk::Stack::new();
    stack.add_named(&placeholder, Some("placeholder"));
    stack.add_named(&picture, Some("picture"));
    let tall = adw::Clamp::builder()
        .orientation(gtk::Orientation::Vertical)
        .maximum_size(h)
        .tightening_threshold(h)
        .child(&stack)
        .build();
    let thumb = adw::Clamp::builder()
        .orientation(gtk::Orientation::Horizontal)
        .maximum_size(w)
        .tightening_threshold(w)
        .valign(gtk::Align::Center)
        .child(&tall)
        .build();
    (thumb.upcast(), picture, stack)
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    c.next().map_or_else(String::new, |f| f.to_uppercase().chain(c).collect())
}

fn set_dimmed(w: &impl IsA<gtk::Widget>, dimmed: bool) {
    set_class(w, "sc-dimmed", dimmed);
}

fn set_class(w: &impl IsA<gtk::Widget>, class: &str, on: bool) {
    if on {
        w.add_css_class(class);
    } else {
        w.remove_css_class(class);
    }
}

fn menu_item(label: &str) -> gtk::Button {
    let l = gtk::Label::builder().label(label).xalign(0.0).hexpand(true).build();
    gtk::Button::builder().css_classes(["flat", "sc-pop-item"]).child(&l).build()
}

fn pop_action_item(label: &str, icon: &str) -> gtk::Button {
    let l = gtk::Label::builder().label(label).xalign(0.0).hexpand(true).build();
    let img = gtk::Image::builder().icon_name(icon).pixel_size(13).css_classes(["sc-muted"]).build();
    let row = gtk::Box::builder().spacing(8).build();
    row.append(&l);
    row.append(&img);
    gtk::Button::builder().css_classes(["flat", "sc-pop-item"]).child(&row).build()
}

/// Primary overflow menu (keeps standard GNOME Preferences, Shortcuts, About and explicit Quit reachable).
fn build_app_menu() -> gio::Menu {
    let pair_sec = gio::Menu::new();
    pair_sec.append(Some("Pair new phone…"), Some("win.pair"));
    pair_sec.append(Some("Command Palette…"), Some("win.command-palette"));

    let app_sec = gio::Menu::new();
    app_sec.append(Some("Preferences"), Some("win.preferences"));
    app_sec.append(Some("Keyboard Shortcuts"), Some("win.shortcuts"));
    app_sec.append(Some("About Smart Connect"), Some("win.about"));

    let quit_sec = gio::Menu::new();
    quit_sec.append(Some("Quit"), Some("win.quit"));

    let root = gio::Menu::new();
    root.append_section(None, &pair_sec);
    root.append_section(None, &app_sec);
    root.append_section(None, &quit_sec);
    root
}

fn count_cached_icons(dir: &Path) -> usize {
    let Ok(entries) = std::fs::read_dir(dir) else { return 0 };
    entries
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("png") && e.path().is_file())
        .count()
}

fn clear_cached_icons(dir: &Path) -> std::io::Result<usize> {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(e) => return Err(e),
    };
    let mut removed = 0;
    for entry in entries.filter_map(Result::ok) {
        let p = entry.path();
        if p.extension().and_then(|s| s.to_str()) == Some("png") && p.is_file() {
            std::fs::remove_file(&p)?;
            removed += 1;
        }
    }
    Ok(removed)
}

impl DeviceMenu {
    fn build() -> Self {
        let phone_icon = gtk::Image::builder().icon_name("phone-symbolic").pixel_size(16).build();
        let icon_tile = gtk::Box::builder()
            .width_request(36)
            .height_request(36)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .css_classes(["sc-icon-box-sm"])
            .build();
        icon_tile.append(&phone_icon);

        let dot = status_dot(10, true);
        dot.set_halign(gtk::Align::End);
        dot.set_valign(gtk::Align::End);
        let icon_overlay = gtk::Overlay::builder().child(&icon_tile).build();
        icon_overlay.add_overlay(&dot);

        let label = gtk::Label::builder()
            .label("Smart Connect")
            .xalign(0.0)
            .hexpand(true)
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .css_classes(["heading"])
            .build();
        let sub_label = gtk::Label::builder()
            .label("Connecting…")
            .xalign(0.0)
            .hexpand(true)
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .css_classes(["sc-mono", "caption", "sc-secondary"])
            .build();
        let text_col = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(2)
            .hexpand(true)
            .valign(gtk::Align::Center)
            .build();
        text_col.append(&label);
        text_col.append(&sub_label);

        let arrow = gtk::Image::builder()
            .icon_name("pan-down-symbolic")
            .pixel_size(14)
            .css_classes(["sc-muted"])
            .valign(gtk::Align::Center)
            .build();
        let content = gtk::Box::builder().spacing(10).build();
        content.append(&icon_overlay);
        content.append(&text_col);
        content.append(&arrow);

        // Popover contents
        let over = overline("Paired Endpoint");
        over.set_margin_start(8);
        over.set_margin_end(8);
        over.set_margin_top(4);
        over.set_margin_bottom(2);

        let pop_phone_ic = gtk::Image::builder().icon_name("phone-symbolic").pixel_size(14).build();
        let pop_active_name = gtk::Label::builder()
            .xalign(0.0)
            .hexpand(true)
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .build();
        let pop_active_check = gtk::Image::builder().icon_name("object-select-symbolic").pixel_size(14).build();
        let active_row = gtk::Box::builder().spacing(8).build();
        active_row.append(&pop_phone_ic);
        active_row.append(&pop_active_name);
        active_row.append(&pop_active_check);
        let pop_active = gtk::Button::builder().css_classes(["sc-pop-active"]).child(&active_row).build();

        let pop_sub = gtk::Label::builder()
            .xalign(0.0)
            .margin_start(8)
            .margin_end(8)
            .margin_top(2)
            .margin_bottom(2)
            .css_classes(["caption", "sc-muted", "sc-mono"])
            .build();

        let rename = pop_action_item("Rename Device…", "document-edit-symbolic");
        let telemetry = pop_action_item("System Telemetry…", "dialog-information-symbolic");
        let pair = pop_action_item("Pair new phone…", "list-add-symbolic");
        let prefs = pop_action_item("Preferences…", "preferences-system-symbolic");
        let connect = menu_item("Disconnect");

        let pop_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(3)
            .margin_top(4)
            .margin_bottom(4)
            .margin_start(4)
            .margin_end(4)
            .build();
        pop_box.append(&over);
        pop_box.append(&pop_active);
        pop_box.append(&pop_sub);
        pop_box.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
        pop_box.append(&rename);
        pop_box.append(&telemetry);
        pop_box.append(&pair);
        pop_box.append(&prefs);
        pop_box.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
        pop_box.append(&connect);

        let popover = gtk::Popover::builder().child(&pop_box).width_request(246).build();
        let button = gtk::MenuButton::builder()
            .child(&content)
            .popover(&popover)
            .css_classes(["sc-device-switcher"])
            .tooltip_text("Paired phone switcher & quick actions")
            .hexpand(true)
            .build();

        Self {
            button,
            popover,
            dot,
            label,
            sub_label,
            pop_active,
            pop_active_name,
            pop_active_check,
            pop_sub,
            rename,
            telemetry,
            pair,
            prefs,
            connect,
        }
    }
}

impl SidebarNav {
    fn nav_button(icon_name: &str, title: &str, subtitle: &str, shortcut: &str, tooltip: &str, badge: Option<gtk::Label>) -> gtk::Button {
        let icon = gtk::Image::builder().icon_name(icon_name).pixel_size(16).valign(gtk::Align::Center).build();
        let title_lbl = gtk::Label::builder()
            .label(title)
            .xalign(0.0)
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .css_classes(["sc-nav-title"])
            .build();
        let sub_lbl = gtk::Label::builder()
            .label(subtitle)
            .xalign(0.0)
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .css_classes(["sc-nav-sub"])
            .build();
        let text_col = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(1)
            .hexpand(true)
            .build();
        text_col.append(&title_lbl);
        text_col.append(&sub_lbl);

        let right_box = gtk::Box::builder().spacing(5).valign(gtk::Align::Center).build();
        if let Some(badge) = badge {
            right_box.append(&badge);
        }
        right_box.append(&gtk::Label::builder().label(shortcut).css_classes(["sc-nav-shortcut"]).build());

        let row = gtk::Box::builder().spacing(10).build();
        row.append(&icon);
        row.append(&text_col);
        row.append(&right_box);

        gtk::Button::builder()
            .css_classes(["sc-nav-item"])
            .child(&row)
            .tooltip_text(tooltip)
            .hexpand(true)
            .build()
    }

    fn build() -> Self {
        // Brand row + Ctrl+K trigger inside a WindowHandle for smooth dragging.
        let brand_title = overline("Smart Connect");
        brand_title.set_margin_start(2);
        let brand_box = gtk::Box::builder().spacing(8).hexpand(true).valign(gtk::Align::Center).build();
        brand_box.append(&brand_mark());
        brand_box.append(&brand_title);

        let cmd_lbl = gtk::Label::builder().label("Ctrl+K").css_classes(["sc-mono"]).build();
        let cmd_trigger = gtk::Button::builder()
            .child(&cmd_lbl)
            .css_classes(["sc-btn-control"])
            .tooltip_text("Command Palette (Ctrl+K)")
            .valign(gtk::Align::Center)
            .build();

        let brand_row = gtk::Box::builder().spacing(8).build();
        brand_row.append(&brand_box);
        brand_row.append(&cmd_trigger);
        let brand_handle = gtk::WindowHandle::builder().child(&brand_row).build();

        let device = DeviceMenu::build();

        let apps_badge = gtk::Label::builder().css_classes(["sc-nav-badge"]).visible(false).build();
        let notif_badge = gtk::Label::builder().css_classes(["sc-nav-badge"]).visible(false).build();

        let btn_phone = Self::nav_button("phone-symbolic", "Phone", "Overview & Telemetry", "^1", "Phone (Ctrl+1 or Alt+1)", None);
        let btn_apps = Self::nav_button(
            "view-app-grid-symbolic",
            "Apps",
            "Launchpad & Mirroring",
            "^2",
            "Apps (Ctrl+2 or Alt+2)",
            Some(apps_badge.clone()),
        );
        let btn_display = Self::nav_button(
            "video-display-symbolic",
            "Display",
            "Cross Control Studio",
            "^3",
            "Display (Ctrl+3 or Alt+3)",
            None,
        );
        let btn_files = Self::nav_button(
            "folder-symbolic",
            "Files",
            "Storage & SFTP Share",
            "^4",
            "Files (Ctrl+4 or Alt+4)",
            None,
        );
        let btn_notifs = Self::nav_button(
            "preferences-system-notifications-symbolic",
            "Notifications",
            "Alert Relay Center",
            "^5",
            "Notifications (Ctrl+5 or Alt+5)",
            Some(notif_badge.clone()),
        );

        let workspace_hdr = overline("Workspace");
        workspace_hdr.set_margin_start(8);
        workspace_hdr.set_margin_bottom(4);
        let workspace_grp = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(3).build();
        workspace_grp.append(&workspace_hdr);
        workspace_grp.append(&btn_phone);
        workspace_grp.append(&btn_apps);

        let continuity_hdr = overline("Continuity");
        continuity_hdr.set_margin_start(8);
        continuity_hdr.set_margin_bottom(4);
        let continuity_grp = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(3).build();
        continuity_grp.append(&continuity_hdr);
        continuity_grp.append(&btn_display);
        continuity_grp.append(&btn_files);
        continuity_grp.append(&btn_notifs);

        let nav_box = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(18).build();
        nav_box.append(&workspace_grp);
        nav_box.append(&continuity_grp);

        let nav_scroll = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .vexpand(true)
            .child(&nav_box)
            .build();

        let top_col = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(16)
            .margin_top(16)
            .margin_bottom(12)
            .margin_start(14)
            .margin_end(14)
            .vexpand(true)
            .build();
        top_col.append(&brand_handle);
        top_col.append(&device.button);
        top_col.append(&nav_scroll);

        // Sidebar Footer: compact telemetry summary + master Connect CTA.
        let foot_bat_icon = gtk::Image::builder().icon_name("battery-level-60-symbolic").pixel_size(13).build();
        let foot_bat_val = gtk::Label::builder().label("—").css_classes(["sc-mono", "caption"]).build();
        let bat_box = gtk::Box::builder().spacing(4).halign(gtk::Align::Center).hexpand(true).build();
        bat_box.append(&foot_bat_icon);
        bat_box.append(&foot_bat_val);

        let foot_wifi_icon = gtk::Image::builder().icon_name("network-wireless-symbolic").pixel_size(13).css_classes(["sc-secondary"]).build();
        let foot_wifi_val = gtk::Label::builder().label("—").css_classes(["sc-mono", "caption", "sc-secondary"]).build();
        let wifi_box = gtk::Box::builder().spacing(4).halign(gtk::Align::Center).hexpand(true).build();
        wifi_box.append(&foot_wifi_icon);
        wifi_box.append(&foot_wifi_val);

        let foot_bt_icon = gtk::Image::builder().icon_name("bluetooth-active-symbolic").pixel_size(13).build();
        let foot_bt_val = gtk::Label::builder().label("—").css_classes(["sc-mono", "caption"]).build();
        let bt_box = gtk::Box::builder().spacing(4).halign(gtk::Align::Center).hexpand(true).build();
        bt_box.append(&foot_bt_icon);
        bt_box.append(&foot_bt_val);

        let foot_stats = gtk::Box::builder().spacing(6).build();
        foot_stats.append(&bat_box);
        foot_stats.append(&gtk::Label::builder().label("·").css_classes(["sc-muted"]).build());
        foot_stats.append(&wifi_box);
        foot_stats.append(&gtk::Label::builder().label("·").css_classes(["sc-muted"]).build());
        foot_stats.append(&bt_box);

        let foot_cta_spinner = adw::Spinner::builder().visible(false).build();
        let foot_cta_dot = status_dot(8, false);
        foot_cta_dot.set_visible(false);
        let foot_cta_label = gtk::Label::builder().label("Connect Device").build();
        let foot_cta_inner = gtk::Box::builder().spacing(8).halign(gtk::Align::Center).build();
        foot_cta_inner.append(&foot_cta_spinner);
        foot_cta_inner.append(&foot_cta_dot);
        foot_cta_inner.append(&foot_cta_label);
        let foot_cta = gtk::Button::builder()
            .child(&foot_cta_inner)
            .css_classes(["sc-btn-primary"])
            .hexpand(true)
            .build();

        let footer = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(10)
            .margin_top(12)
            .margin_bottom(14)
            .margin_start(14)
            .margin_end(14)
            .css_classes(["sc-sidebar-footer"])
            .build();
        footer.append(&foot_stats);
        footer.append(&foot_cta);

        let root = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .width_request(272)
            .css_classes(["sc-sidebar"])
            .build();
        root.append(&top_col);
        root.append(&footer);

        Self {
            root,
            cmd_trigger,
            device,
            nav_buttons: vec![
                ("phone", btn_phone),
                ("apps", btn_apps),
                ("display", btn_display),
                ("files", btn_files),
                ("notifications", btn_notifs),
            ],
            apps_badge,
            notif_badge,
            foot_bat_icon,
            foot_bat_val,
            foot_wifi_icon,
            foot_wifi_val,
            foot_bt_icon,
            foot_bt_val,
            foot_cta,
            foot_cta_spinner,
            foot_cta_dot,
            foot_cta_label,
        }
    }

    fn set_active(&self, tab: &str, enabled: bool) {
        for (id, btn) in &self.nav_buttons {
            btn.set_sensitive(enabled);
            set_class(btn, "active", enabled && *id == tab);
        }
    }
}

impl TopHeader {
    fn build(sidebar_toggle: &gtk::Button, app_menu: &gtk::MenuButton) -> (Self, gtk::Widget) {
        let crumb_device = gtk::Label::builder()
            .label("Smart Connect")
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .max_width_chars(24)
            .css_classes(["sc-muted"])
            .build();
        let sep = gtk::Label::builder().label("/").css_classes(["sc-muted"]).build();
        let crumb_view = gtk::Label::builder().label("Phone").css_classes(["heading"]).build();
        let status_pill = gtk::Label::builder()
            .label("Connecting…")
            .css_classes(["sc-pill"])
            .margin_start(6)
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .max_width_chars(34)
            .build();

        let left_box = gtk::Box::builder().spacing(8).valign(gtk::Align::Center).build();
        left_box.append(sidebar_toggle);
        left_box.append(&crumb_device);
        left_box.append(&sep);
        left_box.append(&crumb_view);
        left_box.append(&status_pill);

        // Right cluster
        let quick_inner = gtk::Box::builder().spacing(7).valign(gtk::Align::Center).build();
        quick_inner.append(&gtk::Image::builder().icon_name("system-search-symbolic").pixel_size(13).build());
        quick_inner.append(&gtk::Label::new(Some("Quick Command…")));
        quick_inner.append(&gtk::Label::builder().label("Ctrl+K").css_classes(["sc-kbd"]).build());
        let quick_cmd = gtk::Button::builder()
            .child(&quick_inner)
            .css_classes(["sc-btn-control"])
            .tooltip_text("Command Palette (Ctrl+K)")
            .valign(gtk::Align::Center)
            .build();

        let telemetry = gtk::Button::builder()
            .icon_name("dialog-information-symbolic")
            .css_classes(["sc-btn-icon"])
            .tooltip_text("Endpoint Telemetry")
            .valign(gtk::Align::Center)
            .build();

        let connect_spinner = adw::Spinner::builder().visible(false).build();
        let connect_label = gtk::Label::new(Some("Connect"));
        let connect_box = gtk::Box::builder().spacing(6).halign(gtk::Align::Center).build();
        connect_box.append(&connect_spinner);
        connect_box.append(&connect_label);
        let connect = gtk::Button::builder()
            .child(&connect_box)
            .css_classes(["sc-btn-primary"])
            .valign(gtk::Align::Center)
            .build();

        let right_box = gtk::Box::builder().spacing(8).valign(gtk::Align::Center).build();
        right_box.append(&quick_cmd);
        right_box.append(&telemetry);
        right_box.append(&connect);
        right_box.append(app_menu);

        let bar = adw::HeaderBar::builder()
            .show_start_title_buttons(false)
            .show_end_title_buttons(true)
            .css_classes(["sc-header"])
            .title_widget(&gtk::Box::new(gtk::Orientation::Horizontal, 0))
            .build();
        bar.pack_start(&left_box);
        bar.pack_end(&right_box);

        (
            Self {
                crumb_device,
                crumb_view,
                status_pill,
                quick_cmd,
                telemetry,
                connect,
                connect_spinner,
                connect_label,
            },
            bar.upcast(),
        )
    }
}

impl DownPage {
    fn build() -> (Self, gtk::Widget) {
        let (thumb, wallpaper, wallpaper_stack) = wallpaper_thumb(48, 96);
        thumb.add_css_class("sc-dimmed");
        let name = gtk::Label::builder().xalign(0.0).css_classes(["heading"]).wrap(true).build();
        let sub = gtk::Label::builder().xalign(0.0).css_classes(["sc-secondary"]).wrap(true).build();
        let text = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(3).valign(gtk::Align::Center).build();
        text.append(&name);
        text.append(&sub);
        let card = gtk::Box::builder()
            .spacing(14)
            .halign(gtk::Align::Center)
            .margin_top(6)
            .margin_bottom(6)
            .margin_start(14)
            .margin_end(14)
            .css_classes(["sc-stat-tile"])
            .visible(false)
            .build();
        card.append(&thumb);
        card.append(&text);

        let icon = gtk::Image::builder()
            .icon_name("network-offline-symbolic")
            .pixel_size(28)
            .width_request(60)
            .height_request(60)
            .css_classes(["sc-icon-box"])
            .halign(gtk::Align::Center)
            .build();
        let title = gtk::Label::builder()
            .label("Smart Connect service isn't running")
            .css_classes(["sc-view-title"])
            .wrap(true)
            .justify(gtk::Justification::Center)
            .build();
        let desc = gtk::Label::builder()
            .label("Start it to reach your phone. This window reconnects on its own once it is up.")
            .css_classes(["sc-secondary"])
            .wrap(true)
            .justify(gtk::Justification::Center)
            .max_width_chars(46)
            .build();
        let start = gtk::Button::builder()
            .label("Start service")
            .css_classes(["sc-btn-primary"])
            .halign(gtk::Align::Center)
            .build();

        let inner = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(18)
            .margin_top(32)
            .margin_bottom(32)
            .margin_start(28)
            .margin_end(28)
            .halign(gtk::Align::Center)
            .build();
        inner.append(&icon);
        inner.append(&title);
        inner.append(&desc);
        inner.append(&card);
        inner.append(&start);

        let bento = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .css_classes(["sc-bento"])
            .valign(gtk::Align::Center)
            .halign(gtk::Align::Center)
            .build();
        bento.append(&inner);

        let clamp = adw::Clamp::builder().maximum_size(540).child(&bento).vexpand(true).valign(gtk::Align::Center).build();
        (Self { card, wallpaper, wallpaper_stack, name, sub, start }, clamp.upcast())
    }
}

impl PhonePage {
    fn build() -> (Self, gtk::Widget) {
        // ── 1. Endpoint Identity & Telemetry Bento ───────────────────────────
        let (thumb_box, wallpaper, thumb_stack) = wallpaper_thumb(54, 108);

        let status_dot = status_dot(8, false);
        let status_pill = gtk::Label::builder().label("Endpoint Standby").build();
        let status_pill_box = gtk::Box::builder().spacing(7).css_classes(["sc-pill"]).valign(gtk::Align::Center).build();
        status_pill_box.append(&status_dot);
        status_pill_box.append(&status_pill);

        let model_pill = gtk::Label::builder().css_classes(["sc-tag"]).visible(false).valign(gtk::Align::Center).build();
        let badges = gtk::Box::builder().spacing(8).valign(gtk::Align::Center).build();
        badges.append(&status_pill_box);
        badges.append(&model_pill);

        let name = gtk::Label::builder()
            .xalign(0.0)
            .css_classes(["sc-hero-title"])
            .wrap(true)
            .wrap_mode(gtk::pango::WrapMode::WordChar)
            .build();
        let rename_btn = gtk::Button::builder()
            .icon_name("document-edit-symbolic")
            .css_classes(["sc-btn-icon"])
            .tooltip_text("Rename display nickname (saved locally on this PC)")
            .valign(gtk::Align::Center)
            .build();
        let name_view = gtk::Box::builder().spacing(10).valign(gtk::Align::Center).build();
        name_view.append(&name);
        name_view.append(&rename_btn);

        let rename_entry = gtk::Entry::builder()
            .width_chars(22)
            .placeholder_text("Phone display name…")
            .css_classes(["sc-inset-entry"])
            .build();
        let rename_save = gtk::Button::builder()
            .icon_name("object-select-symbolic")
            .css_classes(["sc-btn-primary"])
            .tooltip_text("Save nickname")
            .valign(gtk::Align::Center)
            .build();
        let rename_cancel = gtk::Button::builder()
            .icon_name("window-close-symbolic")
            .css_classes(["sc-btn-secondary"])
            .tooltip_text("Cancel")
            .valign(gtk::Align::Center)
            .build();
        let name_edit = gtk::Box::builder().spacing(8).valign(gtk::Align::Center).build();
        name_edit.append(&rename_entry);
        name_edit.append(&rename_save);
        name_edit.append(&rename_cancel);

        let name_stack = gtk::Stack::new();
        name_stack.add_named(&name_view, Some("display"));
        name_stack.add_named(&name_edit, Some("edit"));

        let subtitle = gtk::Label::builder().xalign(0.0).css_classes(["sc-secondary"]).wrap(true).build();

        let id_col = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(8)
            .hexpand(true)
            .valign(gtk::Align::Center)
            .build();
        id_col.append(&badges);
        id_col.append(&name_stack);
        id_col.append(&subtitle);

        let left_cluster = gtk::Box::builder().spacing(16).hexpand(true).valign(gtk::Align::Start).build();
        left_cluster.append(&thumb_box);
        left_cluster.append(&id_col);

        let mirror = gtk::Button::builder().label("Mirror screen").css_classes(["sc-btn-secondary"]).visible(false).build();
        let telemetry_inner = gtk::Box::builder().spacing(6).build();
        telemetry_inner.append(&gtk::Image::builder().icon_name("dialog-information-symbolic").pixel_size(14).build());
        telemetry_inner.append(&gtk::Label::new(Some("Telemetry")));
        let telemetry = gtk::Button::builder()
            .child(&telemetry_inner)
            .css_classes(["sc-btn-secondary"])
            .tooltip_text("View endpoint telemetry and paths")
            .build();

        let connect_spinner = adw::Spinner::builder().visible(false).build();
        let connect_label = gtk::Label::new(Some("Connect"));
        let connect_box = gtk::Box::builder().spacing(8).halign(gtk::Align::Center).build();
        connect_box.append(&connect_spinner);
        connect_box.append(&connect_label);
        let connect = gtk::Button::builder().css_classes(["sc-btn-primary"]).child(&connect_box).build();
        let pair = gtk::Button::builder().css_classes(["sc-btn-primary"]).label("Pair a phone").visible(false).build();

        let actions = gtk::Box::builder().spacing(8).valign(gtk::Align::Center).build();
        actions.append(&mirror);
        actions.append(&telemetry);
        actions.append(&connect);
        actions.append(&pair);

        let hero_top = gtk::Box::builder().spacing(18).build();
        hero_top.append(&left_cluster);
        hero_top.append(&actions);

        // 4-Column Telemetry Tiles
        let bat_icon = gtk::Image::builder().icon_name("battery-level-60-symbolic").pixel_size(15).build();
        let bat_val = gtk::Label::builder().label("—").xalign(0.0).css_classes(["sc-metric"]).build();
        let bat_segments: [gtk::Box; 5] = std::array::from_fn(|_| {
            gtk::Box::builder()
                .height_request(6)
                .hexpand(true)
                .css_classes(["sc-seg-off"])
                .build()
        });
        let bat_bar = gtk::Box::builder().spacing(4).margin_top(6).build();
        for seg in &bat_segments {
            bat_bar.append(seg);
        }
        let bat_sub = gtk::Label::builder().label("Telemetry offline").xalign(0.0).css_classes(["caption", "sc-muted"]).margin_top(4).build();
        let bat_tile = Self::stat_tile("BATTERY", &bat_icon, &bat_val, &bat_bar, &bat_sub);

        let wifi_icon = gtk::Image::builder().icon_name("network-wireless-symbolic").pixel_size(15).css_classes(["sc-secondary"]).build();
        let wifi_val = gtk::Label::builder().label("—").xalign(0.0).css_classes(["sc-metric-text"]).build();
        let wifi_spacer = gtk::Box::builder().height_request(6).margin_top(6).build();
        let wifi_sub = gtk::Label::builder().label("Link offline").xalign(0.0).css_classes(["caption", "sc-muted", "sc-mono"]).margin_top(4).build();
        let wifi_tile = Self::stat_tile("WI-FI LINK", &wifi_icon, &wifi_val, &wifi_spacer, &wifi_sub);

        let cell_icon = gtk::Image::builder().icon_name("network-cellular-signal-none-symbolic").pixel_size(15).css_classes(["sc-secondary"]).build();
        let cell_val = gtk::Label::builder().label("—").xalign(0.0).css_classes(["sc-metric-text"]).build();
        let cell_bars = [
            gtk::Box::builder().width_request(7).height_request(6).valign(gtk::Align::End).css_classes(["sc-seg-off"]).build(),
            gtk::Box::builder().width_request(7).height_request(8).valign(gtk::Align::End).css_classes(["sc-seg-off"]).build(),
            gtk::Box::builder().width_request(7).height_request(10).valign(gtk::Align::End).css_classes(["sc-seg-off"]).build(),
            gtk::Box::builder().width_request(7).height_request(12).valign(gtk::Align::End).css_classes(["sc-seg-off"]).build(),
        ];
        let cell_sub = gtk::Label::builder().label("Mobile signal").xalign(0.0).css_classes(["caption", "sc-muted", "sc-mono"]).margin_start(6).build();
        let cell_meter = gtk::Box::builder().spacing(3).valign(gtk::Align::End).margin_top(6).build();
        for bar in &cell_bars {
            cell_meter.append(bar);
        }
        cell_meter.append(&cell_sub);
        let cell_tile = Self::stat_tile("CELLULAR", &cell_icon, &cell_val, &cell_meter, &gtk::Box::new(gtk::Orientation::Horizontal, 0));

        let bt_icon = gtk::Image::builder().icon_name("bluetooth-disabled-symbolic").pixel_size(15).build();
        let bt_val = gtk::Label::builder().label("—").xalign(0.0).css_classes(["sc-metric-text"]).build();
        let bt_spacer = gtk::Box::builder().height_request(6).margin_top(6).build();
        let bt_sub = gtk::Label::builder().label("Radio unknown").xalign(0.0).css_classes(["caption", "sc-muted", "sc-mono"]).margin_top(4).build();
        let bt_tile = Self::stat_tile("BLUETOOTH", &bt_icon, &bt_val, &bt_spacer, &bt_sub);

        let stats_flow = gtk::FlowBox::builder()
            .selection_mode(gtk::SelectionMode::None)
            .homogeneous(true)
            .min_children_per_line(2)
            .max_children_per_line(4)
            .column_spacing(12)
            .row_spacing(12)
            .margin_top(20)
            .build();
        for tile in [bat_tile, wifi_tile, cell_tile, bt_tile] {
            stats_flow.append(&tile);
            if let Some(cell) = tile.parent() {
                cell.set_focusable(false);
            }
        }
        let stats_wrap = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_top(22)
            .css_classes(["sc-divider-top"])
            .build();
        stats_wrap.append(&stats_flow);

        let hero_card = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_top(24)
            .margin_bottom(24)
            .margin_start(24)
            .margin_end(24)
            .build();
        hero_card.append(&hero_top);
        hero_card.append(&stats_wrap);
        let hero_bento = gtk::Box::builder().orientation(gtk::Orientation::Vertical).css_classes(["sc-bento"]).build();
        hero_bento.append(&hero_card);

        // ── Unread shortcut ──────────────────────────────────────────────────
        let unread_label = gtk::Label::builder().xalign(0.0).hexpand(true).build();
        let unread_link = gtk::Label::builder().label("Open notifications ›").css_classes(["sc-tag"]).build();
        let unread_row = gtk::Box::builder()
            .spacing(10)
            .margin_top(14)
            .margin_bottom(14)
            .margin_start(18)
            .margin_end(18)
            .build();
        unread_row.append(&gtk::Image::builder().icon_name("preferences-system-notifications-symbolic").pixel_size(16).build());
        unread_row.append(&unread_label);
        unread_row.append(&unread_link);
        let unread_card = gtk::Button::builder().css_classes(["sc-bento-btn"]).visible(false).child(&unread_row).build();

        // ── 2. Cross Control Quick-Launch Bento ──────────────────────────────
        let cross_hdr = gtk::Box::builder().spacing(8).margin_start(4).margin_end(4).build();
        let cross_hdr_l = overline("Cross Control");
        cross_hdr_l.set_hexpand(true);
        cross_hdr.append(&cross_hdr_l);
        cross_hdr.append(&gtk::Label::builder().label("Universal Control & Spatial Display").css_classes(["sc-mono", "caption", "sc-muted"]).build());

        let cross_icon_box = gtk::Box::builder()
            .width_request(44)
            .height_request(44)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .css_classes(["sc-icon-box-white"])
            .build();
        cross_icon_box.append(&gtk::Image::builder().icon_name("input-mouse-symbolic").pixel_size(20).build());

        let cross_edge = gtk::Label::builder().label("Edge: RIGHT").css_classes(["sc-tag"]).valign(gtk::Align::Center).build();
        let cross_title_row = gtk::Box::builder().spacing(10).build();
        cross_title_row.append(&gtk::Label::builder().label("Cross Control").css_classes(["heading"]).build());
        cross_title_row.append(&cross_edge);
        let cross_sub = gtk::Label::builder()
            .label("Shared mouse and keyboard, duplicate display or extend display")
            .xalign(0.0)
            .wrap(true)
            .css_classes(["sc-secondary"])
            .build();
        let cross_text = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(3).hexpand(true).build();
        cross_text.append(&cross_title_row);
        cross_text.append(&cross_sub);

        let cross_pills = [
            gtk::Label::builder().label("Pointer").css_classes(["sc-tag"]).valign(gtk::Align::Center).build(),
            gtk::Label::builder().label("Mirror").css_classes(["sc-tag"]).valign(gtk::Align::Center).build(),
            gtk::Label::builder().label("Extend").css_classes(["sc-tag"]).valign(gtk::Align::Center).build(),
        ];
        let cross_right = gtk::Box::builder().spacing(6).valign(gtk::Align::Center).build();
        for pill in &cross_pills {
            cross_right.append(pill);
        }
        cross_right.append(&gtk::Image::builder().icon_name("go-next-symbolic").pixel_size(15).css_classes(["sc-muted"]).margin_start(4).build());

        let cross_row = gtk::Box::builder()
            .spacing(16)
            .margin_top(18)
            .margin_bottom(18)
            .margin_start(20)
            .margin_end(20)
            .build();
        cross_row.append(&cross_icon_box);
        cross_row.append(&cross_text);
        cross_row.append(&cross_right);
        let cross_btn = gtk::Button::builder().css_classes(["sc-bento-btn"]).child(&cross_row).build();

        let cross_section = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(10).build();
        cross_section.append(&cross_hdr);
        cross_section.append(&cross_btn);

        // ── 3. System Continuity 3-Column Diagnostics ────────────────────────
        let cont_hdr = gtk::Box::builder().spacing(8).margin_start(4).margin_end(4).build();
        let cont_hdr_l = overline("System Continuity");
        cont_hdr_l.set_hexpand(true);
        cont_hdr.append(&cont_hdr_l);
        cont_hdr.append(&gtk::Label::builder().label("App Stream · Storage · Share").css_classes(["sc-mono", "caption", "sc-muted"]).build());

        // Tile A: Recent App Stream
        let recent_icon_slot = IconSlot::new(34);
        let recent_placeholder = gtk::Image::builder()
            .icon_name("view-app-grid-symbolic")
            .pixel_size(18)
            .width_request(34)
            .height_request(34)
            .css_classes(["sc-icon-box-sm"])
            .build();
        let recent_icon_stack = gtk::Stack::new();
        recent_icon_stack.add_named(&recent_placeholder, Some("empty"));
        recent_icon_stack.add_named(recent_icon_slot.widget(), Some("app"));

        let recent_name = gtk::Label::builder()
            .label("No recent apps")
            .xalign(0.0)
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .css_classes(["heading"])
            .build();
        let recent_sub = gtk::Label::builder()
            .label("Open Apps to browse")
            .xalign(0.0)
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .css_classes(["caption", "sc-muted"])
            .build();
        let recent_btn = gtk::Button::builder().label("Apps").css_classes(["sc-btn-secondary"]).valign(gtk::Align::Center).build();
        let recent_card = Self::continuity_tile("RECENT APP STREAM", "view-app-grid-symbolic", &recent_icon_stack, &recent_name, &recent_sub, &recent_btn);

        // Tile B: Mount Location
        let mount_path = gtk::Label::builder()
            .label("~/Phone")
            .xalign(0.0)
            .ellipsize(gtk::pango::EllipsizeMode::Middle)
            .css_classes(["sc-mono", "heading"])
            .build();
        let mount_status = gtk::Label::builder()
            .label("Ready to mount")
            .xalign(0.0)
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .css_classes(["caption", "sc-muted"])
            .build();
        let mount_btn = gtk::Button::builder().label("Files").css_classes(["sc-btn-secondary"]).valign(gtk::Align::Center).build();
        let mount_card = Self::continuity_tile("MOUNT LOCATION", "drive-harddisk-symbolic", &gtk::Box::new(gtk::Orientation::Horizontal, 0), &mount_path, &mount_status, &mount_btn);

        // Tile C: Shared Directory
        let share_path = gtk::Label::builder()
            .label("~")
            .xalign(0.0)
            .ellipsize(gtk::pango::EllipsizeMode::Middle)
            .css_classes(["sc-mono", "heading"])
            .build();
        let share_status = gtk::Label::builder()
            .label("Sharing paused")
            .xalign(0.0)
            .ellipsize(gtk::pango::EllipsizeMode::End)
            .css_classes(["caption", "sc-muted"])
            .build();
        let share_btn = gtk::Button::builder().label("Configure").css_classes(["sc-btn-secondary"]).valign(gtk::Align::Center).build();
        let share_card = Self::continuity_tile("SHARED DIRECTORY", "folder-open-symbolic", &gtk::Box::new(gtk::Orientation::Horizontal, 0), &share_path, &share_status, &share_btn);

        let cont_flow = gtk::FlowBox::builder()
            .selection_mode(gtk::SelectionMode::None)
            .homogeneous(true)
            .min_children_per_line(1)
            .max_children_per_line(3)
            .column_spacing(12)
            .row_spacing(12)
            .build();
        for card in [recent_card, mount_card, share_card] {
            cont_flow.append(&card);
            if let Some(cell) = card.parent() {
                cell.set_focusable(false);
            }
        }
        let continuity_section = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(10).build();
        continuity_section.append(&cont_hdr);
        continuity_section.append(&cont_flow);

        let column = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(24)
            .valign(gtk::Align::Start)
            .margin_top(26)
            .margin_bottom(28)
            .margin_start(20)
            .margin_end(20)
            .build();
        column.append(&hero_bento);
        column.append(&unread_card);
        column.append(&cross_section);
        column.append(&continuity_section);

        let page = Self {
            hero_top,
            thumb_box,
            thumb_stack,
            wallpaper,
            status_dot,
            status_pill,
            model_pill,
            name_stack,
            name,
            rename_btn,
            rename_entry,
            rename_save,
            rename_cancel,
            subtitle,
            mirror,
            telemetry,
            connect,
            connect_spinner,
            connect_label,
            pair,
            unread_card,
            unread_label,
            stats_wrap,
            bat_icon,
            bat_val,
            bat_segments,
            bat_sub,
            wifi_icon,
            wifi_val,
            wifi_sub,
            cell_icon,
            cell_val,
            cell_bars,
            cell_sub,
            bt_icon,
            bt_val,
            bt_sub,
            cross_section,
            cross_btn,
            cross_edge,
            cross_pills,
            continuity_section,
            recent_icon_slot,
            recent_icon_stack,
            recent_name,
            recent_sub,
            recent_btn,
            recent_pkg: RefCell::default(),
            mount_path,
            mount_status,
            mount_btn,
            share_path,
            share_status,
            share_btn,
        };
        (page, page_scroller(&column, 1080).upcast())
    }

    fn stat_tile(
        label: &str,
        icon: &gtk::Image,
        main_val: &gtk::Label,
        visual: &impl IsA<gtk::Widget>,
        sub: &impl IsA<gtk::Widget>,
    ) -> gtk::Box {
        let top = gtk::Box::builder().spacing(8).build();
        let hdr = overline(label);
        hdr.set_hexpand(true);
        top.append(&hdr);
        top.append(icon);

        let body = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(2).valign(gtk::Align::End).vexpand(true).build();
        body.append(main_val);
        body.append(visual);
        body.append(sub);

        let tile = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(12)
            .css_classes(["sc-stat-tile"])
            .build();
        tile.append(&top);
        tile.append(&body);
        tile
    }

    fn continuity_tile(
        overline_text: &str,
        icon_name: &str,
        leading: &impl IsA<gtk::Widget>,
        title: &gtk::Label,
        sub: &gtk::Label,
        action: &gtk::Button,
    ) -> gtk::Box {
        let hdr = gtk::Box::builder().spacing(8).build();
        let ov = overline(overline_text);
        ov.set_hexpand(true);
        hdr.append(&ov);
        hdr.append(&gtk::Image::builder().icon_name(icon_name).pixel_size(14).css_classes(["sc-secondary"]).build());

        let text = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(2).hexpand(true).valign(gtk::Align::Center).build();
        text.append(title);
        text.append(sub);

        let bottom = gtk::Box::builder().spacing(10).valign(gtk::Align::Center).build();
        bottom.append(leading);
        bottom.append(&text);
        bottom.append(action);

        let inner = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(14)
            .margin_top(16)
            .margin_bottom(16)
            .margin_start(18)
            .margin_end(18)
            .build();
        inner.append(&hdr);
        inner.append(&bottom);

        let card = gtk::Box::builder().orientation(gtk::Orientation::Vertical).css_classes(["sc-bento"]).build();
        card.append(&inner);
        card
    }
}

impl NotifPage {
    fn build() -> (Self, gtk::Widget) {
        let muted = adw::ExpanderRow::builder().title("Muted apps").build();
        let muted_group = adw::PreferencesGroup::builder().visible(false).build();
        muted_group.add(&muted);
        let empty = adw::StatusPage::builder()
            .icon_name("preferences-system-notifications-symbolic")
            .title("No Notifications")
            .description("Notifications from the phone show up here while it is connected.")
            .vexpand(true)
            .build();

        let today_label = gtk::Label::builder().label("Today").xalign(0.0).hexpand(true).css_classes(["heading"]).build();
        let clear_all = gtk::Button::builder().label("Clear all").css_classes(["sc-btn-secondary"]).build();
        let today = gtk::Box::builder().spacing(8).visible(false).build();
        today.append(&today_label);
        today.append(&clear_all);

        let list = gtk::ListBox::builder().selection_mode(gtk::SelectionMode::None).css_classes(["boxed-list"]).visible(false).build();
        let column = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(12)
            .margin_top(24)
            .margin_bottom(24)
            .margin_start(20)
            .margin_end(20)
            .build();
        column.append(&muted_group);
        column.append(&empty);
        column.append(&today);
        column.append(&list);
        (Self { empty, today, clear_all, list, muted_group, muted }, page_scroller(&column, 900).upcast())
    }
}

impl PairDialog {
    fn build() -> Self {
        let qr = gtk::Picture::builder()
            .content_fit(gtk::ContentFit::Contain)
            .width_request(200)
            .height_request(200)
            .css_classes(["qr"])
            .build();
        let qr_box = gtk::Box::builder().halign(gtk::Align::Center).valign(gtk::Align::Center).build();
        qr_box.append(&qr);

        let steps = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(12).hexpand(true).valign(gtk::Align::Center).build();
        for (i, text) in ["Open Smart Connect on your phone", "Tap + and choose “Scan QR code”", "Point the camera at this code"].iter().enumerate() {
            let row = gtk::Box::builder().spacing(10).build();
            row.append(&gtk::Label::builder().label(&format!("{}", i + 1)).css_classes(["sc-tag"]).valign(gtk::Align::Start).build());
            row.append(&gtk::Label::builder().label(*text).xalign(0.0).wrap(true).build());
            steps.append(&row);
        }
        let spinner = adw::Spinner::new();
        let status = gtk::Label::builder().label("Waiting for phone…").xalign(0.0).hexpand(true).wrap(true).css_classes(["sc-secondary"]).build();
        let status_row = gtk::Box::builder().spacing(8).margin_top(4).build();
        status_row.append(&spinner);
        status_row.append(&status);
        steps.append(&status_row);

        let top = gtk::Box::builder().spacing(18).build();
        top.append(&qr_box);
        top.append(&steps);
        let button = gtk::Button::builder().label("New code").css_classes(["sc-btn-secondary"]).halign(gtk::Align::End).build();
        let content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(18)
            .margin_top(18)
            .margin_bottom(18)
            .margin_start(18)
            .margin_end(18)
            .build();
        content.append(&top);
        let firewall = crate::setup::FirewallHint::build();
        content.append(&firewall.widget);
        content.append(&button);

        let toolbar = adw::ToolbarView::new();
        toolbar.add_top_bar(&adw::HeaderBar::new());
        toolbar.set_content(Some(&content));
        let dialog = adw::Dialog::builder().title("Pair a new phone").content_width(520).child(&toolbar).build();
        let fw = firewall.clone();
        dialog.connect_map(move |_| fw.check());
        Self { dialog, qr, qr_box, status, spinner, button, firewall }
    }
}

impl Ui {
    pub fn new(app: &adw::Application) -> Rc<Self> {
        let win_prefs = prefs::load().window;

        let views = gtk::Stack::builder().transition_type(gtk::StackTransitionType::Crossfade).build();
        let (phone, phone_w) = PhonePage::build();
        let (notifs, notifs_w) = NotifPage::build();
        let toasts = adw::ToastOverlay::new();
        let (apps, apps_w) = AppsPage::build(&toasts);
        let (display, display_w) = DisplayPage::build(&toasts);
        let (cross, cross_modes, cross_position) = CrossControlPage::build(&toasts, display.clone());
        let display_column = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(24)
            .margin_top(24)
            .margin_bottom(24)
            .margin_start(20)
            .margin_end(20)
            .build();
        display_column.append(&cross_modes);
        display_column.append(&cross_position);
        display_column.append(&display_w);
        let display_clamp = adw::Clamp::builder().maximum_size(1060).tightening_threshold(1060).child(&display_column).build();
        let display_w = gtk::ScrolledWindow::builder().hscrollbar_policy(gtk::PolicyType::Never).vexpand(true).child(&display_clamp).build();
        let (files, files_w) = FilesPage::build(&toasts);

        views.add_named(&phone_w, Some("phone"));
        views.add_named(&apps_w, Some("apps"));
        views.add_named(&display_w, Some("display"));
        views.add_named(&files_w, Some("files"));
        views.add_named(&notifs_w, Some("notifications"));
        views.set_visible_child_name(&win_prefs.last_tab);

        let (down_page, down) = DownPage::build();
        let spinner = adw::Spinner::builder().width_request(44).height_request(44).build();
        let connecting_label = gtk::Label::builder().label("Connecting…").css_classes(["sc-secondary"]).build();
        let connecting = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(16)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .build();
        connecting.append(&spinner);
        connecting.append(&connecting_label);

        let root = gtk::Stack::builder().transition_type(gtk::StackTransitionType::Crossfade).build();
        root.add_named(&connecting, Some("connecting"));
        root.add_named(&down, Some("down"));
        root.add_named(&views, Some("main"));

        let sidebar = SidebarNav::build();
        let sidebar_toggle = gtk::Button::builder()
            .icon_name("sidebar-show-symbolic")
            .css_classes(["sc-btn-icon"])
            .tooltip_text("Show sidebar")
            .visible(false)
            .build();
        let app_menu_model = build_app_menu();
        let app_menu = gtk::MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .menu_model(&app_menu_model)
            .primary(true)
            .css_classes(["sc-btn-icon"])
            .tooltip_text("Main Menu")
            .valign(gtk::Align::Center)
            .build();
        let (header, header_w) = TopHeader::build(&sidebar_toggle, &app_menu);

        toasts.set_child(Some(&root));
        let workspace_toolbar = adw::ToolbarView::new();
        workspace_toolbar.add_top_bar(&header_w);
        workspace_toolbar.set_content(Some(&toasts));
        workspace_toolbar.add_css_class("sc-canvas");

        let split_view = adw::OverlaySplitView::builder()
            .sidebar(&sidebar.root)
            .content(&workspace_toolbar)
            .min_sidebar_width(272.0)
            .max_sidebar_width(272.0)
            .sidebar_width_unit(adw::LengthUnit::Px)
            .show_sidebar(true)
            .pin_sidebar(true)
            .build();

        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("Smart Connect")
            .default_width(win_prefs.width)
            .default_height(win_prefs.height)
            .maximized(win_prefs.maximized)
            .content(&split_view)
            .build();

        // Medium width breakpoint: hide verbose header status badge & Quick Command label when header tightens.
        let medium = adw::Breakpoint::new(adw::BreakpointCondition::parse("max-width: 960sp").expect("valid breakpoint condition"));
        medium.add_setter(&header.status_pill, "visible", Some(&false.to_value()));
        window.add_breakpoint(medium);

        // Narrow width breakpoint: collapse 272px sidebar into an overlay drawer and stack phone hero actions.
        let narrow = adw::Breakpoint::new(adw::BreakpointCondition::parse("max-width: 780sp").expect("valid breakpoint condition"));
        narrow.add_setter(&split_view, "collapsed", Some(&true.to_value()));
        narrow.add_setter(&phone.hero_top, "orientation", Some(&gtk::Orientation::Vertical.to_value()));
        window.add_breakpoint(narrow);

        let pair = PairDialog::build();

        let ui = Rc::new(Self {
            window,
            toasts,
            split_view: split_view.clone(),
            sidebar_toggle: sidebar_toggle.clone(),
            root,
            connecting_label,
            views: views.clone(),
            sidebar,
            header,
            down: down_page,
            phone,
            notifs,
            pair,
            apps,
            display,
            cross,
            files,
            tray: RefCell::default(),
            daemon_ctl: RefCell::default(),
            state: RefCell::new(State { last: lastdevice::load(), ..State::default() }),
        });

        ui.setup_actions(app);
        ui.show_switcher(false);
        ui.refresh_header_tab();
        ui.refresh_apps_chrome();
        ui.refresh_device_offline();
        ui.refresh_down();
        ui.connect_signals();

        // Sidebar collapse visibility sync.
        let toggle_w = ui.sidebar_toggle.downgrade();
        ui.split_view.connect_collapsed_notify(move |sv| {
            if let Some(btn) = toggle_w.upgrade() {
                btn.set_visible(sv.is_collapsed());
            }
        });
        let sv_w = ui.split_view.downgrade();
        ui.sidebar_toggle.connect_clicked(move |_| {
            if let Some(sv) = sv_w.upgrade() {
                sv.set_show_sidebar(!sv.shows_sidebar());
            }
        });

        let weak = Rc::downgrade(&ui);
        views.connect_visible_child_name_notify(move |views| {
            let Some(ui) = weak.upgrade() else { return };
            ui.refresh_header_tab();
            if ui.root.visible_child_name().as_deref() == Some("main") {
                let name = views.visible_child_name();
                ui.on_tab_visible(name.as_deref());
                ui.save_window_prefs();
            }
        });
        let weak = Rc::downgrade(&ui);
        ui.apps.connect_sessions_changed(move || {
            if let Some(ui) = weak.upgrade() {
                ui.refresh_phone();
            }
        });
        let weak = Rc::downgrade(&ui);
        ui.apps.connect_apps_changed(move || {
            if let Some(ui) = weak.upgrade() {
                ui.refresh_apps_chrome();
                ui.refresh_phone();
            }
        });
        let weak = Rc::downgrade(&ui);
        ui.cross.connect_state_changed(move || {
            if let Some(ui) = weak.upgrade() {
                ui.refresh_phone();
            }
        });
        let weak = Rc::downgrade(&ui);
        ui.files.connect_changed(move || {
            if let Some(ui) = weak.upgrade() {
                ui.refresh_phone();
            }
        });

        ui
    }

    fn switch_tab(&self, name: &str) {
        if !TABS.contains(&name) {
            return;
        }
        self.views.set_visible_child_name(name);
        if self.split_view.is_collapsed() {
            self.split_view.set_show_sidebar(false);
        }
        self.refresh_header_tab();
    }

    fn refresh_header_tab(&self) {
        let active = self.views.visible_child_name();
        let tab = active.as_deref().unwrap_or("phone");
        let main_up = self.root.visible_child_name().as_deref() == Some("main");
        self.sidebar.set_active(tab, main_up);
        self.header.crumb_view.set_label(if main_up { view_title(tab) } else { "Status" });
    }

    fn refresh_apps_chrome(&self) {
        let count = self.apps.app_count();
        self.sidebar.apps_badge.set_visible(count > 0);
        if count > 0 {
            self.sidebar.apps_badge.set_label(&count.to_string());
        }
    }

    fn resolved_name(&self, raw_name: Option<&str>, model: Option<&str>, unpaired: bool) -> String {
        if unpaired {
            return "No phone paired".to_owned();
        }
        if let Some(alias) = prefs::load().device_alias.map(|a| a.trim().to_owned()).filter(|a| !a.is_empty()) {
            return alias;
        }
        raw_name
            .filter(|n| !n.is_empty())
            .or_else(|| model.filter(|m| !m.is_empty()))
            .unwrap_or("Phone")
            .to_owned()
    }

    fn on_tab_visible(self: &Rc<Self>, name: Option<&str>) {
        self.display.set_visible(name == Some("display"));
        match name {
            Some("apps") => self.apps.shown(),
            Some("display") => self.cross.refresh(),
            Some("notifications") => self.load_muted(),
            _ => {}
        }
    }

    fn save_window_prefs(&self) {
        let mut p = prefs::load();
        let maximized = self.window.is_maximized();
        if !maximized {
            let (aw, ah) = (self.window.width(), self.window.height());
            let (dw, dh) = self.window.default_size();
            let (w, h) = if self.window.is_visible() && aw > 0 && ah > 0 { (aw, ah) } else { (dw, dh) };
            if w > 0 && h > 0 {
                p.window.width = w;
                p.window.height = h;
            }
        }
        p.window.maximized = maximized;
        if let Some(tab) = self.views.visible_child_name() {
            let name = tab.as_str();
            if TABS.contains(&name) {
                p.window.last_tab = name.to_owned();
            }
        }
        p.window = p.window.clone().sanitized();
        if let Err(e) = prefs::save_gui(&p) {
            eprintln!("smartconnect-gui: could not save window preferences: {e}");
        }
    }

    fn setup_actions(self: &Rc<Self>, app: &adw::Application) {
        let cmd_act = gio::SimpleAction::new("command-palette", None);
        let weak = Rc::downgrade(self);
        cmd_act.connect_activate(move |_, _| {
            if let Some(ui) = weak.upgrade() {
                ui.open_command_palette();
            }
        });
        self.window.add_action(&cmd_act);

        let pair_act = gio::SimpleAction::new("pair", None);
        let weak = Rc::downgrade(self);
        pair_act.connect_activate(move |_, _| {
            if let Some(ui) = weak.upgrade() {
                if ui.state.borrow().status.is_some() {
                    ui.open_pair_dialog();
                } else {
                    ui.toast("Start the Smart Connect service before pairing");
                }
            }
        });
        self.window.add_action(&pair_act);

        let prefs_act = gio::SimpleAction::new("preferences", None);
        let weak = Rc::downgrade(self);
        prefs_act.connect_activate(move |_, _| {
            if let Some(ui) = weak.upgrade() {
                ui.open_preferences();
            }
        });
        self.window.add_action(&prefs_act);

        let shortcuts_act = gio::SimpleAction::new("shortcuts", None);
        let weak = Rc::downgrade(self);
        shortcuts_act.connect_activate(move |_, _| {
            if let Some(ui) = weak.upgrade() {
                ui.open_shortcuts();
            }
        });
        self.window.add_action(&shortcuts_act);

        let about_act = gio::SimpleAction::new("about", None);
        let weak = Rc::downgrade(self);
        about_act.connect_activate(move |_, _| {
            if let Some(ui) = weak.upgrade() {
                ui.open_about();
            }
        });
        self.window.add_action(&about_act);

        let focus_search_act = gio::SimpleAction::new("focus-search", None);
        let weak = Rc::downgrade(self);
        focus_search_act.connect_activate(move |_, _| {
            if let Some(ui) = weak.upgrade() {
                if ui.root.visible_child_name().as_deref() == Some("main") && ui.window.visible_dialog().is_none() {
                    ui.switch_tab("apps");
                    ui.apps.focus_search();
                }
            }
        });
        self.window.add_action(&focus_search_act);

        let hide_act = gio::SimpleAction::new("hide", None);
        let weak = Rc::downgrade(self);
        hide_act.connect_activate(move |_, _| {
            if let Some(ui) = weak.upgrade() {
                ui.window.close();
            }
        });
        self.window.add_action(&hide_act);

        let quit_act = gio::SimpleAction::new("quit", None);
        let weak = Rc::downgrade(self);
        quit_act.connect_activate(move |_, _| {
            if let Some(ui) = weak.upgrade() {
                ui.quit();
            }
        });
        self.window.add_action(&quit_act);

        let tab_act = gio::SimpleAction::new("tab", Some(glib::VariantTy::STRING));
        let weak = Rc::downgrade(self);
        tab_act.connect_activate(move |_, param| {
            let Some(ui) = weak.upgrade() else { return };
            if ui.root.visible_child_name().as_deref() != Some("main") || ui.window.visible_dialog().is_some() {
                return;
            }
            if let Some(name) = param.and_then(|v| v.str()) {
                ui.switch_tab(name);
            }
        });
        self.window.add_action(&tab_act);

        app.set_accels_for_action("win.command-palette", &["<Primary>k"]);
        app.set_accels_for_action("win.preferences", &["<Primary>comma"]);
        app.set_accels_for_action("win.shortcuts", &["<Primary>question"]);
        app.set_accels_for_action("win.pair", &["<Primary><Shift>p"]);
        app.set_accels_for_action("win.focus-search", &["<Primary>f"]);
        app.set_accels_for_action("win.hide", &["<Primary>w"]);
        app.set_accels_for_action("win.quit", &["<Primary>q"]);
        app.set_accels_for_action("win.tab::phone", &["<Alt>1", "<Primary>1"]);
        app.set_accels_for_action("win.tab::apps", &["<Alt>2", "<Primary>2"]);
        app.set_accels_for_action("win.tab::display", &["<Alt>3", "<Primary>3"]);
        app.set_accels_for_action("win.tab::files", &["<Alt>4", "<Primary>4"]);
        app.set_accels_for_action("win.tab::notifications", &["<Alt>5", "<Primary>5"]);
    }

    pub fn present(&self) {
        self.window.present();
        self.refresh_tray();
    }

    pub fn shutdown(&self) {
        {
            let mut st = self.state.borrow_mut();
            let st = &mut *st;
            if let Some(mirror) = st.mirror.as_mut() {
                mirror.stop();
            }
            if st.status.as_ref().is_some_and(|s| s.connected) {
                st.last.last_connected = Some(lastdevice::now_secs());
                if let Err(e) = lastdevice::save(&st.last) {
                    eprintln!("smartconnect-gui: could not save the last device: {e}");
                }
            }
        }
        self.files.shutdown();
        self.cross.shutdown();
    }

    fn connect_signals(self: &Rc<Self>) {
        // Sidebar nav clicks
        for (id, btn) in &self.sidebar.nav_buttons {
            let weak = Rc::downgrade(self);
            let id = *id;
            btn.connect_clicked(move |_| {
                if let Some(ui) = weak.upgrade() {
                    if ui.root.visible_child_name().as_deref() == Some("main") {
                        ui.switch_tab(id);
                    }
                }
            });
        }

        // Command palette triggers (sidebar + top header)
        for btn in [&self.sidebar.cmd_trigger, &self.header.quick_cmd] {
            let weak = Rc::downgrade(self);
            btn.connect_clicked(move |_| {
                if let Some(ui) = weak.upgrade() {
                    ui.open_command_palette();
                }
            });
        }

        // Telemetry modal triggers (header, phone card, popover)
        for btn in [&self.header.telemetry, &self.phone.telemetry, &self.sidebar.device.telemetry] {
            let weak = Rc::downgrade(self);
            btn.connect_clicked(move |_| {
                if let Some(ui) = weak.upgrade() {
                    ui.sidebar.device.popover.popdown();
                    ui.open_telemetry_dialog();
                }
            });
        }

        // Primary CTA buttons (sidebar footer + header + phone card)
        for btn in [&self.sidebar.foot_cta, &self.header.connect, &self.phone.connect] {
            let weak = Rc::downgrade(self);
            btn.connect_clicked(move |_| {
                if let Some(ui) = weak.upgrade() {
                    ui.primary_cta_clicked();
                }
            });
        }

        // Phone page actions
        let weak = Rc::downgrade(self);
        self.phone.mirror.connect_clicked(move |_| {
            if let Some(ui) = weak.upgrade() {
                ui.toggle_mirror();
            }
        });
        let weak = Rc::downgrade(self);
        self.phone.cross_btn.connect_clicked(move |_| {
            if let Some(ui) = weak.upgrade() {
                ui.switch_tab("display");
            }
        });
        let weak = Rc::downgrade(self);
        self.phone.unread_card.connect_clicked(move |_| {
            if let Some(ui) = weak.upgrade() {
                ui.switch_tab("notifications");
            }
        });
        let weak = Rc::downgrade(self);
        self.phone.pair.connect_clicked(move |_| {
            if let Some(ui) = weak.upgrade() {
                ui.open_pair_dialog();
            }
        });

        // Phone continuity shortcuts
        let weak = Rc::downgrade(self);
        self.phone.recent_btn.connect_clicked(move |_| {
            let Some(ui) = weak.upgrade() else { return };
            let pkg = ui.phone.recent_pkg.borrow().clone();
            if let Some(pkg) = pkg {
                ui.apps.launch_by_pkg(&pkg);
            } else {
                ui.switch_tab("apps");
            }
        });
        for btn in [&self.phone.mount_btn, &self.phone.share_btn] {
            let weak = Rc::downgrade(self);
            btn.connect_clicked(move |_| {
                if let Some(ui) = weak.upgrade() {
                    ui.switch_tab("files");
                }
            });
        }

        // Inline device rename
        let weak = Rc::downgrade(self);
        self.phone.rename_btn.connect_clicked(move |_| {
            if let Some(ui) = weak.upgrade() {
                ui.open_rename_editor();
            }
        });
        let weak = Rc::downgrade(self);
        self.phone.rename_save.connect_clicked(move |_| {
            if let Some(ui) = weak.upgrade() {
                ui.save_device_rename();
            }
        });
        let weak = Rc::downgrade(self);
        self.phone.rename_entry.connect_activate(move |_| {
            if let Some(ui) = weak.upgrade() {
                ui.save_device_rename();
            }
        });
        let weak = Rc::downgrade(self);
        self.phone.rename_cancel.connect_clicked(move |_| {
            if let Some(ui) = weak.upgrade() {
                ui.cancel_device_rename();
            }
        });
        let key_ctl = gtk::EventControllerKey::new();
        let weak = Rc::downgrade(self);
        key_ctl.connect_key_pressed(move |_, key, _, _| {
            if key == gdk::Key::Escape {
                if let Some(ui) = weak.upgrade() {
                    ui.cancel_device_rename();
                }
                return glib::Propagation::Stop;
            }
            glib::Propagation::Proceed
        });
        self.phone.rename_entry.add_controller(key_ctl);

        // Down page
        let weak = Rc::downgrade(self);
        self.down.start.connect_clicked(move |_| {
            if let Some(ui) = weak.upgrade() {
                ui.start_service();
            }
        });

        // Sidebar endpoint popover items
        let pop_w = self.sidebar.device.popover.downgrade();
        self.sidebar.device.pop_active.connect_clicked(move |_| {
            if let Some(p) = pop_w.upgrade() {
                p.popdown();
            }
        });
        let weak = Rc::downgrade(self);
        self.sidebar.device.rename.connect_clicked(move |_| {
            if let Some(ui) = weak.upgrade() {
                ui.sidebar.device.popover.popdown();
                if ui.root.visible_child_name().as_deref() == Some("main") {
                    ui.switch_tab("phone");
                    ui.open_rename_editor();
                }
            }
        });
        let weak = Rc::downgrade(self);
        self.sidebar.device.connect.connect_clicked(move |_| {
            if let Some(ui) = weak.upgrade() {
                ui.sidebar.device.popover.popdown();
                ui.primary_cta_clicked();
            }
        });
        let weak = Rc::downgrade(self);
        self.sidebar.device.pair.connect_clicked(move |_| {
            if let Some(ui) = weak.upgrade() {
                ui.sidebar.device.popover.popdown();
                ui.open_pair_dialog();
            }
        });
        let weak = Rc::downgrade(self);
        self.sidebar.device.prefs.connect_clicked(move |_| {
            if let Some(ui) = weak.upgrade() {
                ui.sidebar.device.popover.popdown();
                ui.open_preferences();
            }
        });

        // Pair dialog
        let weak = Rc::downgrade(self);
        self.pair.button.connect_clicked(move |_| {
            if let Some(ui) = weak.upgrade() {
                ui.request_pairing();
            }
        });

        // Notifications clear all
        let weak = Rc::downgrade(self);
        self.notifs.clear_all.connect_clicked(move |_| {
            if let Some(ui) = weak.upgrade() {
                ui.clear_all_notifications();
            }
        });

        // Window close -> hide to tray or quit
        let weak = Rc::downgrade(self);
        self.window.connect_close_request(move |_| {
            if let Some(ui) = weak.upgrade() {
                ui.save_window_prefs();
                if prefs::load().window.hide_on_close {
                    ui.window.set_visible(false);
                    ui.refresh_tray();
                } else {
                    ui.quit();
                }
            }
            glib::Propagation::Stop
        });
    }

    fn primary_cta_clicked(self: &Rc<Self>) {
        let state = {
            let st = self.state.borrow();
            match &st.status {
                None => None,
                Some(s) if !s.paired => Some(PhoneState::Unpaired),
                Some(s) if s.connected => Some(PhoneState::Connected),
                Some(_) => Some(PhoneState::Disconnected),
            }
        };
        match state {
            None => self.start_service(),
            Some(PhoneState::Unpaired) => self.open_pair_dialog(),
            Some(PhoneState::Connected | PhoneState::Disconnected) => self.toggle_connection(),
        }
    }

    fn open_rename_editor(&self) {
        let current = self.phone.name.label();
        self.phone.rename_entry.set_text(current.as_str());
        self.phone.name_stack.set_visible_child_name("edit");
        self.phone.rename_entry.grab_focus();
    }

    fn cancel_device_rename(&self) {
        self.phone.name_stack.set_visible_child_name("display");
    }

    fn save_device_rename(&self) {
        let entered = self.phone.rename_entry.text().trim().to_owned();
        self.phone.name_stack.set_visible_child_name("display");
        let mut p = prefs::load();
        let next = (!entered.is_empty()).then_some(entered);
        if p.device_alias != next {
            p.device_alias = next.clone();
            if let Err(e) = prefs::save_gui(&p) {
                self.toast(&format!("Could not save device nickname: {e}"));
                return;
            }
            self.toast(match next {
                Some(ref name) => {
                    let _ = name;
                    "Phone display name updated"
                }
                None => "Restored original phone name",
            });
        }
        if self.state.borrow().status.is_some() {
            self.refresh_phone();
        } else {
            self.refresh_device_offline();
        }
    }

    fn open_command_palette(self: &Rc<Self>) {
        let (daemon_up, paired, connected, mirroring, display_name) = {
            let st = self.state.borrow();
            let mirroring = st.mirror.as_ref().is_some_and(Mirror::running) || self.apps.mirror_session().is_some();
            match &st.status {
                None => (false, st.last.known(), false, mirroring, self.resolved_name(st.last.name.as_deref(), st.last.model.as_deref(), false)),
                Some(s) => {
                    let unpaired = !s.paired;
                    let raw = s.name.as_deref().or(st.last.name.as_deref());
                    let model = s.model.as_deref().or(st.last.model.as_deref());
                    (true, s.paired, s.connected, mirroring, self.resolved_name(raw, model, unpaired))
                }
            }
        };

        let mut entries = Vec::new();
        if !daemon_up {
            entries.push(CommandEntry::new("cta:start", "Start Smart Connect service", "Service", "Enter", "daemon"));
        } else if !paired {
            entries.push(CommandEntry::new("action:pair", "Pair a new phone…", "Continuity Link", "Ctrl+Shift+P", "qr"));
        } else {
            entries.push(CommandEntry::new(
                "cta:toggle",
                if connected { format!("Disconnect {display_name}") } else { format!("Connect {display_name}") },
                "Continuity Link",
                "Enter",
                "wifi phone",
            ));
        }

        if daemon_up {
            if connected || mirroring {
                entries.push(CommandEntry::new(
                    "action:mirror",
                    if mirroring { "Stop phone screen mirror" } else { "Mirror phone screen" },
                    "Screen Mirror",
                    "Mirror",
                    "stream display",
                ));
            }
            entries.push(CommandEntry::new("tab:phone", "Go to Phone — Overview & Telemetry", "Navigation", "Ctrl+1", ""));
            entries.push(CommandEntry::new("tab:apps", format!("Go to Apps — Browse {} apps", self.apps.app_count()), "Navigation", "Ctrl+2", ""));
            entries.push(CommandEntry::new("tab:display", "Go to Display — Cross Control Studio", "Navigation", "Ctrl+3", ""));
            entries.push(CommandEntry::new("tab:files", "Go to Files — Storage & SFTP Share", "Navigation", "Ctrl+4", ""));
            entries.push(CommandEntry::new("tab:notifications", "Go to Notifications — Alert Relay", "Navigation", "Ctrl+5", ""));

            let mounted = self.files.is_mounted();
            let mount_path = format_display_path(&prefs::load().files.mount_path());
            entries.push(CommandEntry::new(
                "action:mount",
                if mounted { format!("Unmount {mount_path}") } else { format!("Mount {mount_path} (Read-Only)") },
                "Storage",
                "FUSE",
                "files sftp",
            ));

            if paired {
                entries.push(CommandEntry::new("cc:mouse", "Cross Control: Toggle Shared mouse & keyboard", "Cross Control", "Ctrl+Alt+Esc", ""));
                entries.push(CommandEntry::new("cc:duplicate", "Cross Control: Toggle Duplicate display", "Cross Control", "Mirror", ""));
                entries.push(CommandEntry::new("cc:extend", "Cross Control: Toggle Extend display", "Cross Control", "Extend", ""));
            }
            entries.push(CommandEntry::new("action:telemetry", "View Endpoint Telemetry", "Diagnostics", "", "battery wifi specs"));
            entries.push(CommandEntry::new("action:pair", "Pair a new phone…", "Setup", "Ctrl+Shift+P", "qr"));
        }

        entries.push(CommandEntry::new("action:prefs", "Open Preferences", "Settings", "Ctrl+,", ""));
        entries.push(CommandEntry::new("action:shortcuts", "Keyboard Shortcuts", "Help", "Ctrl+?", ""));

        if daemon_up {
            entries.extend(palette::build_app_commands(&self.apps.all_apps()));
        }

        let weak = Rc::downgrade(self);
        palette::present_command_palette(&self.window, entries, move |cmd| {
            let Some(ui) = weak.upgrade() else { return };
            if let Some(tab) = cmd.strip_prefix("tab:") {
                ui.switch_tab(tab);
            } else if let Some(pkg) = cmd.strip_prefix("app:") {
                ui.switch_tab("apps");
                ui.apps.launch_by_pkg(pkg);
            } else if let Some(mode) = cmd.strip_prefix("cc:") {
                ui.switch_tab("display");
                ui.cross.toggle_mode_id(mode);
            } else {
                match cmd {
                    "cta:start" | "cta:toggle" => ui.primary_cta_clicked(),
                    "action:mirror" => ui.toggle_mirror(),
                    "action:mount" => {
                        ui.switch_tab("files");
                        ui.files.toggle_mount();
                    }
                    "action:telemetry" => ui.open_telemetry_dialog(),
                    "action:pair" => ui.open_pair_dialog(),
                    "action:prefs" => ui.open_preferences(),
                    "action:shortcuts" => ui.open_shortcuts(),
                    _ => {}
                }
            }
        });
    }

    fn open_telemetry_dialog(&self) {
        let p = prefs::load();
        let st = self.state.borrow();
        let (endpoint, model, status_txt, battery_txt, radio_txt, phone_ip, host_ip) = match &st.status {
            Some(s) if !s.paired => (
                "No phone paired".to_owned(),
                "—".to_owned(),
                "Unpaired".to_owned(),
                "Unknown".to_owned(),
                "—".to_owned(),
                "—".to_owned(),
                "—".to_owned(),
            ),
            Some(s) => {
                let raw = s.name.as_deref().or(st.last.name.as_deref());
                let mdl = s.model.as_deref().or(st.last.model.as_deref());
                let ep = self.resolved_name(raw, mdl, false);
                let mut v = LastDevice::default();
                if s.connected {
                    v.absorb(s);
                } else {
                    v = st.last.clone();
                }
                let st_line = if s.connected {
                    "Connected over Wi-Fi".to_owned()
                } else {
                    st.last.last_seen().map_or_else(|| "Disconnected".to_owned(), |t| format!("Disconnected · {t}"))
                };
                let bat = v.battery.map_or_else(
                    || "Unknown".to_owned(),
                    |b| if v.charging && s.connected { format!("{b}% · Charging") } else { format!("{b}%") },
                );
                let rad = format!(
                    "Wi-Fi {} · Cell {} · BT {}",
                    wifi_summary_word(&v),
                    signal_text(v.cell_signal),
                    if v.bluetooth { "On" } else { "Off" }
                );
                (
                    ep,
                    mdl.unwrap_or("—").to_owned(),
                    st_line,
                    bat,
                    rad,
                    s.phone_address.as_deref().unwrap_or("—").to_owned(),
                    s.pc_address.as_deref().unwrap_or("—").to_owned(),
                )
            }
            None => (
                self.resolved_name(st.last.name.as_deref(), st.last.model.as_deref(), false),
                st.last.model.clone().unwrap_or_else(|| "—".to_owned()),
                "Service stopped".to_owned(),
                st.last.battery.map_or_else(|| "Unknown".to_owned(), |b| format!("{b}% (cached)")),
                "—".to_owned(),
                "—".to_owned(),
                "—".to_owned(),
            ),
        };
        drop(st);

        let app_cnt = self.apps.app_count();
        let icon_cnt = count_cached_icons(&icons::cache_dir());
        let rows = vec![
            ("Endpoint", endpoint),
            ("Hardware Model", model),
            ("Link Status", status_txt),
            ("Phone IP", phone_ip),
            ("Host Interface", host_ip),
            ("Battery", battery_txt),
            ("Radios", radio_txt),
            ("Apps & Icons", format!("{app_cnt} apps · {icon_cnt} cached icons")),
            ("Mount Path", format!("{} ({})", format_display_path(&p.files.mount_path()), self.files.mount_status_label())),
            ("Shared Path", format!("{} ({})", format_display_path(&p.files.share_path()), self.files.share_status_label())),
        ];
        palette::present_telemetry_dialog(&self.window, &rows);
    }

    pub fn start(self: &Rc<Self>) {
        let (daemon, started) = Daemon::ensure_running();
        *self.daemon_ctl.borrow_mut() = daemon;

        if let Some((tray, commands)) = tray::Tray::spawn() {
            *self.tray.borrow_mut() = Some(tray);
            self.listen_tray(commands);
        }
        self.listen(daemon::subscribe());
        if started {
            self.begin_starting();
        }
        self.refresh_tray();
        let weak = Rc::downgrade(self);
        glib::timeout_add_seconds_local(60, move || {
            let Some(ui) = weak.upgrade() else { return glib::ControlFlow::Break };
            if ui.state.borrow().status.is_some() {
                ui.refresh_phone();
            } else {
                ui.refresh_down();
                ui.refresh_device_offline();
            }
            glib::ControlFlow::Continue
        });
    }

    fn start_service(self: &Rc<Self>) {
        {
            let mut st = self.state.borrow_mut();
            if st.service_starting || st.starting {
                return;
            }
            st.service_starting = true;
        }
        self.down.start.set_sensitive(false);
        self.down.start.set_label("Starting…");
        self.refresh_device_offline();
        let weak = Rc::downgrade(self);
        glib::spawn_future_local(async move {
            let result = gio::spawn_blocking(Daemon::ensure_running).await;
            let Some(ui) = weak.upgrade() else { return };
            ui.state.borrow_mut().service_starting = false;
            ui.down.start.set_sensitive(true);
            ui.down.start.set_label("Start service");
            match result {
                Ok((daemon, true)) => {
                    *ui.daemon_ctl.borrow_mut() = daemon;
                    ui.begin_starting();
                }
                Ok(_) if lifecycle::socket_alive() => {}
                Ok(_) => ui.toast(
                    "Could not start smartconnectd. Is it installed? Run it in a terminal, or check `journalctl --user -u smartconnectd`.",
                ),
                Err(_) => ui.toast("Could not start smartconnectd (worker thread failed)"),
            }
            ui.refresh_device_offline();
        });
    }

    fn listen_tray(self: &Rc<Self>, commands: async_channel::Receiver<tray::Command>) {
        let weak = Rc::downgrade(self);
        glib::spawn_future_local(async move {
            while let Ok(command) = commands.recv().await {
                let Some(ui) = weak.upgrade() else { break };
                match command {
                    tray::Command::ToggleWindow => ui.toggle_window(),
                    tray::Command::Connect => ui.tray_connect(true),
                    tray::Command::Disconnect => ui.tray_connect(false),
                    tray::Command::Quit => ui.quit(),
                }
            }
        });
    }

    fn toggle_window(self: &Rc<Self>) {
        if self.window.is_visible() {
            self.window.set_visible(false);
        } else {
            self.window.present();
        }
        self.refresh_tray();
    }

    fn tray_connect(self: &Rc<Self>, connect: bool) {
        let act = {
            let st = self.state.borrow();
            !st.busy && st.status.as_ref().is_some_and(|s| s.connected != connect)
        };
        if act {
            self.toggle_connection();
        }
    }

    fn quit(self: &Rc<Self>) {
        self.save_window_prefs();
        self.shutdown();
        self.daemon_ctl.borrow_mut().stop();
        if let Some(app) = self.window.application() {
            app.quit();
        }
    }

    fn open_preferences(self: &Rc<Self>) {
        let general = adw::PreferencesGroup::builder().title("General").build();

        let start_at_login = adw::SwitchRow::builder()
            .title("Start at login")
            .subtitle("Launch Smart Connect in the background when you sign in")
            .build();
        start_at_login.set_active(lifecycle::autostart_enabled());
        let weak = Rc::downgrade(self);
        start_at_login.connect_active_notify(move |row| {
            if let Some(ui) = weak.upgrade() {
                ui.set_start_at_login(row, row.is_active());
            }
        });
        general.add(&start_at_login);

        let has_tray = self.tray.borrow().is_some();
        let keep_running = adw::SwitchRow::builder()
            .title("Keep running in background")
            .subtitle(if has_tray {
                "Closing the window hides it to the system tray instead of quitting"
            } else {
                "Closing the window keeps Smart Connect running in the background; reopen from the app launcher"
            })
            .build();
        keep_running.set_active(prefs::load().window.hide_on_close);
        let weak = Rc::downgrade(self);
        keep_running.connect_active_notify(move |row| {
            let Some(ui) = weak.upgrade() else { return };
            let on = row.is_active();
            let mut p = prefs::load();
            if p.window.hide_on_close == on {
                return;
            }
            p.window.hide_on_close = on;
            match prefs::save_gui(&p) {
                Ok(()) => ui.toast(if on {
                    "Closing the window will keep Smart Connect running in the background"
                } else {
                    "Closing the window will quit Smart Connect"
                }),
                Err(e) => {
                    ui.toast(&format!("Could not save background setting: {e}"));
                    row.set_active(!on);
                }
            }
        });
        general.add(&keep_running);

        let diagnostics = adw::PreferencesGroup::builder().title("Service & Diagnostics").build();

        let (service_sub, stopped) = {
            let st = self.state.borrow();
            match &st.status {
                Some(s) if s.connected => ("Connected to your phone", false),
                Some(_) => ("Running · phone disconnected", false),
                None if st.starting || st.service_starting => ("Starting…", false),
                None if lifecycle::socket_alive() => ("Running", false),
                None => ("Stopped", true),
            }
        };
        let service_row = adw::ActionRow::builder()
            .title("Background service (smartconnectd)")
            .subtitle(service_sub)
            .build();
        if stopped {
            let start_btn = gtk::Button::builder().label("Start").css_classes(["sc-btn-primary"]).valign(gtk::Align::Center).build();
            let weak = Rc::downgrade(self);
            let row_w = service_row.downgrade();
            start_btn.connect_clicked(move |btn| {
                btn.set_sensitive(false);
                btn.set_label("Starting…");
                if let Some(r) = row_w.upgrade() {
                    r.set_subtitle("Starting…");
                }
                if let Some(ui) = weak.upgrade() {
                    ui.start_service();
                }
            });
            service_row.add_suffix(&start_btn);
        }
        diagnostics.add(&service_row);

        let cache_dir = icons::cache_dir();
        let cached_count = count_cached_icons(&cache_dir);
        let cache_subtitle = match cached_count {
            0 => "No cached icons".to_owned(),
            1 => "1 cached icon".to_owned(),
            n => format!("{n} cached icons"),
        };
        let cache_row = adw::ActionRow::builder().title("App icon cache").subtitle(&cache_subtitle).build();
        let clear_btn = gtk::Button::builder()
            .label("Clear")
            .css_classes(["sc-btn-secondary"])
            .valign(gtk::Align::Center)
            .sensitive(cached_count > 0)
            .build();
        let weak = Rc::downgrade(self);
        let cache_row_w = cache_row.downgrade();
        clear_btn.connect_clicked(move |btn| {
            let dir = icons::cache_dir();
            match clear_cached_icons(&dir) {
                Ok(n) => {
                    icons::rescan();
                    btn.set_sensitive(false);
                    if let Some(r) = cache_row_w.upgrade() {
                        r.set_subtitle("No cached icons");
                    }
                    if let Some(ui) = weak.upgrade() {
                        ui.toast(&match n {
                            0 => "App icon cache is already empty".to_owned(),
                            1 => "Cleared 1 cached app icon".to_owned(),
                            n => format!("Cleared {n} cached app icons"),
                        });
                    }
                }
                Err(e) => {
                    if let Some(ui) = weak.upgrade() {
                        ui.toast(&format!("Could not clear icon cache: {e}"));
                    }
                }
            }
        });
        cache_row.add_suffix(&clear_btn);
        diagnostics.add(&cache_row);

        let config_dir = prefs::config_dir();
        let config_row = adw::ActionRow::builder()
            .title("Configuration folder")
            .subtitle(&format_display_path(&config_dir))
            .build();
        let open_cfg_btn = gtk::Button::builder().label("Open").css_classes(["sc-btn-secondary"]).valign(gtk::Align::Center).build();
        let weak = Rc::downgrade(self);
        open_cfg_btn.connect_clicked(move |_| {
            let Some(ui) = weak.upgrade() else { return };
            let dir = prefs::config_dir();
            let _ = std::fs::create_dir_all(&dir);
            let launcher = gtk::FileLauncher::new(Some(&gio::File::for_path(&dir)));
            let weak_ui = Rc::downgrade(&ui);
            launcher.launch(Some(&ui.window), gio::Cancellable::NONE, move |res| {
                if let Err(e) = res {
                    if !e.matches(gtk::DialogError::Dismissed) {
                        if let Some(ui) = weak_ui.upgrade() {
                            ui.toast(&format!("Could not open folder: {e}"));
                        }
                    }
                }
            });
        });
        config_row.add_suffix(&open_cfg_btn);
        diagnostics.add(&config_row);

        let help = adw::PreferencesGroup::builder().title("Help & About").build();
        let dialog = adw::PreferencesDialog::builder().title("Preferences").build();

        let shortcuts_row = adw::ActionRow::builder()
            .title("Keyboard Shortcuts")
            .subtitle("Navigation, Command Palette, and quick actions")
            .activatable(true)
            .build();
        shortcuts_row.add_suffix(&gtk::Image::from_icon_name("go-next-symbolic"));
        let weak = Rc::downgrade(self);
        let dlg_w = dialog.downgrade();
        shortcuts_row.connect_activated(move |_| {
            if let Some(dlg) = dlg_w.upgrade() {
                dlg.close();
            }
            if let Some(ui) = weak.upgrade() {
                ui.open_shortcuts();
            }
        });
        help.add(&shortcuts_row);

        let about_row = adw::ActionRow::builder()
            .title("About Smart Connect")
            .subtitle(concat!("Version ", env!("CARGO_PKG_VERSION")))
            .activatable(true)
            .build();
        about_row.add_suffix(&gtk::Image::from_icon_name("go-next-symbolic"));
        let weak = Rc::downgrade(self);
        let dlg_w = dialog.downgrade();
        about_row.connect_activated(move |_| {
            if let Some(dlg) = dlg_w.upgrade() {
                dlg.close();
            }
            if let Some(ui) = weak.upgrade() {
                ui.open_about();
            }
        });
        help.add(&about_row);

        let page = adw::PreferencesPage::new();
        page.add(&general);
        page.add(&diagnostics);
        page.add(&help);
        dialog.add(&page);
        dialog.present(Some(&self.window));
    }

    fn open_shortcuts(&self) {
        let keycap = |text: &str| {
            gtk::Label::builder()
                .label(text)
                .valign(gtk::Align::Center)
                .css_classes(["sc-kbd"])
                .build()
        };
        let row = |title: &str, keys: &[&str]| {
            let r = adw::ActionRow::builder().title(title).build();
            let box_ = gtk::Box::builder().spacing(6).valign(gtk::Align::Center).build();
            for k in keys {
                box_.append(&keycap(k));
            }
            r.add_suffix(&box_);
            r
        };

        let nav = adw::PreferencesGroup::builder().title("Navigation & Spotlight").build();
        nav.add(&row("Command Palette", &["Ctrl + K"]));
        nav.add(&row("Phone", &["Alt + 1", "Ctrl + 1"]));
        nav.add(&row("Apps", &["Alt + 2", "Ctrl + 2"]));
        nav.add(&row("Display", &["Alt + 3", "Ctrl + 3"]));
        nav.add(&row("Files", &["Alt + 4", "Ctrl + 4"]));
        nav.add(&row("Notifications", &["Alt + 5", "Ctrl + 5"]));
        nav.add(&row("Search phone apps", &["Ctrl + F"]));

        let app_grp = adw::PreferencesGroup::builder().title("Application").build();
        app_grp.add(&row("Pair a new phone", &["Ctrl + Shift + P"]));
        app_grp.add(&row("Preferences", &["Ctrl + ,"]));
        app_grp.add(&row("Keyboard Shortcuts", &["Ctrl + ?"]));
        app_grp.add(&row("Close or hide window", &["Ctrl + W"]));
        app_grp.add(&row("Quit Smart Connect", &["Ctrl + Q"]));

        let page = adw::PreferencesPage::new();
        page.add(&nav);
        page.add(&app_grp);

        let dialog = adw::PreferencesDialog::builder()
            .title("Keyboard Shortcuts")
            .content_width(460)
            .content_height(540)
            .build();
        dialog.add(&page);
        dialog.present(Some(&self.window));
    }

    fn open_about(&self) {
        let dialog = adw::AboutDialog::builder()
            .application_name("Smart Connect")
            .application_icon("dev.smartconnect.Linux")
            .version(env!("CARGO_PKG_VERSION"))
            .developer_name("Smart Connect for Linux Contributors")
            .comments("Pair, mirror, stream apps, share clipboard, forward notifications, and extend your desktop onto your Motorola phone.")
            .website("https://github.com/Lalitmukesh69/smartconnect-linux-ui")
            .issue_url("https://github.com/Lalitmukesh69/smartconnect-linux-ui/issues")
            .license_type(gtk::License::MitX11)
            .build();
        dialog.present(Some(&self.window));
    }

    fn set_start_at_login(&self, row: &adw::SwitchRow, on: bool) {
        match lifecycle::set_autostart(on) {
            Ok(()) => self.toast(if on {
                "Smart Connect will start at login"
            } else {
                "Smart Connect will no longer start at login"
            }),
            Err(e) => {
                self.toast(&format!("Could not change the login setting: {e}"));
                row.set_active(lifecycle::autostart_enabled());
            }
        }
    }

    fn begin_starting(self: &Rc<Self>) {
        self.state.borrow_mut().starting = true;
        self.connecting_label.set_label("Starting smartconnectd…");
        self.root.set_visible_child_name("connecting");
        self.show_switcher(false);
        self.refresh_header_tab();
        self.refresh_device_offline();
        self.refresh_tray();
        let weak = Rc::downgrade(self);
        glib::timeout_add_local_once(Duration::from_secs(20), move || {
            let Some(ui) = weak.upgrade() else { return };
            let was_starting = std::mem::take(&mut ui.state.borrow_mut().starting);
            ui.connecting_label.set_label("Connecting…");
            if was_starting && ui.state.borrow().status.is_none() {
                ui.set_down();
            }
        });
    }

    fn tray_state(&self) -> tray::State {
        let st = self.state.borrow();
        if st.starting {
            return tray::State::Starting;
        }
        match &st.status {
            Some(s) if s.connected => tray::State::Connected,
            Some(_) => tray::State::Disconnected,
            None => tray::State::DaemonDown,
        }
    }

    fn refresh_tray(&self) {
        let state = self.tray_state();
        let visible = self.window.is_visible();
        if let Some(tray) = self.tray.borrow().as_ref() {
            tray.set(state, visible);
        }
    }

    pub fn listen(self: &Rc<Self>, events: async_channel::Receiver<Event>) {
        let weak = Rc::downgrade(self);
        glib::spawn_future_local(async move {
            while let Ok(event) = events.recv().await {
                let Some(ui) = weak.upgrade() else { break };
                match event {
                    Event::Status(status) => ui.apply_status(*status),
                    Event::Down => ui.set_down(),
                    Event::CrossControlRequested => {
                        ui.present();
                        ui.switch_tab("display");
                        ui.cross.refresh();
                    }
                }
            }
        });
    }

    fn toast(&self, message: &str) {
        if !message.is_empty() {
            self.toasts.add_toast(adw::Toast::new(message));
        }
    }

    fn show_switcher(&self, on: bool) {
        if let Some(act) = self.window.lookup_action("pair").and_downcast::<gio::SimpleAction>() {
            act.set_enabled(on);
        }
    }

    fn command(self: &Rc<Self>, line: String, done: impl FnOnce(&Rc<Self>, Result<Reply, String>) + 'static) {
        let weak = Rc::downgrade(self);
        glib::spawn_future_local(async move {
            let result = gio::spawn_blocking(move || daemon::command(&line))
                .await
                .unwrap_or_else(|_| Err("command thread failed".into()));
            if let Some(ui) = weak.upgrade() {
                done(&ui, result);
            }
        });
    }

    fn toast_reply(&self, result: &Result<Reply, String>) {
        match result {
            Ok(r) if !r.message.is_empty() => self.toast(&r.message),
            Ok(r) => self.toast(if r.ok { "Done" } else { "Failed" }),
            Err(e) => self.toast(e),
        }
    }

    fn set_down(self: &Rc<Self>) {
        if self.state.borrow().starting {
            self.refresh_tray();
            return;
        }
        self.display.set_visible(false);
        self.apps.set_connected(false);
        self.display.set_connected(false);
        self.cross.set_connected(false);
        self.files.set_connected(false);
        {
            let mut st = self.state.borrow_mut();
            st.status = None;
            st.busy = false;
            st.muted = None;
            if std::mem::take(&mut st.was_connected) {
                st.last.last_connected = Some(lastdevice::now_secs());
                if let Err(e) = lastdevice::save(&st.last) {
                    eprintln!("smartconnect-gui: could not save the last device: {e}");
                }
            }
        }
        self.root.set_visible_child_name("down");
        self.show_switcher(false);
        self.refresh_header_tab();
        self.refresh_down();
        self.refresh_device_offline();
        self.refresh_tray();
    }

    fn apply_status(self: &Rc<Self>, status: Status) {
        let entering_main = self.root.visible_child_name().as_deref() != Some("main");
        {
            let mut st = self.state.borrow_mut();
            if !st.logged_first {
                st.logged_first = true;
                eprintln!(
                    "smartconnect-gui: first status from smartconnectd (connected={}, paired={}, {} notifications)",
                    status.connected,
                    status.paired,
                    status.notifications.len()
                );
            }
            if st.show_qr {
                if let Some(qr) = status.pairing_qr.clone() {
                    st.qr_path = Some(qr);
                }
            }
            st.starting = false;
            st.status = Some(status);
        }
        self.remember_device();
        self.show_switcher(true);
        let (connected, muted_known) = {
            let st = self.state.borrow();
            (st.status.as_ref().is_some_and(|s| s.connected), st.muted.is_some())
        };
        self.apps.set_connected(connected);
        self.display.set_connected(connected);
        self.cross.set_connected(connected);
        self.files.set_connected(connected);
        self.root.set_visible_child_name("main");
        self.refresh_header_tab();
        if entering_main {
            self.on_tab_visible(self.views.visible_child_name().as_deref());
        }
        self.refresh_phone();
        self.refresh_notifications();
        self.refresh_pair();
        self.refresh_tray();
        if !muted_known {
            self.load_muted();
        }
    }

    fn remember_device(&self) {
        let mut guard = self.state.borrow_mut();
        let st = &mut *guard;
        let Some(s) = st.status.as_ref() else { return };
        let now = lastdevice::now_secs();
        let save = if s.connected {
            let mut next = st.last.clone();
            next.absorb(s);
            if s.wallpaper.is_some() && s.wallpaper != st.copied_wallpaper {
                if let Some(copy) = s.wallpaper_path().and_then(|p| lastdevice::copy_wallpaper(Path::new(p))) {
                    next.wallpaper = Some(copy);
                }
                st.copied_wallpaper = s.wallpaper.clone();
            }
            let save = !next.same_fields(&st.last) || !st.was_connected || now.saturating_sub(st.last_saved) >= 300;
            next.last_connected = Some(now);
            st.last = next;
            st.was_connected = true;
            save
        } else if st.was_connected {
            st.last.last_connected = Some(now);
            st.was_connected = false;
            true
        } else {
            false
        };
        if save {
            st.last_saved = now;
            if let Err(e) = lastdevice::save(&st.last) {
                eprintln!("smartconnect-gui: could not save the last device: {e}");
            }
        }
    }

    fn refresh_phone(&self) {
        let mut st = self.state.borrow_mut();
        let busy = st.busy;
        let pending = st.mirror_pending;
        let mirroring = st.mirror.as_ref().is_some_and(Mirror::running) || self.apps.mirror_session().is_some();
        let Some(s) = st.status.clone() else { return };
        let last = st.last.clone();
        let p = &self.phone;
        let state = if s.connected {
            PhoneState::Connected
        } else if s.paired {
            PhoneState::Disconnected
        } else {
            PhoneState::Unpaired
        };

        let wall = s
            .wallpaper
            .clone()
            .filter(|_| s.wallpaper_path().is_some_and(|p| Path::new(p).is_file()))
            .or_else(|| last.wallpaper_path().map(str::to_owned))
            .filter(|_| state != PhoneState::Unpaired);
        if wall != st.wallpaper {
            st.wallpaper = wall.clone();
            show_wallpaper(&p.wallpaper, &p.thumb_stack, wall.as_deref());
        }
        drop(st);
        set_dimmed(&p.thumb_box, state != PhoneState::Connected);

        let raw_name = s.name.clone().filter(|n| !n.is_empty()).or_else(|| last.name.clone());
        let model = s.model.clone().filter(|m| !m.is_empty()).or_else(|| last.model.clone());
        let display_name = self.resolved_name(raw_name.as_deref(), model.as_deref(), state == PhoneState::Unpaired);

        p.name.set_label(&display_name);
        p.rename_btn.set_visible(state != PhoneState::Unpaired);

        set_dot_online(&p.status_dot, s.connected);
        p.status_pill.set_label(match state {
            PhoneState::Connected => "Continuity Session Online",
            PhoneState::Disconnected if busy => "Handshaking…",
            PhoneState::Disconnected => "Endpoint Standby",
            PhoneState::Unpaired => "No Phone Paired",
        });

        let model_tag = model.as_deref().filter(|m| state != PhoneState::Unpaired && *m != display_name);
        p.model_pill.set_visible(model_tag.is_some());
        if let Some(m) = model_tag {
            p.model_pill.set_label(m);
        }

        let seen = last.last_seen();
        let line = match state {
            PhoneState::Connected => "Connected over Wi-Fi · Cross Control & storage ready".to_owned(),
            PhoneState::Disconnected if busy => "Negotiating connection over Wi-Fi…".to_owned(),
            PhoneState::Disconnected => seen.as_ref().map_or_else(|| "Disconnected".to_owned(), |t| format!("Disconnected · {t}")),
            PhoneState::Unpaired => "Pair your Motorola phone with Smart Connect to get started".to_owned(),
        };
        p.subtitle.set_label(&line);

        // Live values while connected, last cached values while away.
        let values = if state == PhoneState::Connected {
            let mut v = LastDevice::default();
            v.absorb(&s);
            Some(v)
        } else {
            Some(last.clone()).filter(|l| state == PhoneState::Disconnected && l.known())
        };
        p.stats_wrap.set_visible(values.is_some());
        if let Some(v) = &values {
            self.fill_monochrome_telemetry(v, state == PhoneState::Connected);
        }
        set_dimmed(&p.stats_wrap, state != PhoneState::Connected);

        // Unread notifications shortcut
        let unread = s.unread.unwrap_or(0).max(0);
        p.unread_card.set_visible(state == PhoneState::Connected && unread > 0);
        p.unread_label.set_label(&match unread {
            1 => "1 unread notification from your phone".to_owned(),
            n => format!("{n} unread notifications from your phone"),
        });

        // Phone Hero action buttons
        p.mirror.set_visible(state == PhoneState::Connected || mirroring);
        p.mirror.set_sensitive(!pending && (s.connected || mirroring));
        p.mirror.set_label(if pending {
            "Starting…"
        } else if mirroring {
            "Stop mirroring"
        } else {
            "Mirror screen"
        });
        set_class(&p.mirror, "destructive-action", mirroring);
        set_class(&p.mirror, "sc-btn-secondary", !mirroring);

        p.telemetry.set_visible(state != PhoneState::Unpaired);
        p.pair.set_visible(state == PhoneState::Unpaired);

        p.connect.set_visible(state != PhoneState::Unpaired);
        p.connect.set_sensitive(!busy);
        p.connect_spinner.set_visible(busy);
        p.connect_label.set_label(if busy {
            if s.connected { "Disconnecting…" } else { "Connecting…" }
        } else if s.connected {
            "Disconnect"
        } else {
            "Connect"
        });
        set_class(&p.connect, "sc-btn-primary", !s.connected);
        set_class(&p.connect, "sc-btn-secondary", s.connected);

        // Cross Control Quick-Launch Bento
        p.cross_section.set_visible(s.paired);
        p.cross_edge.set_label(&format!("Edge: {}", self.cross.edge_label()));
        let active_cc = self.cross.active_mode_id();
        for (pill, id) in p.cross_pills.iter().zip(["mouse", "duplicate", "extend"]) {
            let on = active_cc == Some(id);
            set_class(pill, "sc-tag-active", on);
            set_class(pill, "sc-tag", !on);
        }

        // System Continuity 3-Column Diagnostics
        p.continuity_section.set_visible(s.paired);
        if let Some(recent) = self.apps.recent_app() {
            p.recent_icon_slot.set(&recent.query());
            p.recent_icon_stack.set_visible_child_name("app");
            p.recent_name.set_label(&recent.name);
            p.recent_name.set_tooltip_text(Some(&recent.pkg));
            p.recent_sub.set_label("Last streamed · Ready");
            p.recent_btn.set_label("Launch");
            *p.recent_pkg.borrow_mut() = Some(recent.pkg);
        } else {
            p.recent_icon_stack.set_visible_child_name("empty");
            p.recent_name.set_label("No recent streams");
            p.recent_name.set_tooltip_text(None);
            let cnt = self.apps.app_count();
            p.recent_sub.set_label(&if cnt > 0 { format!("{cnt} apps ready in Launchpad") } else { "Browse phone apps".to_owned() });
            p.recent_btn.set_label("Apps");
            *p.recent_pkg.borrow_mut() = None;
        }
        let fp = prefs::load().files;
        p.mount_path.set_label(&format_display_path(&fp.mount_path()));
        p.mount_status.set_label(&self.files.mount_status_label());
        p.share_path.set_label(&format_display_path(&fp.share_path()));
        p.share_status.set_label(&self.files.share_status_label());

        // Sidebar Endpoint Switcher, Footer Telemetry, and Header Breadcrumb/CTA
        self.sync_chrome_online(&display_name, &s, &last, values.as_ref(), state, busy, seen.as_deref());
    }

    fn sync_chrome_online(
        &self,
        display_name: &str,
        s: &Status,
        last: &LastDevice,
        values: Option<&LastDevice>,
        state: PhoneState,
        busy: bool,
        seen: Option<&str>,
    ) {
        let sb = &self.sidebar;
        let d = &sb.device;
        d.label.set_label(display_name);
        set_dot_online(&d.dot, s.connected);

        let bat_opt = s.battery.or(last.battery);
        let compact_bat = bat_opt.filter(|_| state != PhoneState::Unpaired).map(|b| format!(" · {b}%")).unwrap_or_default();
        let sb_sub = match state {
            PhoneState::Connected => format!("Connected{compact_bat}"),
            PhoneState::Disconnected if busy => "Connecting…".to_owned(),
            PhoneState::Disconnected => format!("Disconnected{compact_bat}"),
            PhoneState::Unpaired => "No phone paired".to_owned(),
        };
        d.sub_label.set_label(&sb_sub);

        d.pop_active_name.set_label(display_name);
        d.pop_active_check.set_visible(s.connected);
        d.pop_sub.set_label(&match state {
            PhoneState::Connected => match s.battery {
                Some(b) => format!("Wi-Fi · {b}%"),
                None => "Wi-Fi".to_owned(),
            },
            PhoneState::Disconnected if busy => "Connecting…".to_owned(),
            PhoneState::Disconnected => seen.map_or_else(|| "Disconnected".to_owned(), capitalize),
            PhoneState::Unpaired => "Scan QR to pair a device".to_owned(),
        });
        d.rename.set_sensitive(state != PhoneState::Unpaired);
        d.pair.set_sensitive(true);
        d.connect.set_visible(s.paired);
        d.connect.set_sensitive(!busy);
        if let Some(label) = d.connect.child().and_downcast::<gtk::Label>() {
            label.set_label(if busy {
                "Working…"
            } else if s.connected {
                "Disconnect"
            } else {
                "Connect"
            });
        }

        // Sidebar footer telemetry
        if let Some(v) = values {
            match v.battery {
                Some(b) => {
                    let level = (b.clamp(0, 100) + 5) / 10 * 10;
                    let charging = v.charging && state == PhoneState::Connected;
                    let icon = match (level, charging) {
                        (100, true) => "battery-level-100-charged-symbolic".to_owned(),
                        (l, true) => format!("battery-level-{l}-charging-symbolic"),
                        (l, false) => format!("battery-level-{l}-symbolic"),
                    };
                    sb.foot_bat_icon.set_icon_name(Some(&icon));
                    sb.foot_bat_val.set_label(&format!("{b}%"));
                }
                None => {
                    sb.foot_bat_icon.set_icon_name(Some("battery-missing-symbolic"));
                    sb.foot_bat_val.set_label("—");
                }
            }
            let w_word = wifi_summary_word(v);
            let w_icon = match signal_level(v.wifi_signal).filter(|_| v.wifi) {
                Some(l) => format!("network-wireless-signal-{}-symbolic", SIGNAL_ICONS[l]),
                None if v.wifi => "network-wireless-symbolic".to_owned(),
                None => "network-wireless-offline-symbolic".to_owned(),
            };
            sb.foot_wifi_icon.set_icon_name(Some(&w_icon));
            sb.foot_wifi_val.set_label(w_word);
            sb.foot_bt_icon.set_icon_name(Some(if v.bluetooth { "bluetooth-active-symbolic" } else { "bluetooth-disabled-symbolic" }));
            sb.foot_bt_val.set_label(if v.bluetooth { "On" } else { "Off" });
        } else {
            sb.foot_bat_icon.set_icon_name(Some("battery-missing-symbolic"));
            sb.foot_bat_val.set_label("—");
            sb.foot_wifi_icon.set_icon_name(Some("network-wireless-offline-symbolic"));
            sb.foot_wifi_val.set_label("—");
            sb.foot_bt_icon.set_icon_name(Some("bluetooth-disabled-symbolic"));
            sb.foot_bt_val.set_label("—");
        }

        // Sidebar footer CTA + Header CTA + Breadcrumb
        sb.foot_cta.set_sensitive(!busy);
        sb.foot_cta_spinner.set_visible(busy);
        sb.foot_cta_dot.set_visible(!busy && s.connected);
        set_dot_online(&sb.foot_cta_dot, true);
        sb.foot_cta_label.set_label(match state {
            _ if busy => "Handshaking…",
            PhoneState::Connected => "Disconnect Device",
            PhoneState::Disconnected => "Connect Device",
            PhoneState::Unpaired => "Pair a Phone",
        });
        set_class(&sb.foot_cta, "sc-btn-primary", !s.connected);
        set_class(&sb.foot_cta, "sc-btn-secondary", s.connected);

        let h = &self.header;
        h.crumb_device.set_label(display_name);
        h.status_pill.set_label(&match state {
            PhoneState::Connected => "Connected over Wi-Fi".to_owned(),
            PhoneState::Disconnected if busy => "Connecting…".to_owned(),
            PhoneState::Disconnected => seen.map_or_else(|| "Disconnected".to_owned(), |t| format!("Disconnected · {t}")),
            PhoneState::Unpaired => "Unpaired".to_owned(),
        });
        h.connect.set_sensitive(!busy);
        h.connect_spinner.set_visible(busy);
        h.connect_label.set_label(match state {
            _ if busy => "Working…",
            PhoneState::Connected => "Disconnect",
            PhoneState::Disconnected => "Connect",
            PhoneState::Unpaired => "Pair",
        });
        set_class(&h.connect, "sc-btn-primary", !s.connected);
        set_class(&h.connect, "sc-btn-secondary", s.connected);
    }

    fn fill_monochrome_telemetry(&self, v: &LastDevice, live: bool) {
        let p = &self.phone;
        let pre = if live { "" } else { "Last known · " };

        match v.battery {
            Some(b) => {
                let b_clamped = b.clamp(0, 100);
                let level = (b_clamped + 5) / 10 * 10;
                let charging = v.charging && live;
                let icon = match (level, charging) {
                    (100, true) => "battery-level-100-charged-symbolic".to_owned(),
                    (l, true) => format!("battery-level-{l}-charging-symbolic"),
                    (l, false) => format!("battery-level-{l}-symbolic"),
                };
                p.bat_icon.set_icon_name(Some(&icon));
                p.bat_val.set_label(&format!("{b}%"));
                let lit = ((b_clamped as f64) / 20.0).ceil() as usize;
                for (i, seg) in p.bat_segments.iter().enumerate() {
                    set_class(seg, "sc-seg-on", i < lit);
                    set_class(seg, "sc-seg-off", i >= lit);
                }
                p.bat_sub.set_label(if charging {
                    "Charging via USB/AC"
                } else if live {
                    "Live battery telemetry"
                } else {
                    "Last known charge"
                });
            }
            None => {
                p.bat_icon.set_icon_name(Some("battery-missing-symbolic"));
                p.bat_val.set_label("—");
                for seg in &p.bat_segments {
                    set_class(seg, "sc-seg-on", false);
                    set_class(seg, "sc-seg-off", true);
                }
                p.bat_sub.set_label(&format!("{pre}Unknown"));
            }
        }

        let wifi_word = wifi_summary_word(v);
        match signal_level(v.wifi_signal).filter(|_| v.wifi) {
            Some(l) => {
                p.wifi_icon.set_icon_name(Some(&format!("network-wireless-signal-{}-symbolic", SIGNAL_ICONS[l])));
                p.wifi_val.set_label(wifi_word);
                p.wifi_sub.set_label(if live { "Active Wi-Fi continuity" } else { "Last known signal" });
            }
            None if v.wifi => {
                p.wifi_icon.set_icon_name(Some("network-wireless-symbolic"));
                p.wifi_val.set_label("On");
                p.wifi_sub.set_label(if live { "Wi-Fi radio enabled" } else { "Last known state" });
            }
            None => {
                p.wifi_icon.set_icon_name(Some("network-wireless-offline-symbolic"));
                p.wifi_val.set_label("Off");
                p.wifi_sub.set_label(if live { "Wi-Fi disabled" } else { "Last known state" });
            }
        }

        let cell_word = signal_text(v.cell_signal);
        let cell_icon = signal_level(v.cell_signal).map_or("none", |l| SIGNAL_ICONS[l]);
        p.cell_icon.set_icon_name(Some(&format!("network-cellular-signal-{cell_icon}-symbolic")));
        p.cell_val.set_label(cell_word);
        let lit_bars = cellular_bars(v.cell_signal);
        for (i, bar) in p.cell_bars.iter().enumerate() {
            set_class(bar, "sc-seg-on", i < lit_bars);
            set_class(bar, "sc-seg-off", i >= lit_bars);
        }
        p.cell_sub.set_label(if live { "Mobile signal" } else { "Last known" });

        if v.bluetooth {
            p.bt_icon.set_icon_name(Some("bluetooth-active-symbolic"));
            p.bt_val.set_label("On");
            p.bt_sub.set_label(if live { "BLE proximity active" } else { "Last known state" });
        } else {
            p.bt_icon.set_icon_name(Some("bluetooth-disabled-symbolic"));
            p.bt_val.set_label("Off");
            p.bt_sub.set_label(if live { "Radio disabled" } else { "Last known state" });
        }
    }

    fn refresh_device_offline(&self) {
        let st = self.state.borrow();
        let last = st.last.clone();
        let starting = st.starting || st.service_starting;
        drop(st);

        let display_name = self.resolved_name(last.name.as_deref(), last.model.as_deref(), !last.known());
        let sb = &self.sidebar;
        let d = &sb.device;
        d.label.set_label(if last.known() { &display_name } else { "Smart Connect" });
        d.sub_label.set_label(if starting { "Starting service…" } else { "Service stopped" });
        set_dot_online(&d.dot, false);

        d.pop_active_name.set_label(if last.known() { &display_name } else { "Smart Connect" });
        d.pop_sub.set_label(if starting { "Starting the Smart Connect service…" } else { "Smart Connect service isn't running" });
        d.pop_active_check.set_visible(false);
        d.rename.set_sensitive(false);
        d.pair.set_sensitive(false);
        d.connect.set_visible(true);
        d.connect.set_sensitive(!starting);
        if let Some(label) = d.connect.child().and_downcast::<gtk::Label>() {
            label.set_label(if starting { "Starting…" } else { "Start service" });
        }

        sb.foot_bat_val.set_label("—");
        sb.foot_wifi_val.set_label("—");
        sb.foot_bt_val.set_label("—");
        sb.foot_cta.set_sensitive(!starting);
        sb.foot_cta_spinner.set_visible(starting);
        sb.foot_cta_dot.set_visible(false);
        sb.foot_cta_label.set_label(if starting { "Starting Service…" } else { "Start Service" });
        set_class(&sb.foot_cta, "sc-btn-primary", true);
        set_class(&sb.foot_cta, "sc-btn-secondary", false);

        let h = &self.header;
        h.crumb_device.set_label(if last.known() { &display_name } else { "Smart Connect" });
        h.status_pill.set_label(if starting { "Starting smartconnectd…" } else { "Service stopped" });
        h.connect.set_sensitive(!starting);
        h.connect_spinner.set_visible(starting);
        h.connect_label.set_label(if starting { "Starting…" } else { "Start service" });
        set_class(&h.connect, "sc-btn-primary", true);
        set_class(&h.connect, "sc-btn-secondary", false);
    }

    fn refresh_down(&self) {
        let mut st = self.state.borrow_mut();
        let last = st.last.clone();
        self.down.card.set_visible(last.known());
        let name = self.resolved_name(last.name.as_deref(), last.model.as_deref(), false);
        self.down.name.set_label(&name);
        self.down.sub.set_label(&last.last_seen().map_or_else(|| "Not connected yet".to_owned(), |t| capitalize(&t)));
        let wall = last.wallpaper_path().map(str::to_owned);
        if wall != st.down_wallpaper {
            st.down_wallpaper = wall.clone();
            show_wallpaper(&self.down.wallpaper, &self.down.wallpaper_stack, wall.as_deref());
        }
    }

    fn toggle_connection(self: &Rc<Self>) {
        let connected = {
            let mut st = self.state.borrow_mut();
            if st.busy {
                return;
            }
            st.busy = true;
            st.status.as_ref().is_some_and(|s| s.connected)
        };
        self.refresh_phone();
        let cmd = if connected { "disconnect" } else { "connect" };
        self.command(cmd.into(), move |ui, result| {
            ui.state.borrow_mut().busy = false;
            match &result {
                Ok(r) if !r.ok => {
                    let why = if r.message.is_empty() { "no reason given" } else { r.message.as_str() };
                    ui.toast(&format!("Couldn't {cmd}: {why}"));
                }
                _ => ui.toast_reply(&result),
            }
            ui.refresh_phone();
        });
    }

    fn toggle_mirror(self: &Rc<Self>) {
        if let Some(sid) = self.apps.mirror_session() {
            let weak = Rc::downgrade(self);
            glib::spawn_future_local(async move {
                let result = gio::spawn_blocking(move || apps::close(sid)).await.unwrap_or_else(|_| Err("close thread failed".into()));
                let Some(ui) = weak.upgrade() else { return };
                if let Err(e) = result {
                    ui.toast(&e);
                }
                ui.apps.poll_soon();
            });
            return;
        }
        {
            let mut st = self.state.borrow_mut();
            if st.mirror_pending {
                return;
            }
            if let Some(mirror) = st.mirror.as_mut().filter(|m| m.running()) {
                mirror.stop();
                drop(st);
                self.refresh_phone();
                return;
            }
            st.mirror_pending = true;
        }
        self.refresh_phone();
        let weak = Rc::downgrade(self);
        glib::spawn_future_local(async move {
            let result = gio::spawn_blocking(apps::start_mirror).await.unwrap_or_else(|_| Err("mirror thread failed".into()));
            let Some(ui) = weak.upgrade() else { return };
            ui.state.borrow_mut().mirror_pending = false;
            match result {
                Ok(_) => {
                    ui.toast("Mirroring opens in its own window");
                    ui.apps.poll_soon();
                }
                Err(e) => {
                    eprintln!("smartconnect-gui: stream host unavailable ({e}); running the viewer standalone");
                    ui.start_viewer();
                }
            }
            ui.refresh_phone();
        });
    }

    fn start_viewer(self: &Rc<Self>) {
        let started = {
            let mut st = self.state.borrow_mut();
            let mirror = st.mirror.get_or_insert_with(Mirror::viewer);
            match mirror.start(&[]) {
                Ok(()) => true,
                Err(e) => {
                    drop(st);
                    self.toast(&e);
                    false
                }
            }
        };
        if started {
            self.watch_mirror();
        }
    }

    fn watch_mirror(self: &Rc<Self>) {
        let weak: Weak<Self> = Rc::downgrade(self);
        glib::timeout_add_local(Duration::from_millis(500), move || {
            let Some(ui) = weak.upgrade() else { return glib::ControlFlow::Break };
            let exit = ui.state.borrow_mut().mirror.as_mut().and_then(Mirror::poll);
            match exit {
                None => glib::ControlFlow::Continue,
                Some(exit) => {
                    if let Exit::Failed(msg) = exit {
                        ui.toast(&msg);
                    }
                    ui.apps.poll_soon();
                    ui.refresh_phone();
                    glib::ControlFlow::Break
                }
            }
        });
    }

    // ── Notifications page ──────────────────────────────────────────────────

    fn refresh_notifications(self: &Rc<Self>) {
        let raw = match &self.state.borrow().status {
            Some(s) => s.notifications.clone(),
            None => return,
        };
        let connected = self.state.borrow().status.as_ref().is_some_and(|s| s.connected);

        let notifications: Vec<Notification> = {
            let mut st = self.state.borrow_mut();
            let present: HashSet<&String> = raw.iter().map(|n| &n.key).collect();
            st.dismissed.retain(|k| present.contains(k));
            raw.iter().filter(|n| !st.dismissed.contains(&n.key)).cloned().collect()
        };

        let count = notifications.len();
        self.sidebar.notif_badge.set_visible(count > 0);
        if count > 0 {
            self.sidebar.notif_badge.set_label(&count.to_string());
        }
        self.notifs.empty.set_visible(notifications.is_empty());
        self.notifs.today.set_visible(!notifications.is_empty());
        self.notifs.list.set_visible(!notifications.is_empty());

        let mut st = self.state.borrow_mut();
        for n in notifications.iter().filter(|n| !n.pkg.is_empty() && !n.app.is_empty()) {
            st.app_names.insert(n.pkg.clone(), n.app.clone());
        }
        st.rows.retain(|key, row| {
            let keep = notifications.iter().any(|n| &n.key == key);
            if !keep {
                self.notifs.list.remove(&row.row);
            }
            keep
        });
        for n in &notifications {
            if !st.rows.contains_key(&n.key) {
                let row = self.notification_row(n);
                st.rows.insert(n.key.clone(), row);
            }
            let muted = st.muted.as_ref().is_some_and(|m| m.contains(&n.pkg));
            let row = st.rows.get_mut(&n.key).expect("row just inserted");
            if row.data != *n {
                row.update(n, &self.apps.icon_query(&n.pkg, &n.app));
            }
            row.send.set_sensitive(connected);
            row.entry.set_sensitive(connected);
            row.open_item.set_sensitive(connected);
            row.set_muted(muted);
        }
        let mut child = self.notifs.list.first_child();
        let in_order = notifications.iter().all(|n| {
            let ok = child.as_ref() == Some(st.rows[&n.key].row.upcast_ref());
            child = child.as_ref().and_then(|c| c.next_sibling());
            ok
        });
        if !in_order {
            for n in &notifications {
                let row = &st.rows[&n.key].row;
                if row.parent().is_some() {
                    self.notifs.list.remove(row);
                }
            }
            for n in &notifications {
                self.notifs.list.append(&st.rows[&n.key].row);
            }
        }
    }

    fn notification_row(self: &Rc<Self>, n: &Notification) -> NotifRow {
        let icon = IconSlot::new(32);
        icon.widget().set_valign(gtk::Align::Start);
        let header = gtk::Label::builder().xalign(0.0).wrap(true).hexpand(true).css_classes(["heading"]).build();
        let sub = gtk::Label::builder().xalign(0.0).wrap(true).css_classes(["caption", "sc-muted"]).build();
        let name_col = gtk::Box::builder().orientation(gtk::Orientation::Vertical).hexpand(true).build();
        name_col.append(&header);
        name_col.append(&sub);

        let open_item = menu_item("Open on phone");
        let dismiss_item = menu_item("Dismiss");
        let mute_item = menu_item("Mute");
        let menu_box = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(2).build();
        menu_box.append(&open_item);
        menu_box.append(&dismiss_item);
        menu_box.append(&mute_item);
        let menu_pop = gtk::Popover::builder().child(&menu_box).width_request(180).build();
        let menu = gtk::MenuButton::builder()
            .icon_name("view-more-symbolic")
            .valign(gtk::Align::Start)
            .css_classes(["flat", "circular"])
            .popover(&menu_pop)
            .build();

        let header_box = gtk::Box::builder().spacing(10).build();
        header_box.append(icon.widget());
        header_box.append(&name_col);
        header_box.append(&menu);

        let body = gtk::Label::builder()
            .xalign(0.0)
            .wrap(true)
            .wrap_mode(gtk::pango::WrapMode::WordChar)
            .selectable(true)
            .build();
        let entry = gtk::Entry::builder().hexpand(true).placeholder_text("Reply…").build();
        let send = gtk::Button::builder().icon_name("mail-send-symbolic").tooltip_text("Send reply").css_classes(["suggested-action"]).build();
        let reply_box = gtk::Box::builder().spacing(6).margin_top(6).css_classes(["linked"]).build();
        reply_box.append(&entry);
        reply_box.append(&send);
        let column = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(6)
            .css_classes(["sc-notif"])
            .build();
        column.append(&header_box);
        column.append(&body);
        column.append(&reply_box);
        let row = gtk::ListBoxRow::builder().activatable(false).child(&column).build();

        let key = n.key.clone();
        let weak = Rc::downgrade(self);
        let pop = menu_pop.downgrade();
        open_item.connect_clicked(move |_| {
            if let Some(p) = pop.upgrade() {
                p.popdown();
            }
            let Some(ui) = weak.upgrade() else { return };
            let data = ui.state.borrow().rows.get(&key).map(|r| r.data.clone());
            if let Some(n) = data {
                ui.open_on_phone(&n);
            }
        });
        let key = n.key.clone();
        let weak = Rc::downgrade(self);
        let pop = menu_pop.downgrade();
        dismiss_item.connect_clicked(move |_| {
            if let Some(p) = pop.upgrade() {
                p.popdown();
            }
            if let Some(ui) = weak.upgrade() {
                ui.dismiss_notification(&key);
            }
        });
        let key = n.key.clone();
        let weak = Rc::downgrade(self);
        let pop = menu_pop.downgrade();
        mute_item.connect_clicked(move |_| {
            if let Some(p) = pop.upgrade() {
                p.popdown();
            }
            let Some(ui) = weak.upgrade() else { return };
            let data = ui.state.borrow().rows.get(&key).map(|r| r.data.clone());
            if let Some(n) = data.filter(|n| !n.pkg.is_empty()) {
                let muted = ui.state.borrow().muted.as_ref().is_some_and(|m| m.contains(&n.pkg));
                ui.set_app_muted(&n.pkg, !muted);
            }
        });

        let key = n.key.clone();
        let weak = Rc::downgrade(self);
        let entry_ref = entry.downgrade();
        let send_reply = move || {
            let (Some(ui), Some(entry)) = (weak.upgrade(), entry_ref.upgrade()) else { return };
            let text = entry.text().trim().to_owned();
            if text.is_empty() {
                return;
            }
            entry.set_sensitive(false);
            let entry_weak = entry.downgrade();
            ui.command(daemon::reply_line(&key, &text), move |ui, result| {
                if let Some(entry) = entry_weak.upgrade() {
                    entry.set_sensitive(true);
                    if matches!(result, Ok(Reply { ok: true, .. })) {
                        entry.set_text("");
                    }
                }
                ui.toast_reply(&result);
            });
        };
        let send_reply = Rc::new(send_reply);
        let f = send_reply.clone();
        entry.connect_activate(move |_| f());
        send.connect_clicked(move |_| send_reply());

        let mut r = NotifRow { row, icon, header, sub, body, reply_box, entry, send, menu, open_item, mute_item, data: Notification::default() };
        r.update(n, &self.apps.icon_query(&n.pkg, &n.app));
        r.set_muted(false);
        r
    }

    fn open_on_phone(self: &Rc<Self>, n: &Notification) {
        if !self.state.borrow().status.as_ref().is_some_and(|s| s.connected) {
            self.toast("The phone is not connected");
            return;
        }
        self.command(daemon::notif_open_line(&n.key, n.service), |ui, result| ui.toast_reply(&result));
    }

    fn dismiss_notification(self: &Rc<Self>, key: &str) {
        self.state.borrow_mut().dismissed.insert(key.to_owned());
        self.refresh_notifications();
    }

    fn clear_all_notifications(self: &Rc<Self>) {
        {
            let mut st = self.state.borrow_mut();
            let keys: Vec<String> = match &st.status {
                Some(s) => s.notifications.iter().map(|n| n.key.clone()).collect(),
                None => return,
            };
            st.dismissed.extend(keys);
        }
        self.refresh_notifications();
    }

    fn load_muted(self: &Rc<Self>) {
        {
            let mut st = self.state.borrow_mut();
            if st.muted_loading || st.status.is_none() {
                return;
            }
            st.muted_loading = true;
        }
        self.command(daemon::mute_line(None), |ui, result| {
            ui.state.borrow_mut().muted_loading = false;
            let list = result.as_ref().ok().and_then(Reply::muted).unwrap_or_else(|| {
                eprintln!("smartconnect-gui: no muted-app list from smartconnectd");
                Vec::new()
            });
            ui.set_muted_list(list);
        });
    }

    fn app_name(&self, pkg: &str) -> String {
        let known = self.state.borrow().app_names.get(pkg).cloned();
        known.or_else(|| self.apps.app_name(pkg)).unwrap_or_else(|| pkg.to_owned())
    }

    fn set_app_muted(self: &Rc<Self>, pkg: &str, mute: bool) {
        let name = self.app_name(pkg);
        self.command(daemon::mute_line(Some((pkg, mute))), move |ui, result| match &result {
            Ok(reply) if reply.ok => {
                if let Some(list) = reply.muted() {
                    ui.set_muted_list(list);
                }
                ui.toast(&if mute { format!("{name} muted: no more desktop popups from it") } else { format!("{name} unmuted") });
            }
            _ => {
                ui.toast_reply(&result);
                ui.refresh_notifications();
            }
        });
    }

    fn set_muted_list(self: &Rc<Self>, mut list: Vec<String>) {
        list.sort();
        if self.state.borrow().muted.as_ref() == Some(&list) {
            self.refresh_notifications();
            return;
        }
        let old_rows = std::mem::take(&mut self.state.borrow_mut().muted_rows);
        for row in old_rows {
            self.notifs.muted.remove(&row);
        }
        let mut rows = Vec::new();
        for pkg in &list {
            let name = self.app_name(pkg);
            let row = adw::ActionRow::builder().title(&name).use_markup(false).build();
            if name != *pkg {
                row.set_subtitle(pkg);
            }
            let unmute = gtk::Button::builder().label("Unmute").css_classes(["sc-btn-secondary"]).valign(gtk::Align::Center).build();
            let weak = Rc::downgrade(self);
            let pkg = pkg.clone();
            unmute.connect_clicked(move |b| {
                b.set_sensitive(false);
                if let Some(ui) = weak.upgrade() {
                    ui.set_app_muted(&pkg, false);
                }
            });
            row.add_suffix(&unmute);
            self.notifs.muted.add_row(&row);
            rows.push(row);
        }
        self.notifs.muted.set_subtitle(&match list.len() {
            1 => "1 app shows no desktop popups".to_owned(),
            n => format!("{n} apps show no desktop popups"),
        });
        self.notifs.muted_group.set_visible(!list.is_empty());
        {
            let mut st = self.state.borrow_mut();
            st.muted_rows = rows;
            st.muted = Some(list);
        }
        self.refresh_notifications();
    }

    // ── Pair dialog ─────────────────────────────────────────────────────────

    fn open_pair_dialog(self: &Rc<Self>) {
        self.pair.dialog.present(Some(&self.window));
        self.request_pairing();
    }

    fn request_pairing(self: &Rc<Self>) {
        self.pair.button.set_sensitive(false);
        self.command("pair".into(), |ui, result| {
            ui.pair.button.set_sensitive(true);
            if let Ok(reply) = &result {
                if reply.ok {
                    let mut st = ui.state.borrow_mut();
                    st.show_qr = true;
                    st.qr_path = reply
                        .raw
                        .get("qr")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_owned)
                        .or_else(|| st.status.as_ref().and_then(|s| s.pairing_qr.clone()));
                    drop(st);
                    ui.pair.qr.set_filename(None::<&std::path::Path>);
                } else {
                    ui.toast_reply(&result);
                }
            } else {
                ui.toast_reply(&result);
            }
            ui.refresh_pair();
        });
    }

    fn refresh_pair(&self) {
        let st = self.state.borrow();
        let path = st.qr_path.as_deref().filter(|_| st.show_qr).map(|p| p.split('?').next().unwrap_or(p));
        match path {
            Some(path) => {
                let current = self.pair.qr.file().and_then(|f| f.path());
                if current.as_deref() != Some(std::path::Path::new(path)) {
                    self.pair.qr.set_filename(Some(path));
                }
                self.pair.qr_box.set_visible(true);
                self.pair.spinner.set_visible(true);
                self.pair.status.set_label("Waiting for phone… the phone connects automatically once pairing finishes.");
            }
            None => {
                self.pair.qr_box.set_visible(false);
                self.pair.spinner.set_visible(true);
                self.pair.status.set_label("Requesting a pairing code…");
            }
        }
    }
}

impl NotifRow {
    fn update(&mut self, n: &Notification, icon: &Query) {
        self.icon.set(icon);
        let (header, sub) = match (n.title.is_empty(), n.app.is_empty()) {
            (false, _) => (n.title.clone(), n.app.clone()),
            (true, false) => (n.app.clone(), String::new()),
            (true, true) => (String::new(), String::new()),
        };
        self.header.set_label(&header);
        self.sub.set_label(&sub);
        self.sub.set_visible(!sub.is_empty());
        self.body.set_label(&n.body);
        self.body.set_visible(!n.body.is_empty());
        self.reply_box.set_visible(n.replyable);
        self.mute_item.set_visible(!n.pkg.is_empty());
        self.data = n.clone();
    }

    fn set_muted(&self, muted: bool) {
        let app = if self.data.app.is_empty() { "this app" } else { self.data.app.as_str() };
        if let Some(label) = self.mute_item.child().and_downcast::<gtk::Label>() {
            label.set_label(&if muted { format!("Unmute {app}") } else { format!("Mute {app}") });
        }
        self.menu.set_tooltip_text(Some(if muted { "Muted · more actions" } else { "More actions" }));
    }
}
