//! Component Library for Smart Connect (2026 Desktop Standard).
//!
//! Build once, use everywhere: PageHeader, Section, Surface, Row, Toggle, Button,
//! IconButton, SegmentedControl, OptionCard, Kbd, StatusDot, Notice, Tooltip,
//! Input/PathPicker, Stat, EmptyState, Skeleton, and a complete Kitchen Sink preview page.

use adw::prelude::*;

use crate::theme::{IconKind, ModernIcon};
use crate::tokens::*;

/// Visual connection state for dots, labels, and single connect control.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Error,
}

impl ConnectionState {
    pub fn label(self) -> &'static str {
        match self {
            Self::Disconnected => "Disconnected",
            Self::Connecting => "Connecting…",
            Self::Connected => "Connected",
            Self::Error => "Error",
        }
    }
}

// ── 1. PageHeader ───────────────────────────────────────────────────────────

/// Standard PageHeader: title 28/36 600, description 14/20 muted (max 68ch), 32px below.
/// At most ONE optional secondary action on the right.
pub fn page_header(
    title: &str,
    description: &str,
    action: Option<&impl IsA<gtk::Widget>>,
) -> (gtk::Box, gtk::Label, gtk::Label) {
    let title_lbl = gtk::Label::builder()
        .label(title)
        .xalign(0.0)
        .css_classes(["sc-page-title"])
        .build();

    let desc_lbl = gtk::Label::builder()
        .label(description)
        .xalign(0.0)
        .wrap(true)
        .max_width_chars(68)
        .css_classes(["sc-page-desc"])
        .build();

    let text_col = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(SPACE_1)
        .hexpand(true)
        .build();
    text_col.append(&title_lbl);
    text_col.append(&desc_lbl);

    let row = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(SPACE_4)
        .margin_bottom(SPACE_8)
        .valign(gtk::Align::Center)
        .build();
    row.append(&text_col);

    if let Some(act) = action {
        act.set_valign(gtk::Align::Center);
        row.append(act);
    }

    (row, title_lbl, desc_lbl)
}

/// Unified page scroller with centered max-width clamp and overlay scrollbar
pub fn page_scroller(child: &impl IsA<gtk::Widget>, max_width: i32) -> gtk::ScrolledWindow {
    let clamp = adw::Clamp::builder()
        .maximum_size(max_width)
        .tightening_threshold(max_width)
        .child(child)
        .build();
    gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vexpand(true)
        .child(&clamp)
        .build()
}

// ── 2. Section & Surface ────────────────────────────────────────────────────

/// Standard Section: title 16/600, optional 13/20 muted description, 12px gap, then ONE Surface.
/// Sections are spaced 40px apart.
pub fn section(
    title: &str,
    description: Option<&str>,
    surface: &impl IsA<gtk::Widget>,
) -> gtk::Box {
    let title_lbl = section_title(title);

    let header_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(SPACE_1)
        .build();
    header_box.append(&title_lbl);

    if let Some(desc) = description {
        let desc_lbl = gtk::Label::builder()
            .label(desc)
            .xalign(0.0)
            .wrap(true)
            .max_width_chars(68)
            .css_classes(["sc-section-desc"])
            .build();
        header_box.append(&desc_lbl);
    }

    let sec = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(SPACE_3)
        .margin_bottom(SPACE_10)
        .build();
    sec.append(&header_box);
    sec.append(surface);
    sec
}

/// Standalone section title label (16/600 sentence case)
pub fn section_title(title: &str) -> gtk::Label {
    gtk::Label::builder()
        .label(title)
        .xalign(0.0)
        .css_classes(["sc-section-title"])
        .build()
}

/// Standard Surface: surface-1 background, 1px border, radius 12, rows separated by 1px hairlines.
pub fn surface() -> gtk::Box {
    gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .css_classes(["sc-surface"])
        .build()
}

/// 1px hairline divider (border color: rgba(255, 255, 255, 0.08))
pub fn hairline() -> gtk::Box {
    gtk::Box::builder()
        .css_classes(["sc-hairline"])
        .hexpand(true)
        .build()
}

// ── 3. Row ──────────────────────────────────────────────────────────────────

