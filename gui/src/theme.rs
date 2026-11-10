//! Apple Pro Dark Monochrome design system (palette, typography, Cairo modern icons, and GTK4 CSS).

use std::cell::Cell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use adw::prelude::*;
use gtk::{cairo, gdk};

pub const CSS: &str = r#"
:root {
  --window-bg-color: #050506;
  --window-fg-color: #F4F4F6;
  --view-bg-color: #050506;
  --view-fg-color: #F4F4F6;
  --sidebar-bg-color: #0A0A0C;
  --sidebar-fg-color: #F4F4F6;
  --sidebar-border-color: rgba(255, 255, 255, 0.07);
  --headerbar-bg-color: #050506;
  --headerbar-fg-color: #F4F4F6;
  --headerbar-border-color: rgba(255, 255, 255, 0.07);
  --card-bg-color: #0E0E11;
  --card-fg-color: #F4F4F6;
  --dialog-bg-color: #0E0E11;
  --dialog-fg-color: #F4F4F6;
  --popover-bg-color: #141418;
  --popover-fg-color: #F4F4F6;
  --accent-bg-color: #FFFFFF;
  --accent-fg-color: #050506;
  --accent-color: #FFFFFF;
  --destructive-bg-color: #1E1E24;
  --destructive-fg-color: #F4F4F6;
  --destructive-color: #F4F4F6;
}

window.sc-root,
.sc-canvas {
  background-color: #050506;
  background-image: none;
  color: #F4F4F6;
}

window.sc-root headerbar.sc-header {
  background-color: #050506;
  background-image: none;
  border-bottom: 1px solid rgba(255, 255, 255, 0.07);
  box-shadow: none;
  min-height: 56px;
  padding-left: 14px;
  padding-right: 14px;
}

.sc-sidebar {
  background-color: #0A0A0C;
  background-image: none;
  border-right: 1px solid rgba(255, 255, 255, 0.07);
}

.sc-sidebar-footer {
  background-color: #08080A;
  background-image: none;
  border-top: 1px solid rgba(255, 255, 255, 0.07);
}

.sc-bento {
  background-color: #0E0E11;
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 14px;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.07);
}

window.sc-root button.sc-bento-btn {
  background-color: #0E0E11;
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 14px;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.07);
  padding: 0;
  outline: none;
  transition: background-color 150ms ease, border-color 150ms ease;
}

window.sc-root button.sc-bento-btn:hover {
  background-color: #141418;
  border-color: rgba(255, 255, 255, 0.22);
}

.sc-stat-tile {
  background-color: #131317;
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.07);
  border-radius: 12px;
  padding: 15px 16px;
}

.sc-icon-box {
  background-color: #16161B;
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.11);
  border-radius: 12px;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.08);
  color: #FFFFFF;
}

.sc-icon-box-sm {
  background-color: #18181E;
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.10);
  border-radius: 10px;
  color: #F4F4F6;
}

.sc-icon-box-white {
  background-color: #FFFFFF;
  background-image: none;
  color: #050506;
  border-radius: 11px;
  box-shadow: 0 0 20px rgba(255, 255, 255, 0.18);
}

.sc-icon-box-white .sc-vec-icon {
  color: #050506;
}

/* MenuButton wraps an internal GtkButton child — style both directly and defeat user-theme chrome */
window.sc-root menubutton.sc-device-switcher > button {
  background-color: #121216;
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.09);
  border-radius: 12px;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.06);
  padding: 10px 11px;
  color: #F4F4F6;
  outline: none;
  transition: background-color 150ms ease, border-color 150ms ease;
}

window.sc-root menubutton.sc-device-switcher > button:hover,
window.sc-root menubutton.sc-device-switcher > button:checked {
  background-color: #18181E;
  border-color: rgba(255, 255, 255, 0.16);
  color: #FFFFFF;
}

/* Sidebar Navigation Rows */
window.sc-root button.sc-nav-item {
  background-color: transparent;
  background-image: none;
  border: 1px solid transparent;
  border-radius: 10px;
  padding: 10px 12px;
  color: #A1A1AA;
  box-shadow: none;
  outline: none;
  min-height: 0;
  transition: background-color 150ms ease, color 150ms ease, border-color 150ms ease;
}

window.sc-root button.sc-nav-item:hover {
  background-color: #15151A;
  border-color: rgba(255, 255, 255, 0.06);
  color: #FFFFFF;
}

window.sc-root button.sc-nav-item:disabled {
  opacity: 0.45;
}

window.sc-root button.sc-nav-item .sc-nav-title {
  font-size: 13px;
  font-weight: 500;
  color: #E4E4E7;
}

window.sc-root button.sc-nav-item:hover .sc-nav-title {
  color: #FFFFFF;
}

