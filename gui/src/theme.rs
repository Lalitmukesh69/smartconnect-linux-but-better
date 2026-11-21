//! Apple Pro Dark Monochrome design system (palette, typography, Cairo modern icons, and GTK4 CSS).

use std::cell::Cell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use adw::prelude::*;
use gtk::{cairo, gdk};

pub const CSS: &str = r#"
:root {
  /* 2026 Dark Palette Tokens */
  --bg: #022B3A;
  --text: #FFFFFF;
  --accent: #1F7A8C;
  --highlight: #BFDBF7;
  --soft: #E1E5F2;
  --danger: #E06C75;

  /* Derived Surface Tokens */
  --surface-1: rgba(255, 255, 255, 0.04);
  --surface-2: rgba(255, 255, 255, 0.07);
  --surface-3: rgba(255, 255, 255, 0.11);
  --border: rgba(255, 255, 255, 0.08);
  --border-control: rgba(225, 229, 242, 0.50);
  --border-subtle: rgba(255, 255, 255, 0.06);
  --text-muted: rgba(225, 229, 242, 0.75);
  --text-subtle: rgba(225, 229, 242, 0.55);
  --accent-tint: rgba(31, 122, 140, 0.18);
  --accent-hover: #1B7082;
  --accent-active: #196A7C;

  /* Base Radii */
  --radius-control: 8px;
  --radius-surface: 12px;
  --radius-pill: 9999px;

  /* Standard Libadwaita / GTK System Mappings */
  --window-bg-color: #022B3A;
  --window-fg-color: #FFFFFF;
  --view-bg-color: #022B3A;
  --view-fg-color: #FFFFFF;
  --headerbar-bg-color: #022B3A;
  --headerbar-fg-color: #FFFFFF;
  --headerbar-border-color: transparent;
  --card-bg-color: rgba(255, 255, 255, 0.04);
  --card-fg-color: #FFFFFF;
  --dialog-bg-color: #0C3342;
  --dialog-fg-color: #FFFFFF;
  --popover-bg-color: #0C3342;
  --popover-fg-color: #FFFFFF;
  --accent-bg-color: #1F7A8C;
  --accent-fg-color: #FFFFFF;
  --accent-color: #1F7A8C;
  --destructive-bg-color: rgba(224, 108, 117, 0.15);
  --destructive-fg-color: #FFFFFF;
  --destructive-color: #E06C75;
}

window.sc-root,
window.sc-root *,
window.sc-root .monospace,
window.sc-root label,
.sc-canvas,
.sc-canvas *,
.sc-dialog,
.sc-dialog *,
popover,
popover *,
dialog,
dialog *,
toast,
toast * {
  font-family: "Helvetica", "Nimbus Sans", "Liberation Sans", sans-serif;
}

window.sc-root,
.sc-canvas {
  background-color: #022B3A;
  background-image: none;
  color: #FFFFFF;
}

window.sc-root button:focus-visible,
window.sc-root entry:focus-visible,
window.sc-root switch:focus-visible,
window.sc-root checkbutton:focus-visible {
  outline: 2px solid #BFDBF7;
  outline-offset: 2px;
}

window.sc-root headerbar.sc-header {
  background-color: #022B3A;
  background-image: none;
  border-bottom: 1px solid transparent;
  box-shadow: none;
  min-height: 56px;
  min-height: 56px;
  padding-left: 16px;
  padding-right: 16px;
}

window.sc-root headerbar.sc-header.sc-header-scrolled {
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
}

.sc-sidebar {
  background-color: #022B3A;
  background-image: none;
  border-right: 1px solid rgba(255, 255, 255, 0.08);
  min-width: 240px;
  min-width: 240px;
  transition: min-width 200ms ease, width 200ms ease;
}

.sc-sidebar.collapsed {
  min-width: 64px;
  min-width: 64px;
}

.sc-sidebar-header {
  background-color: #022B3A;
  background-image: none;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  min-height: 56px;
  min-height: 56px;
  padding-left: 16px;
  padding-right: 16px;
}

.sc-sidebar-footer {
  background-color: #022B3A;
  background-image: none;
  border-top: 1px solid rgba(255, 255, 255, 0.08);
  padding: 12px 16px 16px 16px;
}