/// Standard Surface Row: min-height 56 (64 with helper), padding 12 vertical / 16 horizontal.
/// Left: optional 20px icon or 40px tile, label 14/500, helper 13 muted.
/// Right: control or value right-aligned in one consistent column.
pub fn surface_row(
    leading: Option<&gtk::Widget>,
    label: &str,
    helper: Option<&str>,
    trailing: &impl IsA<gtk::Widget>,
) -> gtk::Box {
    let lbl = gtk::Label::builder()
        .label(label)
        .xalign(0.0)
        .css_classes(["sc-row-label"])
        .ellipsize(gtk::pango::EllipsizeMode::End)
        .build();

    let text_col = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(2)
        .hexpand(true)
        .valign(gtk::Align::Center)
        .build();
    text_col.append(&lbl);

    let min_height = if let Some(h) = helper {
        let helper_lbl = gtk::Label::builder()
            .label(h)
            .xalign(0.0)
            .wrap(true)
            .css_classes(["sc-row-helper"])
            .build();
        text_col.append(&helper_lbl);
        HEIGHT_ROW_HELPER
    } else {
        HEIGHT_ROW_DEFAULT
    };

    let row = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(SPACE_4)
        .css_classes(["sc-row"])
        .height_request(min_height)
        .valign(gtk::Align::Center)
        .build();

    if let Some(lead) = leading {
        lead.set_valign(gtk::Align::Center);
        row.append(lead);
    }
    row.append(&text_col);

    trailing.set_valign(gtk::Align::Center);
    trailing.set_halign(gtk::Align::End);
    row.append(trailing);

    row
}

/// Navigating row: entire row is clickable with a 16px chevron right.
pub fn navigating_row(
    leading: Option<&gtk::Widget>,
    label: &str,
    helper: Option<&str>,
    meta: Option<&str>,
) -> (gtk::Button, gtk::Label) {
    let lbl = gtk::Label::builder()
        .label(label)
        .xalign(0.0)
        .css_classes(["sc-row-label"])
        .build();

    let text_col = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(2)
        .hexpand(true)
        .valign(gtk::Align::Center)
        .build();
    text_col.append(&lbl);

    let min_height = if let Some(h) = helper {
        let helper_lbl = gtk::Label::builder()
            .label(h)
            .xalign(0.0)
            .wrap(true)
            .css_classes(["sc-row-helper"])
            .build();
        text_col.append(&helper_lbl);
        HEIGHT_ROW_HELPER
    } else {
        HEIGHT_ROW_DEFAULT
    };

    let right_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(SPACE_2)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::End)
        .build();

    let meta_lbl = gtk::Label::builder()
        .label(meta.unwrap_or(""))
        .css_classes(["sc-meta-text"])
        .visible(meta.is_some())
        .build();
    right_box.append(&meta_lbl);

    let chev = ModernIcon::new(IconKind::ChevronRight, 16);
    chev.add_css_class("sc-icon-muted");
    right_box.append(chev.widget());

    let row = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(SPACE_4)
        .css_classes(["sc-row", "sc-row-interactive"])
        .height_request(min_height)
        .valign(gtk::Align::Center)
        .build();

    if let Some(lead) = leading {
        lead.set_valign(gtk::Align::Center);
        row.append(lead);
    }
    row.append(&text_col);
    row.append(&right_box);

    let btn = gtk::Button::builder()
        .child(&row)
        .css_classes(["sc-btn-surface-row"])
        .hexpand(true)
        .build();

    (btn, meta_lbl)
}

// ── 4. Buttons & Toggles ────────────────────────────────────────────────────

/// Standard Primary Button: accent fill, white text, darker on hover/pressed.
pub fn primary_button(label: &str) -> gtk::Button {
    gtk::Button::builder()
        .label(label)
        .css_classes(["sc-btn-primary"])
        .height_request(HEIGHT_BTN_DEFAULT)
        .build()
}

/// Standard Secondary Button: surface-2 fill, soft 20% border.
pub fn secondary_button(label: &str) -> gtk::Button {
    gtk::Button::builder()
        .label(label)
        .css_classes(["sc-btn-secondary"])
        .height_request(HEIGHT_BTN_DEFAULT)
        .build()
}

/// Standard Ghost Button: transparent background, hover surface-2.
pub fn ghost_button(label: &str) -> gtk::Button {
    gtk::Button::builder()
        .label(label)
        .css_classes(["sc-btn-ghost"])
        .height_request(HEIGHT_BTN_MD)
        .build()
}