window.sc-root button.sc-nav-item .sc-nav-sub {
  font-size: 11px;
  color: #8E8E98;
}

window.sc-root button.sc-nav-item .sc-nav-shortcut {
  font-family: "JetBrains Mono", "SF Mono", monospace;
  font-size: 10px;
  color: #7A7A86;
}

window.sc-root button.sc-nav-item .sc-nav-badge {
  background-color: #1F1F26;
  color: #D4D4D8;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 5px;
  padding: 1px 6px;
  font-family: "JetBrains Mono", "SF Mono", monospace;
  font-size: 10px;
  font-weight: 600;
  min-height: 0;
}

window.sc-root button.sc-nav-item .sc-vec-icon {
  color: #A1A1AA;
}

window.sc-root button.sc-nav-item:hover .sc-vec-icon {
  color: #FFFFFF;
}

window.sc-root button.sc-nav-item.active,
window.sc-root button.sc-nav-item.active:hover {
  background-color: #FFFFFF;
  background-image: none;
  border-color: #FFFFFF;
  color: #050506;
  box-shadow: 0 4px 18px rgba(255, 255, 255, 0.14);
}

window.sc-root button.sc-nav-item.active .sc-nav-title,
window.sc-root button.sc-nav-item.active:hover .sc-nav-title {
  font-weight: 600;
  color: #050506;
}

window.sc-root button.sc-nav-item.active .sc-nav-sub,
window.sc-root button.sc-nav-item.active:hover .sc-nav-sub {
  color: rgba(5, 5, 6, 0.68);
  font-weight: 500;
}

window.sc-root button.sc-nav-item.active .sc-nav-shortcut,
window.sc-root button.sc-nav-item.active:hover .sc-nav-shortcut {
  color: rgba(5, 5, 6, 0.55);
}

window.sc-root button.sc-nav-item.active .sc-nav-badge,
window.sc-root button.sc-nav-item.active:hover .sc-nav-badge {
  background-color: #050506;
  color: #FFFFFF;
  border-color: transparent;
}

window.sc-root button.sc-nav-item.active .sc-vec-icon,
window.sc-root button.sc-nav-item.active:hover .sc-vec-icon {
  color: #050506;
}

/* Pure White Primary CTA Buttons */
window.sc-root button.sc-btn-primary,
window.sc-root headerbar button.sc-btn-primary,
window.sc-root button.suggested-action {
  background-color: #FFFFFF;
  background-image: none;
  color: #050506;
  border: 1px solid #FFFFFF;
  border-radius: 9px;
  font-weight: 600;
  padding: 6px 15px;
  min-height: 32px;
  box-shadow: 0 0 18px rgba(255, 255, 255, 0.15);
  outline: none;
  transition: background-color 150ms ease, border-color 150ms ease, color 150ms ease;
}

window.sc-root button.sc-btn-primary label,
window.sc-root headerbar button.sc-btn-primary label,
window.sc-root button.sc-btn-primary .sc-vec-icon {
  color: #050506;
}

window.sc-root button.sc-btn-primary:hover,
window.sc-root headerbar button.sc-btn-primary:hover,
window.sc-root button.suggested-action:hover,
window.sc-root button.sc-btn-primary:active,
window.sc-root headerbar button.sc-btn-primary:active {
  background-color: #E4E4E7;
  border-color: #E4E4E7;
  color: #050506;
}

window.sc-root button.sc-btn-primary:disabled,
window.sc-root headerbar button.sc-btn-primary:disabled,
window.sc-root button.suggested-action:disabled {
  background-color: #16161A;
  border-color: rgba(255, 255, 255, 0.06);
  color: #52525B;
  box-shadow: none;
  opacity: 1;
}

window.sc-root button.sc-btn-primary:disabled label {
  color: #52525B;
}

/* Secondary & Utility Buttons */
window.sc-root button.sc-btn-secondary,
window.sc-root headerbar button.sc-btn-secondary {
  background-color: #16161B;
  background-image: none;
  color: #F4F4F6;
  border: 1px solid rgba(255, 255, 255, 0.11);
  border-radius: 9px;
  font-weight: 500;
  padding: 6px 13px;
  min-height: 32px;
  box-shadow: none;
  outline: none;
  transition: background-color 150ms ease, border-color 150ms ease, color 150ms ease;
}

window.sc-root button.sc-btn-secondary:hover,
window.sc-root headerbar button.sc-btn-secondary:hover {
  background-color: #202027;
  border-color: rgba(255, 255, 255, 0.24);
  color: #FFFFFF;
}

window.sc-root button.sc-btn-secondary:disabled {
  opacity: 0.45;
}