window.sc-root button.sc-nav-item {
  background-color: transparent;
  background-image: none;
  border: 1px solid transparent;
  border-radius: 8px;
  padding: 0 12px;
  min-height: 40px;
  min-height: 40px;
  color: rgba(225, 229, 242, 0.75);
  box-shadow: none;
  outline: none;
  transition: background-color 140ms ease, color 140ms ease, border-color 140ms ease;
}

window.sc-root button.sc-nav-item:hover {
  background-color: rgba(255, 255, 255, 0.07);
  color: #FFFFFF;
}

window.sc-root button.sc-nav-item .sc-nav-title {
  font-size: 14px;
  font-weight: 500;
  color: rgba(225, 229, 242, 0.85);
}

window.sc-root button.sc-nav-item:hover .sc-nav-title {
  color: #FFFFFF;
}

window.sc-root button.sc-nav-item .sc-vec-icon {
  color: rgba(225, 229, 242, 0.75);
}

window.sc-root button.sc-nav-item:hover .sc-vec-icon {
  color: #FFFFFF;
}

window.sc-root button.sc-nav-item.active,
window.sc-root button.sc-nav-item.active:hover {
  background-color: rgba(255, 255, 255, 0.11);
  border-color: transparent;
  color: #FFFFFF;
  box-shadow: none;
}

window.sc-root button.sc-nav-item.active .sc-nav-title {
  font-weight: 600;
  color: #FFFFFF;
}

window.sc-root button.sc-nav-item.active .sc-vec-icon {
  color: #BFDBF7;
}

window.sc-root button.sc-nav-item .sc-nav-count {
  font-family: "Helvetica", "Nimbus Sans", "Liberation Sans", sans-serif;
  font-feature-settings: "tnum" 1;
  font-size: 12px;
  font-weight: 500;
  color: rgba(225, 229, 242, 0.75);
}

window.sc-root button.sc-nav-item .sc-nav-shortcut {
  font-family: "Helvetica", "Nimbus Sans", "Liberation Sans", sans-serif;
  font-feature-settings: "tnum" 1;
  font-size: 11px;
  color: rgba(225, 229, 242, 0.55);
  opacity: 0;
  transition: opacity 140ms ease;
}

window.sc-root button.sc-nav-item:hover .sc-nav-shortcut,
window.sc-root button.sc-nav-item:focus-visible .sc-nav-shortcut {
  opacity: 1;
}

.sc-nav-divider {
  min-height: 1px;
  min-height: 1px;
  background-color: rgba(255, 255, 255, 0.08);
  margin-top: 8px;
  margin-bottom: 8px;
  margin-left: 12px;
  margin-right: 12px;
}

window.sc-root button.sc-device-row,
window.sc-root menubutton.sc-device-switcher > button {
  background-color: transparent;
  background-image: none;
  border: 1px solid transparent;
  border-radius: 8px;
  padding: 6px 8px;
  min-height: 48px;
  color: #FFFFFF;
  outline: none;
  transition: background-color 140ms ease;
}

window.sc-root button.sc-device-row:hover,
window.sc-root menubutton.sc-device-switcher > button:hover {
  background-color: rgba(255, 255, 255, 0.07);
}

.sc-device-name {
  font-size: 14px;
  font-weight: 600;
  color: #FFFFFF;
}

.sc-device-status {
  font-size: 12px;
  color: rgba(225, 229, 242, 0.75);
}

window.sc-root button.sc-btn-primary,
window.sc-root button.suggested-action,
.sc-dialog button.sc-btn-primary,
.sc-dialog button.suggested-action {
  background-color: #1F7A8C;
  background-image: none;
  color: #FFFFFF;
  border: 1px solid #1F7A8C;
  border-radius: 8px;
  font-weight: 600;
  font-size: 13px;
  padding: 0 16px;
  min-height: 40px;
  min-height: 40px;
  box-shadow: none;
  outline: none;
  transition: background-color 140ms ease, border-color 140ms ease;
}

window.sc-root button.sc-btn-primary label,
window.sc-root button.suggested-action label {
  color: #FFFFFF;
}

window.sc-root button.sc-btn-primary:hover,
window.sc-root button.suggested-action:hover {
  background-color: #1B7082;
  border-color: #1B7082;
  color: #FFFFFF;
}