/// Row Action Button with standardized min-width 88px.
pub fn row_action_button(label: &str, primary: bool) -> gtk::Button {
    let cls = if primary { "sc-btn-primary" } else { "sc-btn-secondary" };
    gtk::Button::builder()
        .label(label)
        .css_classes([cls, "sc-btn-row-action"])
        .width_request(MIN_WIDTH_ROW_BTN)
        .height_request(HEIGHT_BTN_MD)
        .valign(gtk::Align::Center)
        .build()
}

/// Destructive Button (danger off-palette red).
pub fn destructive_button(label: &str) -> gtk::Button {
    gtk::Button::builder()
        .label(label)
        .css_classes(["sc-btn-danger"])
        .height_request(HEIGHT_BTN_DEFAULT)
        .build()
}

/// Standard IconButton: 32x32 / 40x40 touch target with 16px/20px icon.
pub fn icon_button(kind: IconKind, size: i32, tooltip: &str) -> (gtk::Button, ModernIcon) {
    let icon = ModernIcon::new(kind, size);
    let btn = gtk::Button::builder()
        .child(icon.widget())
        .css_classes(["sc-btn-icon"])
        .width_request(HEIGHT_BTN_DEFAULT)
        .height_request(HEIGHT_BTN_DEFAULT)
        .tooltip_text(tooltip)
        .valign(gtk::Align::Center)
        .build();
    (btn, icon)
}

/// Standard Switch / Toggle: role=switch, 40x24, 18px thumb.
pub fn toggle_switch(active: bool) -> gtk::Switch {
    gtk::Switch::builder()
        .active(active)
        .css_classes(["sc-toggle"])
        .valign(gtk::Align::Center)
        .build()
}

// ── 5. Status & Badges ──────────────────────────────────────────────────────

/// StatusDot: 8px dot with status vocabulary (dot + text, never color alone).
pub fn status_dot(state: ConnectionState) -> gtk::Box {
    let dot = gtk::Box::builder()
        .width_request(8)
        .height_request(8)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .build();
    set_status_dot_state(&dot, state);
    dot
}

pub fn set_status_dot_state(dot: &gtk::Box, state: ConnectionState) {
    dot.remove_css_class("sc-dot-connected");
    dot.remove_css_class("sc-dot-connecting");
    dot.remove_css_class("sc-dot-disconnected");
    dot.remove_css_class("sc-dot-error");

    match state {
        ConnectionState::Connected => dot.add_css_class("sc-dot-connected"),
        ConnectionState::Connecting => dot.add_css_class("sc-dot-connecting"),
        ConnectionState::Disconnected => dot.add_css_class("sc-dot-disconnected"),
        ConnectionState::Error => dot.add_css_class("sc-dot-error"),
    }
}

/// StatusIndicator: Dot + Text compound widget.
pub fn status_indicator(state: ConnectionState) -> (gtk::Box, gtk::Box, gtk::Label) {
    let dot = status_dot(state);
    let lbl = gtk::Label::builder()
        .label(state.label())
        .css_classes(["sc-status-label"])
        .valign(gtk::Align::Center)
        .build();

    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(SPACE_2)
        .valign(gtk::Align::Center)
        .build();
    container.append(&dot);
    container.append(&lbl);

    (container, dot, lbl)
}

/// Keyboard shortcut tag (Kbd).
pub fn kbd_tag(text: &str) -> gtk::Label {
    gtk::Label::builder()
        .label(text)
        .css_classes(["sc-kbd"])
        .valign(gtk::Align::Center)
        .build()
}

// ── 6. Notice & OptionCard ──────────────────────────────────────────────────

/// Notice banner: icon plus text, accent-tint background, points to sidebar.
pub fn notice_banner(kind: IconKind, message: &str) -> gtk::Box {
    let icon = ModernIcon::new(kind, 16);
    icon.add_css_class("sc-icon-highlight");
    icon.widget().set_valign(gtk::Align::Center);

    let lbl = gtk::Label::builder()
        .label(message)
        .xalign(0.0)
        .wrap(true)
        .css_classes(["sc-notice-text"])
        .hexpand(true)
        .valign(gtk::Align::Center)
        .build();

    let banner = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(SPACE_3)
        .css_classes(["sc-notice"])
        .margin_bottom(SPACE_6)
        .valign(gtk::Align::Center)
        .build();
    banner.append(icon.widget());
    banner.append(&lbl);
    banner
}