window.sc-root button.sc-btn-control,
window.sc-root headerbar button.sc-btn-control {
  background-color: #0E0E11;
  background-image: none;
  color: #B4B4BC;
  border: 1px solid rgba(255, 255, 255, 0.09);
  border-radius: 8px;
  padding: 5px 10px;
  min-height: 30px;
  font-size: 12px;
  box-shadow: none;
  outline: none;
  transition: background-color 150ms ease, color 150ms ease, border-color 150ms ease;
}

window.sc-root button.sc-btn-control:hover,
window.sc-root headerbar button.sc-btn-control:hover {
  background-color: #18181E;
  border-color: rgba(255, 255, 255, 0.18);
  color: #FFFFFF;
}

window.sc-root button.sc-btn-icon,
window.sc-root headerbar button.sc-btn-icon,
window.sc-root menubutton.sc-btn-icon > button {
  background-color: #121216;
  background-image: none;
  color: #B4B4BC;
  border: 1px solid rgba(255, 255, 255, 0.09);
  border-radius: 8px;
  padding: 6px;
  min-width: 32px;
  min-height: 32px;
  box-shadow: none;
  outline: none;
  transition: background-color 150ms ease, color 150ms ease, border-color 150ms ease;
}

window.sc-root button.sc-btn-icon:hover,
window.sc-root button.sc-btn-icon:active,
window.sc-root headerbar button.sc-btn-icon:hover,
window.sc-root menubutton.sc-btn-icon > button:hover,
window.sc-root menubutton.sc-btn-icon > button:checked {
  background-color: #1C1C22;
  color: #FFFFFF;
  border-color: rgba(255, 255, 255, 0.20);
}

window.sc-root button.sc-btn-ghost {
  background-color: transparent;
  background-image: none;
  color: #8E8E98;
  border: 1px solid transparent;
  border-radius: 7px;
  padding: 5px;
  min-width: 28px;
  min-height: 28px;
  box-shadow: none;
  outline: none;
  transition: background-color 150ms ease, color 150ms ease, border-color 150ms ease;
}

window.sc-root button.sc-btn-ghost:hover {
  background-color: #16161B;
  border-color: rgba(255, 255, 255, 0.10);
  color: #FFFFFF;
}

window.sc-root button.destructive-action {
  background-color: #1E1E24;
  background-image: none;
  color: #FFFFFF;
  border: 1px solid rgba(255, 255, 255, 0.26);
  border-radius: 9px;
  font-weight: 600;
  box-shadow: none;
}

window.sc-root button.destructive-action:hover {
  background-color: #2A2A32;
  border-color: #FFFFFF;
}

.sc-hero-title {
  font-size: 26px;
  font-weight: 600;
  letter-spacing: -0.6px;
  color: #FFFFFF;
}

.sc-view-title {
  font-size: 22px;
  font-weight: 600;
  letter-spacing: -0.4px;
  color: #FFFFFF;
}

.sc-metric {
  font-family: "JetBrains Mono", "SF Mono", monospace;
  font-size: 21px;
  font-weight: 600;
  color: #FFFFFF;
}

.sc-metric-text {
  font-size: 21px;
  font-weight: 600;
  color: #FFFFFF;
}

.sc-overline {
  font-family: "JetBrains Mono", "SF Mono", monospace;
  font-size: 10.5px;
  font-weight: 600;
  letter-spacing: 1.4px;
  color: #8E8E98;
}

.sc-mono {
  font-family: "JetBrains Mono", "SF Mono", monospace;
}

.sc-primary {
  color: #F4F4F6;
}

.sc-secondary {
  color: #A1A1AA;
}

.sc-muted {
  color: #8E8E98;
}

.sc-pill {
  background-color: #141418;
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.09);
  border-radius: 9999px;
  padding: 3px 10px;
  font-family: "JetBrains Mono", "SF Mono", monospace;
  font-size: 10.5px;
  color: #B4B4BC;
}

.sc-tag {
  background-color: #16161B;
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.09);
  border-radius: 6px;
  padding: 2px 8px;
  font-family: "JetBrains Mono", "SF Mono", monospace;
  font-size: 11px;
  color: #A1A1AA;
}

.sc-tag-active {
  background-color: #FFFFFF;
  background-image: none;
  border: 1px solid #FFFFFF;
  border-radius: 6px;
  padding: 2px 8px;
  font-family: "JetBrains Mono", "SF Mono", monospace;
  font-size: 11px;
  font-weight: 600;
  color: #050506;
}

.sc-dot-online {
  background-color: #FFFFFF;
  border-radius: 9999px;
  box-shadow: 0 0 8px rgba(255, 255, 255, 0.85);
}

.sc-dot-offline {
  background-color: #52525B;
  border-radius: 9999px;
  box-shadow: none;
}

.sc-dot-ring {
  border: 2px solid #121216;
}

.sc-seg-on {
  background-color: #FFFFFF;
  border-radius: 2px;
}