window.sc-root button.sc-btn-primary:active,
window.sc-root button.suggested-action:active {
  background-color: #196A7C;
  border-color: #196A7C;
  color: #FFFFFF;
}

window.sc-root button.sc-btn-primary:disabled,
window.sc-root button.suggested-action:disabled {
  opacity: 0.40;
  background-color: #1F7A8C;
  border-color: #1F7A8C;
  color: #FFFFFF;
}

window.sc-root button.sc-btn-secondary,
.sc-dialog button.sc-btn-secondary {
  background-color: rgba(255, 255, 255, 0.07);
  background-image: none;
  color: #FFFFFF;
  border: 1px solid rgba(225, 229, 242, 0.20);
  border-radius: 8px;
  font-weight: 500;
  font-size: 13px;
  padding: 0 16px;
  min-height: 40px;
  min-height: 40px;
  box-shadow: none;
  outline: none;
  transition: background-color 140ms ease, border-color 140ms ease;
}

window.sc-root button.sc-btn-secondary:hover,
.sc-dialog button.sc-btn-secondary:hover {
  background-color: rgba(255, 255, 255, 0.11);
  border-color: rgba(225, 229, 242, 0.35);
  color: #FFFFFF;
}

window.sc-root button.sc-btn-secondary:disabled {
  opacity: 0.40;
}

window.sc-root button.sc-btn-row-action {
  min-width: 88px;
  min-height: 36px;
  min-height: 36px;
  font-size: 13px;
  padding: 0 14px;
}

window.sc-root button.sc-btn-ghost,
.sc-dialog button.sc-btn-ghost {
  background-color: transparent;
  background-image: none;
  color: rgba(225, 229, 242, 0.75);
  border: 1px solid transparent;
  border-radius: 8px;
  padding: 0 12px;
  min-height: 36px;
  min-height: 36px;
  box-shadow: none;
  outline: none;
  transition: background-color 140ms ease, color 140ms ease;
}

window.sc-root button.sc-btn-ghost:hover {
  background-color: rgba(255, 255, 255, 0.07);
  color: #FFFFFF;
}

window.sc-root button.sc-btn-icon,
window.sc-root menubutton.sc-btn-icon > button,
.sc-dialog button.sc-btn-icon {
  background-color: transparent;
  background-image: none;
  color: rgba(225, 229, 242, 0.75);
  border: 1px solid transparent;
  border-radius: 8px;
  padding: 0;
  min-width: 40px;
  min-height: 40px;
  min-width: 40px;
  min-height: 40px;
  box-shadow: none;
  outline: none;
  transition: background-color 140ms ease, color 140ms ease;
}

window.sc-root button.sc-btn-icon:hover,
window.sc-root menubutton.sc-btn-icon > button:hover {
  background-color: rgba(255, 255, 255, 0.07);
  color: #FFFFFF;
}

window.sc-root button.sc-btn-danger,
window.sc-root button.destructive-action {
  background-color: rgba(224, 108, 117, 0.15);
  background-image: none;
  color: #FFFFFF;
  border: 1px solid #E06C75;
  border-radius: 8px;
  font-weight: 600;
  font-size: 13px;
  padding: 0 16px;
  min-height: 40px;
  min-height: 40px;
  outline: none;
  transition: background-color 140ms ease;
}

window.sc-root button.sc-btn-danger:hover,
window.sc-root button.destructive-action:hover {
  background-color: rgba(224, 108, 117, 0.28);
}

.sc-surface {
  background-color: rgba(255, 255, 255, 0.04);
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 12px;
}

box.sc-hairline {
  min-height: 1px;
  background-color: rgba(255, 255, 255, 0.08);
}

progressbar.sc-storage-bar trough {
  min-height: 4px;
  background-color: rgba(255, 255, 255, 0.08);
  border-radius: 999px;
}

progressbar.sc-storage-bar progress {
  background-color: var(--accent);
  border-radius: 999px;
}

.sc-pref-group list.boxed-list {
  background-color: rgba(255, 255, 255, 0.04);
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 12px;
  box-shadow: none;
}

.sc-pref-group list.boxed-list > row {
  min-height: 56px;
  padding: 12px 16px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  background-color: transparent;
  background-image: none;
}

.sc-pref-group list.boxed-list > row:last-child {
  border-bottom: none;
}

.sc-row {
  min-height: 56px;
  padding: 12px 16px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  background-color: transparent;
}