/// OptionCard: single select card, equal height, padding 20, 24px icon, label 14/600, description 13 muted.
pub fn option_card(
    group: Option<&gtk::CheckButton>,
    kind: IconKind,
    title: &str,
    description: &str,
) -> (gtk::CheckButton, ModernIcon) {
    let icon = ModernIcon::new(kind, 24);
    icon.add_css_class("sc-card-icon");
    icon.widget().set_valign(gtk::Align::Start);

    let title_lbl = gtk::Label::builder()
        .label(title)
        .xalign(0.0)
        .css_classes(["sc-card-title"])
        .build();

    let desc_lbl = gtk::Label::builder()
        .label(description)
        .xalign(0.0)
        .wrap(true)
        .css_classes(["sc-card-desc"])
        .build();

    let text_col = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(SPACE_1)
        .hexpand(true)
        .build();
    text_col.append(&title_lbl);
    text_col.append(&desc_lbl);

    let inner = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(SPACE_3)
        .build();
    inner.append(icon.widget());
    inner.append(&text_col);

    let check = gtk::CheckButton::builder()
        .css_classes(["sc-option-card"])
        .child(&inner)
        .hexpand(true)
        .build();

    if let Some(grp) = group {
        check.set_group(Some(grp));
    }

    (check, icon)
}

// ── 7. Stat Widget ──────────────────────────────────────────────────────────

/// Standard Stat: label 12/500 muted, value 20/28 600 tabular, 4px indicator slot.
pub fn stat_widget(
    label: &str,
    value: &str,
    indicator: Option<&impl IsA<gtk::Widget>>,
) -> (gtk::Box, gtk::Label, gtk::Box) {
    let lbl = gtk::Label::builder()
        .label(label)
        .xalign(0.0)
        .css_classes(["sc-stat-label"])
        .build();

    let val = gtk::Label::builder()
        .label(value)
        .xalign(0.0)
        .css_classes(["sc-stat-value"])
        .build();

    let slot = gtk::Box::builder()
        .height_request(4)
        .valign(gtk::Align::Center)
        .build();
    if let Some(ind) = indicator {
        slot.append(ind);
    }

    let col = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(SPACE_2)
        .hexpand(true)
        .css_classes(["sc-stat-col"])
        .build();
    col.append(&lbl);
    col.append(&val);
    col.append(&slot);

    (col, val, slot)
}

// ── 8. EmptyState & Skeleton ────────────────────────────────────────────────

/// EmptyState: one line, one small icon, no illustration.
pub fn empty_state(kind: IconKind, message: &str) -> gtk::Box {
    let icon = ModernIcon::new(kind, 20);
    icon.add_css_class("sc-icon-muted");

    let lbl = gtk::Label::builder()
        .label(message)
        .css_classes(["sc-empty-label"])
        .build();

    let box_ = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(SPACE_3)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .css_classes(["sc-empty-state"])
        .margin_top(SPACE_8)
        .margin_bottom(SPACE_8)
        .build();
    box_.append(icon.widget());
    box_.append(&lbl);
    box_
}

/// Skeleton placeholder for loading / connecting states.
pub fn skeleton(width: i32, height: i32) -> gtk::Box {
    gtk::Box::builder()
        .width_request(width)
        .height_request(height)
        .css_classes(["sc-skeleton"])
        .build()
}

/// 40px Icon Tile with 12px rounded corner for continuity and app rows.
pub fn icon_tile(child: &impl IsA<gtk::Widget>) -> gtk::Box {
    let tile = gtk::Box::builder()
        .width_request(ICON_TILE_SIZE)
        .height_request(ICON_TILE_SIZE)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .css_classes(["sc-icon-tile"])
        .build();
    tile.append(child);
    tile
}

// ── 9. Kitchen Sink Demo Page ───────────────────────────────────────────────

