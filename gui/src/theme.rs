//! Apple Pro Dark Monochrome design system (palette, typography helpers, and GTK4 CSS).

use std::path::{Path, PathBuf};

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

window,
.sc-canvas {
  background-color: #050506;
  color: #F4F4F6;
}

headerbar {
  background-color: #050506;
  background-image: none;
  box-shadow: none;
}

.sc-header {
  background-color: #050506;
  border-bottom: 1px solid rgba(255, 255, 255, 0.07);
  min-height: 56px;
}

.sc-sidebar {
  background-color: #0A0A0C;
  border-right: 1px solid rgba(255, 255, 255, 0.07);
}

.sc-sidebar-footer {
  background-color: #08080A;
  border-top: 1px solid rgba(255, 255, 255, 0.07);
}

.sc-bento {
  background-color: #0E0E11;
  border: 1px solid rgba(255, 255, 255, 0.07);
  border-radius: 14px;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.08);
}

button.sc-bento-btn {
  background-color: #0E0E11;
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.07);
  border-radius: 14px;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.08);
  padding: 0;
  transition: background-color 150ms ease, border-color 150ms ease;
}

button.sc-bento-btn:hover {
  background-color: #141418;
  border-color: rgba(255, 255, 255, 0.20);
}

.sc-stat-tile {
  background-color: #131317;
  border: 1px solid rgba(255, 255, 255, 0.06);
  border-radius: 12px;
  padding: 14px;
}

.sc-icon-box {
  background-color: #16161B;
  border: 1px solid rgba(255, 255, 255, 0.10);
  border-radius: 12px;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.08);
  color: #FFFFFF;
}

.sc-icon-box-sm {
  background-color: #1C1C22;
  border: 1px solid rgba(255, 255, 255, 0.10);
  border-radius: 9px;
  color: #F4F4F6;
}

.sc-icon-box-white {
  background-color: #FFFFFF;
  color: #050506;
  border-radius: 11px;
  box-shadow: 0 0 20px rgba(255, 255, 255, 0.18);
}

.sc-device-switcher {
  background-color: #121216;
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 12px;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.06);
  padding: 10px 12px;
  transition: background-color 150ms ease, border-color 150ms ease;
}

.sc-device-switcher:hover {
  background-color: #18181E;
  border-color: rgba(255, 255, 255, 0.14);
}

button.sc-nav-item {
  background-color: transparent;
  background-image: none;
  border: 1px solid transparent;
  border-radius: 10px;
  padding: 9px 11px;
  color: #A1A1AA;
  box-shadow: none;
  transition: background-color 150ms ease, color 150ms ease;
}

button.sc-nav-item:hover {
  background-color: #141418;
  color: #FFFFFF;
}

button.sc-nav-item:disabled {
  opacity: 0.45;
}

button.sc-nav-item .sc-nav-title {
  font-size: 13px;
  font-weight: 500;
}

button.sc-nav-item .sc-nav-sub {
  font-size: 11px;
  color: #71717A;
}

button.sc-nav-item .sc-nav-shortcut {
  font-family: "JetBrains Mono", "SF Mono", monospace;
  font-size: 10px;
  color: #52525B;
}

button.sc-nav-item .sc-nav-badge {
  background-color: #1F1F24;
  color: #A1A1AA;
  border: 1px solid rgba(255, 255, 255, 0.06);
  border-radius: 5px;
  padding: 1px 6px;
  font-family: "JetBrains Mono", "SF Mono", monospace;
  font-size: 10px;
  font-weight: 600;
  min-height: 0;
}

button.sc-nav-item.active,
button.sc-nav-item.active:hover {
  background-color: #FFFFFF;
  color: #050506;
  box-shadow: 0 4px 18px rgba(255, 255, 255, 0.14);
}

button.sc-nav-item.active .sc-nav-title {
  font-weight: 600;
  color: #050506;
}

button.sc-nav-item.active .sc-nav-sub {
  color: rgba(5, 5, 6, 0.72);
  font-weight: 500;
}

button.sc-nav-item.active .sc-nav-shortcut {
  color: rgba(5, 5, 6, 0.60);
}

button.sc-nav-item.active .sc-nav-badge {
  background-color: #050506;
  color: #FFFFFF;
  border-color: transparent;
}

button.sc-btn-primary,
button.suggested-action {
  background-color: #FFFFFF;
  background-image: none;
  color: #050506;
  border: 1px solid #FFFFFF;
  border-radius: 9px;
  font-weight: 600;
  padding: 6px 14px;
  box-shadow: 0 0 18px rgba(255, 255, 255, 0.14);
  transition: background-color 150ms ease, border-color 150ms ease, color 150ms ease;
}

button.sc-btn-primary:hover,
button.suggested-action:hover {
  background-color: #E4E4E7;
  border-color: #E4E4E7;
  color: #050506;
}

button.sc-btn-primary:disabled,
button.suggested-action:disabled {
  background-color: #16161A;
  border-color: rgba(255, 255, 255, 0.06);
  color: #52525B;
  box-shadow: none;
  opacity: 1;
}

button.sc-btn-secondary {
  background-color: #16161A;
  background-image: none;
  color: #F4F4F6;
  border: 1px solid rgba(255, 255, 255, 0.10);
  border-radius: 9px;
  font-weight: 500;
  padding: 6px 12px;
  box-shadow: none;
  transition: background-color 150ms ease, border-color 150ms ease, color 150ms ease;
}

button.sc-btn-secondary:hover {
  background-color: #1E1E24;
  border-color: rgba(255, 255, 255, 0.20);
  color: #FFFFFF;
}

button.sc-btn-secondary:disabled {
  opacity: 0.45;
}