.sc-seg-off {
  background-color: rgba(255, 255, 255, 0.14);
  border-radius: 2px;
}

.sc-inset-entry {
  background-color: #08080A;
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.18);
  border-radius: 9px;
  color: #FFFFFF;
}

.sc-inset-entry:focus-within {
  border-color: #FFFFFF;
}

.sc-divider-top {
  border-top: 1px solid rgba(255, 255, 255, 0.08);
}

list.boxed-list {
  background-color: #0E0E11;
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 14px;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.07);
}

list.boxed-list > row {
  background-color: transparent;
}

list.boxed-list > row:hover {
  background-color: #141418;
}

popover > contents {
  background-color: #141418;
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.12);
  border-radius: 12px;
  box-shadow: 0 20px 50px rgba(0, 0, 0, 0.85);
}

switch {
  background-color: #1E1E24;
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.10);
  color: transparent;
}

switch:checked {
  background-color: #FFFFFF;
  border-color: #FFFFFF;
}

switch > slider {
  background-color: #71717A;
  background-image: none;
}

switch:checked > slider {
  background-color: #050506;
}

window.sc-root button.sc-pop-active {
  background-color: #FFFFFF;
  background-image: none;
  color: #050506;
  border-radius: 8px;
  border: none;
  box-shadow: none;
  font-weight: 600;
}

window.sc-root button.sc-pop-item {
  background-color: transparent;
  background-image: none;
  border: none;
  box-shadow: none;
  border-radius: 7px;
  color: #E4E4E7;
}

window.sc-root button.sc-pop-item:hover {
  background-color: rgba(255, 255, 255, 0.08);
  color: #FFFFFF;
}

.sc-cmd-row {
  border-radius: 10px;
  padding: 6px 10px;
}

.sc-cmd-list row:selected,
.sc-cmd-list row:hover {
  background-color: rgba(255, 255, 255, 0.08);
  border-radius: 10px;
}

picture.wallpaper { border-radius: 11px; }
picture.qr { background: white; border-radius: 12px; padding: 14px; }
.sc-card { padding: 18px; border-radius: 14px; }
.sc-mode { padding: 16px 12px; border-radius: 12px; transition: background-color 150ms ease, outline-color 150ms ease; }
.sc-mode:checked { outline: 2px solid #FFFFFF; outline-offset: -2px; }
.sc-tile { padding: 10px 8px; border-radius: 12px; transition: background-color 150ms ease; }
.sc-chip { padding: 6px 12px; border-radius: 9999px; }
.sc-dimmed { opacity: 0.72; }
.sc-badge { padding: 2px 9px; border-radius: 9999px; min-height: 0; font-weight: 600; }
.sc-notif { padding: 14px; }
.sc-device-btn { border-radius: 9999px; padding: 4px 10px; }

.sc-kbd,
button.sc-kbd {
  padding: 2px 7px;
  border-radius: 5px;
  background-color: #16161B;
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.09);
  font-family: "JetBrains Mono", "SF Mono", monospace;
  font-size: 10.5px;
  font-weight: 600;
  color: #B4B4BC;
  min-height: 0;
  box-shadow: none;
}

button.sc-kbd:hover {
  background-color: #1E1E24;
  border-color: rgba(255, 255, 255, 0.24);
  color: #FFFFFF;
}
"#;

pub fn apply_theme() {
    adw::StyleManager::default().set_color_scheme(adw::ColorScheme::ForceDark);
    let provider = gtk::CssProvider::new();
    provider.load_from_string(CSS);
    if let Some(display) = gdk::Display::default() {
        // Priority above GTK_STYLE_PROVIDER_PRIORITY_USER (800) so custom desktop themes
        // in ~/.config/gtk-4.0/gtk.css cannot override our monochrome button/surface system.
        gtk::style_context_add_provider_for_display(&display, &provider, gtk::STYLE_PROVIDER_PRIORITY_USER + 50);
    }
}

/// Formats `path` cleanly for UI display, collapsing a leading `$HOME` into `~`.
pub fn format_display_path(path: &Path) -> String {
    if let Some(home) = std::env::var_os("HOME").map(PathBuf::from).filter(|h| !h.as_os_str().is_empty()) {
        if let Ok(rel) = path.strip_prefix(&home) {
            return if rel.as_os_str().is_empty() {
                "~".to_owned()
            } else {
                format!("~/{}", rel.display())
            };
        }
    }
    path.display().to_string()
}

/// Uppercase tracked monospace overline (`WORKSPACE`, `BATTERY`, etc.).
pub fn overline(text: &str) -> gtk::Label {
    gtk::Label::builder()
        .label(text.to_uppercase())
        .xalign(0.0)
        .css_classes(["sc-overline"])
        .build()
}