/// Complete interactive Kitchen Sink page demonstrating every component in every state.
pub fn build_kitchen_sink_page() -> gtk::Widget {
    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(0)
        .margin_start(CONTENT_PAD_HORIZ)
        .margin_end(CONTENT_PAD_HORIZ)
        .margin_top(CONTENT_PAD_TOP)
        .margin_bottom(CONTENT_PAD_BOTTOM)
        .build();

    // 1. Page Header with optional action
    let refresh_btn = secondary_button("Refresh");
    let (header, _, _) = page_header(
        "Design System Kitchen Sink",
        "Visual verification of all tokens and components under the 2026 dark palette.",
        Some(&refresh_btn),
    );
    content.append(&header);

    // 2. Notices
    let notice = notice_banner(
        IconKind::Smartphone,
        "Connect your phone from the bottom of the sidebar to activate system continuity.",
    );
    content.append(&notice);

    // 3. Section with Buttons & Controls
    let btn_row = gtk::Box::builder().spacing(SPACE_3).build();
    btn_row.append(&primary_button("Primary Button"));
    btn_row.append(&secondary_button("Secondary Button"));
    btn_row.append(&ghost_button("Ghost Button"));
    btn_row.append(&destructive_button("Destructive"));
    btn_row.append(&row_action_button("Action", false));

    let surf_btns = surface();
    surf_btns.append(&surface_row(None, "Standard Buttons", Some("Accent fill, surface-2, ghost, and danger states"), &btn_row));

    let toggle = toggle_switch(true);
    surf_btns.append(&surface_row(None, "Toggle Switch (Role=Switch)", Some("40×24px, 18px thumb with border-control outline"), &toggle));

    let (nav_row, _) = navigating_row(None, "Interactive Navigating Row", Some("Clickable row with hover surface and chevron"), Some("Meta: Ready"));
    surf_btns.append(&nav_row);

    content.append(&section("Buttons and Surfaces", Some("Single Surface with hairline-separated rows"), &surf_btns));

    // 4. Section with OptionCards
    let cards_box = gtk::Box::builder().spacing(SPACE_4).homogeneous(true).build();
    let (card1, _) = option_card(None, IconKind::Mouse, "Shared mouse", "Universal cursor control");
    card1.set_active(true);
    let (card2, _) = option_card(Some(&card1), IconKind::Monitor, "Duplicate display", "Mirror this PC screen");
    let (card3, _) = option_card(Some(&card1), IconKind::Monitor, "Extend display", "Secondary workspace monitor");
    cards_box.append(&card1);
    cards_box.append(&card2);
    cards_box.append(&card3);
    content.append(&section("Option Cards (Single Select)", Some("Accent-tint fill and border on active state"), &cards_box));

    // 5. Section with Stats Strip
    let stat_surface = surface();
    let stats_strip = gtk::Box::builder().homogeneous(true).css_classes(["sc-stat-strip"]).build();
    let ind1 = gtk::Box::builder().height_request(4).css_classes(["sc-indicator-fill"]).build();
    let (stat1, _, _) = stat_widget("Battery", "85%", Some(&ind1));
    let ind2 = gtk::Box::builder().height_request(4).css_classes(["sc-indicator-fill"]).build();
    let (stat2, _, _) = stat_widget("Wi-Fi link", "5 GHz", Some(&ind2));
    let ind3 = gtk::Box::builder().height_request(4).css_classes(["sc-indicator-fill"]).build();
    let (stat3, _, _) = stat_widget("Cellular", "5G Full", Some(&ind3));
    let (stat4, _, _) = stat_widget("Bluetooth", "Active", None::<&gtk::Widget>);
    stats_strip.append(&stat1);
    stats_strip.append(&stat2);
    stats_strip.append(&stat3);
    stats_strip.append(&stat4);
    stat_surface.append(&stats_strip);
    content.append(&section("4-Column Stat Strip", Some("Reserved 4px indicator slot for tabular alignment"), &stat_surface));

    // 6. Section with Status Dots, Badges, and Skeletons
    let status_surface = surface();
    let dots_box = gtk::Box::builder().spacing(SPACE_4).build();
    let (c1, _, _) = status_indicator(ConnectionState::Connected);
    let (c2, _, _) = status_indicator(ConnectionState::Connecting);
    let (c3, _, _) = status_indicator(ConnectionState::Disconnected);
    let (c4, _, _) = status_indicator(ConnectionState::Error);
    dots_box.append(&c1);
    dots_box.append(&c2);
    dots_box.append(&c3);
    dots_box.append(&c4);
    status_surface.append(&surface_row(None, "Status Dots & Vocabulary", Some("Dot plus text, never color alone"), &dots_box));

    let skel = skeleton(140, 24);
    status_surface.append(&surface_row(None, "Skeleton Loading Shimmer", Some("Connecting and loading state placeholder"), &skel));

    let empty = empty_state(IconKind::Bell, "No notifications from the phone");
    status_surface.append(&empty);

    content.append(&section("States & Indicators", Some("Accessibility-compliant indicators and empty states"), &status_surface));

    let clamp = adw::Clamp::builder()
        .maximum_size(CONTENT_MAX_WIDTH)
        .tightening_threshold(CONTENT_MAX_WIDTH)
        .child(&content)
        .build();

    gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vexpand(true)
        .child(&clamp)
        .build()
        .upcast()
}