.sc-surface > *:last-child .sc-row,
.sc-surface > .sc-row:last-child {
  border-bottom: none;
}

.sc-row-interactive {
  transition: background-color 140ms ease;
}

.sc-row-interactive:hover {
  background-color: rgba(255, 255, 255, 0.03);
}

button.sc-btn-surface-row {
  background-color: transparent;
  background-image: none;
  border: none;
  border-radius: 0;
  padding: 0;
  outline: none;
  box-shadow: none;
}

button.sc-btn-surface-row:hover .sc-row {
  background-color: rgba(255, 255, 255, 0.03);
}

.sc-page-title {
  font-size: 28px;
  font-weight: 600;
  line-height: 36px;
  letter-spacing: -0.01em;
  color: #FFFFFF;
}

.sc-page-desc {
  font-size: 14px;
  font-weight: 400;
  line-height: 20px;
  color: rgba(225, 229, 242, 0.75);
}

.sc-section-title {
  font-size: 16px;
  font-weight: 600;
  line-height: 24px;
  color: #FFFFFF;
}

.sc-section-desc {
  font-size: 13px;
  font-weight: 400;
  line-height: 20px;
  color: rgba(225, 229, 242, 0.75);
}

.sc-row-label {
  font-size: 14px;
  font-weight: 500;
  line-height: 20px;
  color: #FFFFFF;
}

.sc-row-helper {
  font-size: 13px;
  font-weight: 400;
  line-height: 20px;
  color: rgba(225, 229, 242, 0.75);
}

.sc-meta-text {
  font-size: 13px;
  font-weight: 400;
  color: rgba(225, 229, 242, 0.75);
}

.sc-stat-label {
  font-size: 12px;
  font-weight: 500;
  color: rgba(225, 229, 242, 0.75);
}

.sc-stat-value {
  font-family: "Helvetica", "Nimbus Sans", "Liberation Sans", sans-serif;
  font-feature-settings: "tnum" 1;
  font-size: 20px;
  font-weight: 600;
  line-height: 28px;
  color: #FFFFFF;
}

switch.sc-toggle {
  background-color: rgba(255, 255, 255, 0.07);
  background-image: none;
  border: 1px solid rgba(225, 229, 242, 0.50);
  border-radius: 12px;
  min-width: 40px;
  min-height: 24px;
  min-width: 40px;
  min-height: 24px;
  outline: none;
  transition: background-color 140ms ease, border-color 140ms ease;
}

switch.sc-toggle > slider {
  background-color: rgba(225, 229, 242, 0.70);
  background-image: none;
  border-radius: 9999px;
  min-width: 18px;
  min-height: 18px;
  margin: 2px;
  transition: all 140ms ease;
}

switch.sc-toggle:checked {
  background-color: #1F7A8C;
  border-color: #1F7A8C;
}

switch.sc-toggle:checked > slider {
  background-color: #FFFFFF;
}

checkbutton.sc-option-card {
  background-color: rgba(255, 255, 255, 0.04);
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 12px;
  padding: 20px;
  outline: none;
  transition: background-color 140ms ease, border-color 140ms ease;
}

checkbutton.sc-option-card:hover {
  background-color: rgba(255, 255, 255, 0.07);
}

checkbutton.sc-option-card:checked {
  background-color: rgba(31, 122, 140, 0.18);
  border: 1.5px solid #1F7A8C;
}

checkbutton.sc-option-card .sc-card-title {
  font-size: 14px;
  font-weight: 600;
  color: #FFFFFF;
}

checkbutton.sc-option-card .sc-card-desc {
  font-size: 13px;
  color: rgba(225, 229, 242, 0.75);
}

checkbutton.sc-option-card .sc-card-icon {
  color: rgba(225, 229, 242, 0.75);
}

checkbutton.sc-option-card:checked .sc-card-icon {
  color: #BFDBF7;
}

checkbutton.sc-option-card:disabled {
  opacity: 0.50;
}

.sc-notice {
  background-color: rgba(31, 122, 140, 0.18);
  background-image: none;
  border: 1px solid rgba(31, 122, 140, 0.35);
  border-radius: 12px;
  padding: 12px 16px;
  color: #E1E5F2;
}

.sc-notice-text {
  font-size: 13px;
  line-height: 20px;
  color: #E1E5F2;
}