/// Modern geometric vector icon kinds inspired by Apple SF Symbols / Lucide stroke linework.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconKind {
    Smartphone,
    AppsGrid,
    Monitor,
    Folder,
    Bell,
    Battery { pct: Option<u8>, charging: bool },
    Wifi { bars: u8, on: bool },
    Cellular { bars: u8 },
    Bluetooth { on: bool },
    Mouse,
    HardDrive,
    Edit,
    Search,
    Activity,
    ChevronRight,
    ChevronDown,
    MoreDots,
    MenuSidebar,
    Check,
    Close,
    Plus,
    Settings,
    WifiOff,
    #[allow(dead_code)] // Constructed in Apps Launchpad (Phase 2).
    Refresh,
    #[allow(dead_code)]
    ListRows,
    #[allow(dead_code)]
    Sparkles,
}

/// A sharp, resolution-independent modern stroke icon that dynamically follows CSS `color`.
#[derive(Clone)]
pub struct ModernIcon {
    area: gtk::DrawingArea,
    kind: Rc<Cell<IconKind>>,
}

impl ModernIcon {
    pub fn new(kind: IconKind, size: i32) -> Self {
        let state = Rc::new(Cell::new(kind));
        let area = gtk::DrawingArea::builder()
            .content_width(size)
            .content_height(size)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .css_classes(["sc-vec-icon"])
            .build();
        let state_ref = state.clone();
        area.set_draw_func(move |w_ref, cr, w, h| {
            let color = w_ref.color();
            cr.save().ok();
            cr.scale(f64::from(w) / 24.0, f64::from(h) / 24.0);
            cr.set_line_cap(cairo::LineCap::Round);
            cr.set_line_join(cairo::LineJoin::Round);
            draw_icon(
                cr,
                state_ref.get(),
                (f64::from(color.red()), f64::from(color.green()), f64::from(color.blue()), f64::from(color.alpha())),
            );
            cr.restore().ok();
        });
        Self { area, kind: state }
    }

    pub fn widget(&self) -> &gtk::DrawingArea {
        &self.area
    }

    pub fn set_kind(&self, kind: IconKind) {
        if self.kind.get() != kind {
            self.kind.set(kind);
            self.area.queue_draw();
        }
    }

    pub fn add_css_class(&self, cls: &str) {
        self.area.add_css_class(cls);
    }

    pub fn queue_draw(&self) {
        self.area.queue_draw();
    }
}

fn rounded_rect(cr: &cairo::Context, x: f64, y: f64, w: f64, h: f64, r: f64) {
    let r = r.min(w / 2.0).min(h / 2.0);
    cr.new_sub_path();
    cr.arc(x + w - r, y + r, r, -std::f64::consts::FRAC_PI_2, 0.0);
    cr.arc(x + w - r, y + h - r, r, 0.0, std::f64::consts::FRAC_PI_2);
    cr.arc(x + r, y + h - r, r, std::f64::consts::FRAC_PI_2, std::f64::consts::PI);
    cr.arc(x + r, y + r, r, std::f64::consts::PI, 1.5 * std::f64::consts::PI);
    cr.close_path();
}

