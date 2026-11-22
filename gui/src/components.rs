//! Component Library for Smart Connect (2026 Desktop Standard).
//!
//! Build once, use everywhere: PageHeader, Section, Surface, Row, Toggle, Button,
//! IconButton, SegmentedControl, OptionCard, Kbd, StatusDot, Notice, Tooltip,
//! Input/PathPicker, Stat, EmptyState, Skeleton.

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
    child.set_hexpand(true);
    child.set_vexpand(true);
    child.set_halign(gtk::Align::Center);
    child.set_valign(gtk::Align::Center);
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

// ── 14. Dialog Primitive (Shared Component Specification) ───────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogSize {
    Sm, // 440px (Software Update)
    Md, // 480px (About Developer)
    Lg, // 560px
}

impl DialogSize {
    pub fn width(self) -> i32 {
        match self {
            Self::Sm => DIALOG_WIDTH_SM,
            Self::Md => DIALOG_WIDTH_MD,
            Self::Lg => DIALOG_WIDTH_LG,
        }
    }
}

pub struct DialogPrimitive {
    pub dialog: adw::Dialog,
    pub root: gtk::Box,
    pub title_label: gtk::Label,
    pub desc_label: gtk::Label,
    pub close_button: gtk::Button,
    pub header_box: gtk::Box,
    pub header_hairline: gtk::Box,
    pub body_scroller: gtk::ScrolledWindow,
    pub body_box: gtk::Box,
    pub footer_hairline: gtk::Box,
    pub footer_box: gtk::Box,
    pub footer_left_caption: gtk::Box,
    pub footer_actions: gtk::Box,
    pub live_region: gtk::Label,
}

impl DialogPrimitive {
    pub fn new(size: DialogSize, title: &str, description: Option<&str>) -> Self {
        let width = size.width();
        let dialog = adw::Dialog::builder()
            .content_width(width)
            .can_close(true)
            .css_classes(["sc-dialog"])
            .build();
        Self::with_dialog(dialog, size, title, description)
    }

    pub fn with_dialog(dialog: adw::Dialog, size: DialogSize, title: &str, description: Option<&str>) -> Self {
        let width = size.width();
        dialog.set_content_width(width);
        dialog.set_can_close(true);
        dialog.add_css_class("sc-dialog");

        let title_label = gtk::Label::builder()
            .label(title)
            .xalign(0.0)
            .css_classes(["sc-dialog-title"])
            .build();

        let desc_label = gtk::Label::builder()
            .xalign(0.0)
            .wrap(true)
            .css_classes(["sc-dialog-desc"])
            .visible(description.is_some())
            .build();
        if let Some(desc) = description {
            desc_label.set_label(desc);
        }

        let title_stack = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(SPACE_1)
            .hexpand(true)
            .build();
        title_stack.append(&title_label);
        title_stack.append(&desc_label);

        let close_ic = ModernIcon::new(IconKind::Close, 14);
        let close_button = gtk::Button::builder()
            .child(close_ic.widget())
            .css_classes(["sc-dialog-close-btn"])
            .tooltip_text("Close (Esc)")
            .valign(gtk::Align::Center)
            .build();

        let header_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(SPACE_3)
            .valign(gtk::Align::Center)
            .css_classes(["sc-dialog-header"])
            .build();
        header_box.append(&title_stack);
        header_box.append(&close_button);

        let header_hairline = gtk::Box::builder()
            .css_classes(["sc-dialog-hairline"])
            .opacity(0.0)
            .margin_top(SPACE_4)
            .build();

        let body_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(SPACE_4)
            .css_classes(["sc-dialog-body"])
            .build();

        let body_scroller = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Never)
            .propagate_natural_height(true)
            .max_content_height(480)
            .min_content_height(100)
            .child(&body_box)
            .build();

        let footer_hairline = gtk::Box::builder()
            .css_classes(["sc-dialog-hairline"])
            .opacity(0.0)
            .build();

        let footer_left_caption = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(SPACE_2)
            .valign(gtk::Align::Center)
            .hexpand(true)
            .css_classes(["sc-dialog-footer-caption"])
            .build();

        let footer_actions = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(SPACE_2)
            .halign(gtk::Align::End)
            .valign(gtk::Align::Center)
            .build();

        let footer_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(SPACE_3)
            .valign(gtk::Align::Center)
            .css_classes(["sc-dialog-footer"])
            .build();
        footer_box.append(&footer_left_caption);
        footer_box.append(&footer_actions);

        let live_region = gtk::Label::builder()
            .visible(false)
            .accessible_role(gtk::AccessibleRole::Status)
            .build();

        let h_hair = header_hairline.clone();
        let f_hair = footer_hairline.clone();
        body_scroller.vadjustment().connect_value_changed(move |adj| {
            let val = adj.value();
            h_hair.set_opacity(if val > 1.0 { 1.0 } else { 0.0 });
            let at_bottom = val >= adj.upper() - adj.page_size() - 2.0;
            f_hair.set_opacity(if at_bottom { 0.0 } else { 1.0 });
        });

        let root = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .width_request(width)
            .css_classes(["sc-dialog"])
            .build();
        root.append(&header_box);
        root.append(&header_hairline);
        root.append(&body_scroller);
        root.append(&footer_hairline);
        root.append(&footer_box);
        root.append(&live_region);

        let d_weak = dialog.downgrade();
        close_button.connect_clicked(move |_| {
            if let Some(d) = d_weak.upgrade() {
                d.close();
            }
        });

        dialog.set_child(Some(&root));

        Self {
            dialog,
            root,
            title_label,
            desc_label,
            close_button,
            header_box,
            header_hairline,
            body_scroller,
            body_box,
            footer_hairline,
            footer_box,
            footer_left_caption,
            footer_actions,
            live_region,
        }
    }

    pub fn announce(&self, text: &str) {
        self.live_region.set_label(text);
    }
}