.sc-dot-connected {
  background-color: #BFDBF7;
  border-radius: 9999px;
  box-shadow: 0 0 6px rgba(191, 219, 247, 0.6);
}

.sc-dot-connecting {
  background-color: #1F7A8C;
  border-radius: 9999px;
  box-shadow: 0 0 6px rgba(31, 122, 140, 0.6);
}

.sc-dot-disconnected {
  background-color: transparent;
  border: 1.5px solid rgba(225, 229, 242, 0.50);
  border-radius: 9999px;
}

.sc-dot-error {
  background-color: #E06C75;
  border-radius: 9999px;
}

.sc-status-label {
  font-size: 13px;
  color: rgba(225, 229, 242, 0.75);
}

.sc-stat-strip {
}

.sc-stat-col {
  padding: 16px;
  border-right: 1px solid rgba(255, 255, 255, 0.08);
}

.sc-stat-strip > *:last-child .sc-stat-col,
.sc-stat-strip > .sc-stat-col:last-child {
  border-right: none;
}

.sc-indicator-fill {
  background-color: #1F7A8C;
  border-radius: 9999px;
}

.sc-indicator-empty {
  background-color: rgba(225, 229, 242, 0.20);
  border-radius: 9999px;
}

.sc-icon-tile {
  min-width: 40px;
  min-height: 40px;
  min-width: 40px;
  min-height: 40px;
  border-radius: 12px;
  background-color: rgba(255, 255, 255, 0.06);
  border: 1px solid rgba(255, 255, 255, 0.08);
  color: rgba(225, 229, 242, 0.75);
}

.sc-icon-muted {
  color: rgba(225, 229, 242, 0.75);
}

.sc-icon-highlight {
  color: #BFDBF7;
}

.sc-kbd {
  font-family: "Helvetica", "Nimbus Sans", "Liberation Sans", sans-serif;
  font-feature-settings: "tnum" 1;
  font-size: 11px;
  font-weight: 600;
  padding: 2px 6px;
  border-radius: 6px;
  background-color: rgba(255, 255, 255, 0.06);
  border: 1px solid rgba(255, 255, 255, 0.10);
  color: rgba(225, 229, 242, 0.75);
}

entry.sc-input,
.sc-input {
  background-color: rgba(255, 255, 255, 0.07);
  background-image: none;
  border: 1px solid rgba(225, 229, 242, 0.50);
  border-radius: 8px;
  color: #FFFFFF;
  padding: 0 12px;
  min-height: 40px;
}

entry.sc-input:focus-within {
  border-color: #BFDBF7;
}

.sc-skeleton {
  background-color: rgba(255, 255, 255, 0.08);
  border-radius: 6px;
}

.sc-empty-state {
  padding: 16px;
  color: rgba(225, 229, 242, 0.75);
}

.sc-empty-label {
  font-size: 13px;
  color: rgba(225, 229, 242, 0.75);
}

popover > contents {
  background-color: #0C3342;
  background-image: none;
  border: 1px solid rgba(255, 255, 255, 0.12);
  border-radius: 12px;
  box-shadow: 0 16px 36px rgba(0, 0, 0, 0.55);
  padding: 6px;
}

popover button.sc-menu-item {
  background-color: transparent;
  background-image: none;
  border: none;
  border-radius: 6px;
  color: #FFFFFF;
  font-size: 13px;
  padding: 8px 12px;
  min-height: 36px;
  outline: none;
  transition: background-color 140ms ease;
}

popover button.sc-menu-item:hover {
  background-color: rgba(255, 255, 255, 0.07);
}

window.sc-root headerbar.sc-header windowcontrols button {
  min-width: 28px;
  min-height: 28px;
  margin: 0 4px;
  padding: 0;
  border-radius: 6px;
}

"#;

fn helvetica_font_setting(current: Option<&str>) -> String {
    let size = current
        .and_then(|s| s.split_whitespace().last())
        .and_then(|tok| tok.parse::<f64>().ok())
        .filter(|&n| (8.0..=24.0).contains(&n))
        .unwrap_or(11.0);
    if size.fract().abs() < f64::EPSILON {
        format!("Helvetica {}", size as u32)
    } else {
        format!("Helvetica {size:.1}")
    }
}