fn draw_icon(cr: &cairo::Context, kind: IconKind, rgba: (f64, f64, f64, f64)) {
    let (r, g, b, a) = rgba;
    cr.set_source_rgba(r, g, b, a);
    cr.set_line_width(1.85);

    match kind {
        IconKind::Smartphone => {
            rounded_rect(cr, 6.2, 2.5, 11.6, 19.0, 2.3);
            let _ = cr.stroke();
            cr.move_to(10.4, 5.2);
            cr.line_to(13.6, 5.2);
            let _ = cr.stroke();
            cr.move_to(9.8, 18.6);
            cr.line_to(14.2, 18.6);
            let _ = cr.stroke();
        }
        IconKind::AppsGrid => {
            for (x, y) in [(3.5, 3.5), (13.2, 3.5), (3.5, 13.2), (13.2, 13.2)] {
                rounded_rect(cr, x, y, 7.3, 7.3, 1.8);
                let _ = cr.stroke();
            }
        }
        IconKind::Monitor => {
            rounded_rect(cr, 2.5, 3.5, 19.0, 13.0, 2.2);
            let _ = cr.stroke();
            cr.move_to(12.0, 16.5);
            cr.line_to(12.0, 20.5);
            let _ = cr.stroke();
            cr.move_to(8.0, 20.5);
            cr.line_to(16.0, 20.5);
            let _ = cr.stroke();
        }
        IconKind::Folder => {
            cr.move_to(2.8, 6.8);
            cr.curve_to(2.8, 5.4, 3.6, 4.5, 5.0, 4.5);
            cr.line_to(9.0, 4.5);
            cr.line_to(11.0, 7.0);
            cr.line_to(19.0, 7.0);
            cr.curve_to(20.4, 7.0, 21.2, 7.9, 21.2, 9.2);
            cr.line_to(21.2, 17.5);
            cr.curve_to(21.2, 18.8, 20.3, 19.6, 19.0, 19.6);
            cr.line_to(5.0, 19.6);
            cr.curve_to(3.6, 19.6, 2.8, 18.8, 2.8, 17.5);
            cr.close_path();
            let _ = cr.stroke();
        }
        IconKind::Bell => {
            cr.move_to(18.0, 14.5);
            cr.curve_to(18.0, 9.5, 16.2, 4.0, 12.0, 4.0);
            cr.curve_to(7.8, 4.0, 6.0, 9.5, 6.0, 14.5);
            cr.curve_to(6.0, 16.3, 4.5, 17.2, 3.8, 17.6);
            cr.line_to(20.2, 17.6);
            cr.curve_to(19.5, 17.2, 18.0, 16.3, 18.0, 14.5);
            let _ = cr.stroke();
            cr.arc(12.0, 18.2, 2.2, 0.2, std::f64::consts::PI - 0.2);
            let _ = cr.stroke();
        }
        IconKind::Battery { pct, charging } => {
            rounded_rect(cr, 2.0, 6.5, 17.2, 11.0, 2.4);
            let _ = cr.stroke();
            cr.move_to(21.5, 10.2);
            cr.line_to(21.5, 13.8);
            let _ = cr.stroke();
            if charging {
                cr.set_line_width(1.65);
                cr.move_to(11.6, 8.3);
                cr.line_to(8.8, 12.2);
                cr.line_to(11.4, 12.2);
                cr.line_to(9.8, 15.7);
                cr.line_to(12.6, 11.8);
                cr.line_to(10.0, 11.8);
                cr.close_path();
                let _ = cr.fill();
            } else if let Some(p) = pct {
                let frac = f64::from(p.min(100)) / 100.0;
                let fill_w = (12.6 * frac).max(1.8);
                rounded_rect(cr, 4.3, 8.8, fill_w, 6.4, 1.0);
                let _ = cr.fill();
            }
        }
        IconKind::Wifi { bars, on } => {
            let active_a = if on { a } else { a * 0.35 };
            let dim_a = a * 0.25;
            cr.set_source_rgba(r, g, b, if on && bars >= 4 { active_a } else { dim_a });
            cr.arc(12.0, 19.2, 12.5, -2.35, -0.79);
            let _ = cr.stroke();
            cr.set_source_rgba(r, g, b, if on && bars >= 3 { active_a } else { dim_a });
            cr.arc(12.0, 19.2, 8.8, -2.32, -0.82);
            let _ = cr.stroke();
            cr.set_source_rgba(r, g, b, if on && bars >= 1 { active_a } else { dim_a });
            cr.arc(12.0, 19.2, 5.1, -2.26, -0.88);
            let _ = cr.stroke();
            cr.set_source_rgba(r, g, b, active_a);
            cr.arc(12.0, 19.2, 1.35, 0.0, std::f64::consts::TAU);
            let _ = cr.fill();
        }
        IconKind::Cellular { bars } => {
            cr.set_line_width(2.2);
            let heights = [(5.0, 15.8), (9.6, 12.5), (14.2, 9.0), (18.8, 5.2)];
            for (idx, (x, top)) in heights.into_iter().enumerate() {
                cr.set_source_rgba(r, g, b, if (idx as u8) < bars { a } else { a * 0.26 });
                cr.move_to(x, 19.5);
                cr.line_to(x, top);
                let _ = cr.stroke();
            }
        }
        IconKind::Bluetooth { on } => {
            cr.set_source_rgba(r, g, b, if on { a } else { a * 0.40 });
            cr.move_to(7.0, 7.5);
            cr.line_to(17.0, 16.5);
            cr.line_to(12.0, 21.0);
            cr.line_to(12.0, 3.0);
            cr.line_to(17.0, 7.5);
            cr.line_to(7.0, 16.5);
            let _ = cr.stroke();
            if !on {
                cr.move_to(4.0, 4.0);
                cr.line_to(20.0, 20.0);
                let _ = cr.stroke();
            }
        }
        IconKind::Mouse => {
            rounded_rect(cr, 6.5, 2.8, 11.0, 18.4, 5.5);
            let _ = cr.stroke();
            cr.move_to(12.0, 7.2);
            cr.line_to(12.0, 10.6);
            let _ = cr.stroke();
        }
        IconKind::HardDrive => {
            rounded_rect(cr, 2.8, 4.5, 18.4, 15.0, 2.8);
            let _ = cr.stroke();
            cr.move_to(3.0, 14.0);
            cr.line_to(21.0, 14.0);
            let _ = cr.stroke();
            cr.arc(17.2, 16.8, 0.9, 0.0, std::f64::consts::TAU);
            let _ = cr.fill();
            cr.arc(13.8, 16.8, 0.9, 0.0, std::f64::consts::TAU);
            let _ = cr.fill();
        }
        IconKind::Edit => {
            // Crisp SquarePen vector
            cr.move_to(12.2, 4.2);
            cr.line_to(5.2, 4.2);
            cr.curve_to(4.1, 4.2, 3.5, 4.8, 3.5, 6.0);
            cr.line_to(3.5, 18.8);
            cr.curve_to(3.5, 19.9, 4.1, 20.5, 5.2, 20.5);
            cr.line_to(18.0, 20.5);
            cr.curve_to(19.1, 20.5, 19.8, 19.9, 19.8, 18.8);
            cr.line_to(19.8, 11.8);
            let _ = cr.stroke();
            cr.move_to(18.5, 3.8);
            cr.line_to(20.2, 5.5);
            cr.line_to(11.4, 14.3);
            cr.line_to(9.2, 14.8);
            cr.line_to(9.7, 12.6);
            cr.close_path();
            let _ = cr.stroke();
        }
        IconKind::Search => {
            cr.arc(10.8, 10.8, 6.4, 0.0, std::f64::consts::TAU);
            let _ = cr.stroke();
            cr.move_to(15.5, 15.5);
            cr.line_to(20.2, 20.2);
            let _ = cr.stroke();
        }
        IconKind::Activity => {
            cr.move_to(2.4, 12.0);
            cr.line_to(6.2, 12.0);
            cr.line_to(9.2, 4.5);
            cr.line_to(14.8, 19.5);
            cr.line_to(17.8, 12.0);
            cr.line_to(21.6, 12.0);
            let _ = cr.stroke();
        }
        IconKind::ChevronRight => {
            cr.move_to(9.2, 5.5);
            cr.line_to(15.6, 12.0);
            cr.line_to(9.2, 18.5);
            let _ = cr.stroke();
        }
        IconKind::ChevronDown => {
            cr.move_to(6.0, 9.2);
            cr.line_to(12.0, 15.2);
            cr.line_to(18.0, 9.2);
            let _ = cr.stroke();
        }
        IconKind::MoreDots => {
            for x in [6.0, 12.0, 18.0] {
                cr.arc(x, 12.0, 1.35, 0.0, std::f64::consts::TAU);
                let _ = cr.fill();
            }
        }
        IconKind::MenuSidebar => {
            rounded_rect(cr, 3.0, 4.0, 18.0, 16.0, 2.4);
            let _ = cr.stroke();
            cr.move_to(9.5, 4.2);
            cr.line_to(9.5, 19.8);
            let _ = cr.stroke();
        }
        IconKind::Check => {
            cr.move_to(5.0, 12.5);
            cr.line_to(9.8, 17.2);
            cr.line_to(19.0, 7.2);
            let _ = cr.stroke();
        }
        IconKind::Close => {
            cr.move_to(6.0, 6.0);
            cr.line_to(18.0, 18.0);
            let _ = cr.stroke();
            cr.move_to(18.0, 6.0);
            cr.line_to(6.0, 18.0);
            let _ = cr.stroke();
        }
        IconKind::Plus => {
            cr.move_to(12.0, 5.0);
            cr.line_to(12.0, 19.0);
            let _ = cr.stroke();
            cr.move_to(5.0, 12.0);
            cr.line_to(19.0, 12.0);
            let _ = cr.stroke();
        }
        IconKind::Settings => {
            cr.arc(12.0, 12.0, 3.0, 0.0, std::f64::consts::TAU);
            let _ = cr.stroke();
            cr.arc(12.0, 12.0, 7.2, 0.0, std::f64::consts::TAU);
            let _ = cr.stroke();
            for i in 0..4 {
                let ang = f64::from(i) * std::f64::consts::FRAC_PI_4;
                let (s, c) = ang.sin_cos();
                cr.move_to(12.0 + c * 7.2, 12.0 + s * 7.2);
                cr.line_to(12.0 + c * 9.6, 12.0 + s * 9.6);
                let _ = cr.stroke();
                cr.move_to(12.0 - c * 7.2, 12.0 - s * 7.2);
                cr.line_to(12.0 - c * 9.6, 12.0 - s * 9.6);
                let _ = cr.stroke();
            }
        }
        IconKind::WifiOff => {
            cr.set_source_rgba(r, g, b, a * 0.45);
            cr.arc(12.0, 19.2, 11.5, -2.35, -0.79);
            let _ = cr.stroke();
            cr.arc(12.0, 19.2, 6.8, -2.30, -0.84);
            let _ = cr.stroke();
            cr.set_source_rgba(r, g, b, a);
            cr.move_to(3.8, 3.8);
            cr.line_to(20.2, 20.2);
            let _ = cr.stroke();
        }
        IconKind::Refresh => {
            cr.arc(12.0, 12.0, 7.5, -0.4, 4.7);
            let _ = cr.stroke();
            cr.move_to(15.2, 4.2);
            cr.line_to(19.2, 4.5);
            cr.line_to(18.6, 8.6);
            let _ = cr.stroke();
        }
        IconKind::ListRows => {
            for y in [6.5, 12.0, 17.5] {
                cr.arc(4.2, y, 1.1, 0.0, std::f64::consts::TAU);
                let _ = cr.fill();
                cr.move_to(8.0, y);
                cr.line_to(20.5, y);
                let _ = cr.stroke();
            }
        }
        IconKind::Sparkles => {
            cr.move_to(12.0, 3.0);
            cr.curve_to(12.4, 9.4, 14.6, 11.6, 21.0, 12.0);
            cr.curve_to(14.6, 12.4, 12.4, 14.6, 12.0, 21.0);
            cr.curve_to(11.6, 14.6, 9.4, 12.4, 3.0, 12.0);
            cr.curve_to(9.4, 11.6, 11.6, 9.4, 12.0, 3.0);
            let _ = cr.stroke();
        }
    }
}