button.sc-btn-control {
  background-color: #0E0E11;
  background-image: none;
  color: #A1A1AA;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 8px;
  padding: 5px 10px;
  font-size: 12px;
  box-shadow: none;
  transition: background-color 150ms ease, color 150ms ease, border-color 150ms ease;
}

button.sc-btn-control:hover {
  background-color: #16161A;
  border-color: rgba(255, 255, 255, 0.16);
  color: #FFFFFF;
}

button.sc-btn-icon {
  background-color: #141418;
  background-image: none;
  color: #A1A1AA;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 8px;
  padding: 6px;
  box-shadow: none;
}

button.sc-btn-icon:hover {
  background-color: #1E1E24;
  color: #FFFFFF;
  border-color: rgba(255, 255, 255, 0.18);
}

button.destructive-action {
  background-color: #1E1E24;
  background-image: none;
  color: #FFFFFF;
  border: 1px solid rgba(255, 255, 255, 0.24);
  border-radius: 9px;
  font-weight: 600;
  box-shadow: none;
}

button.destructive-action:hover {
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
  letter-spacing: 1.5px;
  color: #71717A;
}

.sc-mono {
  font-family: "JetBrains Mono", "SF Mono", monospace;
}

.sc-secondary {
  color: #A1A1AA;
}

.sc-muted {
  color: #71717A;
}

.sc-pill {
  background-color: #141418;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 9999px;
  padding: 3px 10px;
  font-family: "JetBrains Mono", "SF Mono", monospace;
  font-size: 10.5px;
  color: #A1A1AA;
}

.sc-tag {
  background-color: #16161A;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 6px;
  padding: 2px 8px;
  font-family: "JetBrains Mono", "SF Mono", monospace;
  font-size: 11px;
  color: #A1A1AA;
}

.sc-tag-active {
  background-color: #FFFFFF;
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
  border: 1px solid rgba(255, 255, 255, 0.18);
  border-radius: 9px;
  color: #FFFFFF;
}

.sc-inset-entry:focus-within {
  border-color: #FFFFFF;
}

.sc-divider-top {
  border-top: 1px solid rgba(255, 255, 255, 0.07);
}

list.boxed-list {
  background-color: #0E0E11;
  border: 1px solid rgba(255, 255, 255, 0.07);
  border-radius: 14px;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.08);
}

list.boxed-list > row {
  background-color: transparent;
}

list.boxed-list > row:hover {
  background-color: #141418;
}

popover > contents {
  background-color: #141418;
  border: 1px solid rgba(255, 255, 255, 0.12);
  border-radius: 12px;
  box-shadow: 0 20px 50px rgba(0, 0, 0, 0.85);
}

switch {
  background-color: #1E1E24;
  border: 1px solid rgba(255, 255, 255, 0.10);
  color: transparent;
}

switch:checked {
  background-color: #FFFFFF;
  border-color: #FFFFFF;
}

switch > slider {
  background-color: #71717A;
}

switch:checked > slider {
  background-color: #050506;
}

button.sc-pop-active {
  background-color: #FFFFFF;
  background-image: none;
  color: #050506;
  border-radius: 8px;
  font-weight: 600;
}

button.sc-pop-item {
  border-radius: 7px;
  color: #E4E4E7;
}

button.sc-pop-item:hover {
  background-color: rgba(255, 255, 255, 0.07);
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

picture.wallpaper { border-radius: 12px; }
picture.qr { background: white; border-radius: 12px; padding: 14px; }
.sc-card { padding: 18px; border-radius: 14px; }
.sc-mode { padding: 16px 12px; border-radius: 12px; transition: background-color 150ms ease, outline-color 150ms ease; }
.sc-mode:checked { outline: 2px solid #FFFFFF; outline-offset: -2px; }
.sc-tile { padding: 10px 8px; border-radius: 12px; transition: background-color 150ms ease; }
.sc-chip { padding: 6px 12px; border-radius: 9999px; }
.sc-dimmed { opacity: 0.55; }
.sc-badge { padding: 2px 9px; border-radius: 9999px; min-height: 0; font-weight: 600; }
.sc-notif { padding: 14px; }
.sc-device-btn { border-radius: 9999px; padding: 4px 10px; }
.sc-kbd,
button.sc-kbd {
  padding: 2px 7px;
  border-radius: 5px;
  background-color: #16161A;
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.08);
  font-family: "JetBrains Mono", "SF Mono", monospace;
  font-size: 10.5px;
  font-weight: 600;
  color: #A1A1AA;
  min-height: 0;
  box-shadow: none;
}

button.sc-kbd:hover {
  background-color: #1E1E24;
  border-color: rgba(255, 255, 255, 0.22);
  color: #FFFFFF;
}
"#;

pub fn apply_theme() {
    adw::StyleManager::default().set_color_scheme(adw::ColorScheme::ForceDark);
    let provider = gtk::CssProvider::new();
    provider.load_from_string(CSS);
    if let Some(display) = gdk::Display::default() {
        gtk::style_context_add_provider_for_display(&display, &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION + 10);
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
        // Top diamond
        cr.move_to(cx, cy - 5.0);
        cr.line_to(cx + 5.0, cy - 2.2);
        cr.line_to(cx, cy + 0.6);
        cr.line_to(cx - 5.0, cy - 2.2);
        cr.close_path();
        let _ = cr.stroke();
        // Mid chevron
        cr.move_to(cx - 5.0, cy + 0.8);
        cr.line_to(cx, cy + 3.4);
        cr.line_to(cx + 5.0, cy + 0.8);
        let _ = cr.stroke();
        // Bottom chevron
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