pub fn apply_theme() {
    adw::StyleManager::default().set_color_scheme(adw::ColorScheme::ForceDark);
    if let Some(settings) = gtk::Settings::default() {
        let current = settings.gtk_font_name();
        let font = helvetica_font_setting(current.as_deref());
        settings.set_gtk_font_name(Some(&font));
    }
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

/// Uppercase tracked Helvetica overline (`WORKSPACE`, `BATTERY`, etc.).
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
    Refresh,
    User,
    ExternalLink,
    #[allow(dead_code)] // Constructed in Apps Launchpad (Phase 2).
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
    cr.set_line_width(1.5);

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
        IconKind::User => {
            cr.arc(12.0, 7.6, 3.6, 0.0, std::f64::consts::TAU);
            let _ = cr.stroke();
            cr.move_to(4.6, 19.6);
            cr.curve_to(4.6, 15.2, 8.0, 13.4, 12.0, 13.4);
            cr.curve_to(16.0, 13.4, 19.4, 15.2, 19.4, 19.6);
            let _ = cr.stroke();
        }
        IconKind::ExternalLink => {
            cr.move_to(14.5, 4.2);
            cr.line_to(19.8, 4.2);
            cr.line_to(19.8, 9.5);
            let _ = cr.stroke();
            cr.move_to(10.5, 13.5);
            cr.line_to(19.6, 4.4);
            let _ = cr.stroke();
            cr.move_to(11.0, 5.2);
            cr.line_to(5.8, 5.2);
            cr.curve_to(4.7, 5.2, 4.2, 5.7, 4.2, 6.8);
            cr.line_to(4.2, 18.2);
            cr.curve_to(4.2, 19.3, 4.7, 19.8, 5.8, 19.8);
            cr.line_to(17.2, 19.8);
            cr.curve_to(18.3, 19.8, 18.8, 19.3, 18.8, 18.2);
            cr.line_to(18.8, 13.0);
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
        // Accent background (#1F7A8C)
        cr.set_source_rgb(31.0 / 255.0, 122.0 / 255.0, 140.0 / 255.0);
        let _ = cr.fill();

        // Three minimalist stacked layers inside in #FFFFFF / #BFDBF7
        cr.set_source_rgb(1.0, 1.0, 1.0);
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
        assert!(CSS.contains("Helvetica"));
        assert!(!CSS.contains("JetBrains Mono"));
    }

    #[test]
    fn helvetica_font_setting_preserves_size() {
        assert_eq!(helvetica_font_setting(Some("Adwaita Sans 11")), "Helvetica 11");
        assert_eq!(helvetica_font_setting(Some("Inter 10.5")), "Helvetica 10.5");
        assert_eq!(helvetica_font_setting(None), "Helvetica 11");
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



    #[test]
    fn test_capture_baselines() {
        if adw::init().is_err() { return; }
        use gtk::gsk::prelude::*;
        use gtk::prelude::*;

        let app = adw::Application::builder().application_id("dev.smartconnect.baseline").build();
        let _ = app.register(None::<&gtk::gio::Cancellable>);
        let ui = crate::window::Ui::new(&app);
        crate::theme::apply_theme();
        ui.window().set_default_size(1280, 720);
        ui.window().present();

        let r = gtk::gsk::CairoRenderer::new();
        let _ = r.realize(None::<&gdk::Surface>);

        let art_dir = "/home/lalitmukesh/.gemini/antigravity/brain/06bb29b7-6cfe-49b1-a088-1f523ed5698c";
        let tabs = [("phone", "baseline_phone.png"),
                    ("apps", "baseline_apps.png"),
                    ("display", "baseline_display.png"),
                    ("files", "baseline_files.png"),
                    ("notifications", "baseline_notifications.png")];

        for (tab, filename) in tabs {
            ui.switch_tab(tab);
            for _ in 0..20 {
                gtk::glib::MainContext::default().iteration(false);
            }
            let snap = gtk::Snapshot::new();
            let paintable = gtk::WidgetPaintable::new(Some(ui.window()));
            paintable.snapshot(&snap, 1280.0, 720.0);
            if let Some(node) = snap.to_node() {
                let texture = r.render_texture(&node, None);
                let out_path = format!("{}/{}", art_dir, filename);
                let _ = texture.save_to_png(&out_path);
                println!("Captured {}", out_path);
            }
        }
    }

}