/// 20×20 crisp Apple Pro monochrome stacked-layer brand badge.
pub fn brand_mark() -> gtk::DrawingArea {
    let area = gtk::DrawingArea::builder()
        .content_width(20)
        .content_height(20)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .build();
    area.set_draw_func(|_, cr, w, h| {
        let (w, h) = (f64::from(w), f64::from(h));
        let r = 6.0_f64;
        cr.new_sub_path();
        cr.arc(w - r, r, r, -std::f64::consts::FRAC_PI_2, 0.0);
        cr.arc(w - r, h - r, r, 0.0, std::f64::consts::FRAC_PI_2);
        cr.arc(r, h - r, r, std::f64::consts::FRAC_PI_2, std::f64::consts::PI);
        cr.arc(r, r, r, std::f64::consts::PI, 1.5 * std::f64::consts::PI);
        cr.close_path();
        cr.set_source_rgb(1.0, 1.0, 1.0);
        let _ = cr.fill();

        // Three minimalist stacked layers inside in #050506.
        cr.set_source_rgb(0.02, 0.02, 0.024);
        cr.set_line_width(1.5);
        cr.set_line_cap(cairo::LineCap::Round);
        cr.set_line_join(cairo::LineJoin::Round);

        let cx = w / 2.0;
        let cy = h / 2.0;
        cr.move_to(cx, cy - 5.0);
        cr.line_to(cx + 5.0, cy - 2.2);
        cr.line_to(cx, cy + 0.6);
        cr.line_to(cx - 5.0, cy - 2.2);
        cr.close_path();
        let _ = cr.stroke();

        cr.move_to(cx - 5.0, cy + 0.8);
        cr.line_to(cx, cy + 3.4);
        cr.line_to(cx + 5.0, cy + 0.8);
        let _ = cr.stroke();

        cr.move_to(cx - 5.0, cy + 3.4);
        cr.line_to(cx, cy + 5.8);
        cr.line_to(cx + 5.0, cy + 3.4);
        let _ = cr.stroke();
    });
    area
}

/// Circular status dot styled via `.sc-dot-online` / `.sc-dot-offline`.
pub fn status_dot(size: i32, with_ring: bool) -> gtk::Box {
    let mut classes = vec!["sc-dot-offline"];
    if with_ring {
        classes.push("sc-dot-ring");
    }
    gtk::Box::builder()
        .width_request(size)
        .height_request(size)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .css_classes(classes)
        .build()
}

pub fn set_dot_online(dot: &gtk::Box, online: bool) {
    if online {
        dot.remove_css_class("sc-dot-offline");
        dot.add_css_class("sc-dot-online");
    } else {
        dot.remove_css_class("sc-dot-online");
        dot.add_css_class("sc-dot-offline");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn css_contains_no_unsupported_web_properties() {
        // GTK4 CSS rejects backdrop-filter and CSS flex `gap`.
        assert!(!CSS.contains("backdrop-filter"));
        assert!(!CSS.contains("\n  gap:"));
    }

    #[test]
    fn format_display_path_collapses_home() {
        let p = Path::new("/tmp/smartconnect-test");
        assert_eq!(format_display_path(p), "/tmp/smartconnect-test");
        if let Some(home) = std::env::var_os("HOME").map(PathBuf::from).filter(|h| !h.as_os_str().is_empty()) {
            assert_eq!(format_display_path(&home), "~");
            assert_eq!(format_display_path(&home.join("Phone")), "~/Phone");
        }
    }
}
