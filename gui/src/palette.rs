//! Command Palette (`Ctrl+K`) and Endpoint Telemetry modal dialogs.

use std::rc::Rc;

use adw::prelude::*;
use gtk::{gdk, glib};

use crate::apps::App;
use crate::theme::{IconKind, ModernIcon};

#[derive(Clone)]
pub struct CommandEntry {
    pub id: String,
    pub title: String,
    pub category: String,
    pub shortcut: String,
    pub search_haystack: String,
}

impl CommandEntry {
    pub fn new(id: impl Into<String>, title: impl Into<String>, category: impl Into<String>, shortcut: impl Into<String>, extra: &str) -> Self {
        let title = title.into();
        let category = category.into();
        let search_haystack = format!("{title} {category} {extra}").to_lowercase();
        Self {
            id: id.into(),
            title,
            category,
            shortcut: shortcut.into(),
            search_haystack,
        }
    }
}

pub fn filter_commands<'a>(entries: &'a [CommandEntry], query: &str, max_idle: usize) -> Vec<&'a CommandEntry> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return entries.iter().take(max_idle).collect();
    }
    let words: Vec<&str> = q.split_whitespace().collect();
    entries
        .iter()
        .filter(|e| words.iter().all(|w| e.search_haystack.contains(w)))
        .take(40)
        .collect()
}

pub fn build_app_commands(apps: &[App]) -> Vec<CommandEntry> {
    apps.iter()
        .map(|app| {
            let cat = if app.work { "App Stream · Work" } else { "App Stream" };
            CommandEntry::new(
                format!("app:{}", app.pkg),
                format!("Launch {}", app.name),
                cat,
                "Stream",
                &app.pkg,
            )
        })
        .collect()
}

/// Presents the Apple Pro Spotlight (`Ctrl+K`) dialog.
pub fn present_command_palette(
    parent: &impl IsA<gtk::Widget>,
    entries: Vec<CommandEntry>,
    on_run: impl Fn(&str) + 'static,
) {
    let dialog = adw::Dialog::builder()
        .content_width(580)
        .content_height(430)
        .can_close(true)
        .build();

    let search_icon = ModernIcon::new(IconKind::Search, 16);
    search_icon.add_css_class("sc-secondary");
    let entry = gtk::Entry::builder()
        .hexpand(true)
        .placeholder_text("Type a command, switch views, or launch a phone app…")
        .has_frame(false)
        .build();
    let esc_btn = gtk::Button::builder()
        .label("ESC")
        .css_classes(["sc-kbd"])
        .valign(gtk::Align::Center)
        .tooltip_text("Close command palette (Esc)")
        .build();
    let header_bar = gtk::Box::builder()
        .spacing(10)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(16)
        .margin_end(16)
        .build();
    header_bar.append(search_icon.widget());
    header_bar.append(&entry);
    header_bar.append(&esc_btn);

    let list = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::Single)
        .css_classes(["sc-cmd-list"])
        .activate_on_single_click(true)
        .margin_top(6)
        .margin_bottom(6)
        .margin_start(8)
        .margin_end(8)
        .build();
    let scroller = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vexpand(true)
        .child(&list)
        .build();
    let empty_label = gtk::Label::builder()
        .label("No matching commands or phone apps.")
        .css_classes(["sc-muted"])
        .vexpand(true)
        .valign(gtk::Align::Center)
        .halign(gtk::Align::Center)
        .visible(false)
        .build();

    let body = gtk::Box::builder().orientation(gtk::Orientation::Vertical).build();
    body.append(&header_bar);
    body.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
    body.append(&scroller);
    body.append(&empty_label);
    dialog.set_child(Some(&body));

    let entries = Rc::new(entries);
    let shown_ids: Rc<std::cell::RefCell<Vec<String>>> = Rc::default();
    let on_run = Rc::new(on_run);

    let populate = {
        let entries = entries.clone();
        let shown_ids = shown_ids.clone();
        let list = list.clone();
        let scroller = scroller.clone();
        let empty_label = empty_label.clone();
        move |query: &str| {
            while let Some(child) = list.first_child() {
                list.remove(&child);
            }
            let filtered = filter_commands(&entries, query, 12);
            let mut ids = Vec::with_capacity(filtered.len());
            for item in &filtered {
                ids.push(item.id.clone());
                let title = gtk::Label::builder()
                    .label(&item.title)
                    .xalign(0.0)
                    .ellipsize(gtk::pango::EllipsizeMode::End)
                    .build();
                let cat = gtk::Label::builder()
                    .label(&item.category)
                    .xalign(0.0)
                    .ellipsize(gtk::pango::EllipsizeMode::End)
                    .css_classes(["caption", "sc-muted", "sc-mono"])
                    .build();
                let left = gtk::Box::builder()
                    .orientation(gtk::Orientation::Vertical)
                    .spacing(2)
                    .hexpand(true)
                    .build();
                left.append(&title);
                left.append(&cat);

                let row_box = gtk::Box::builder()
                    .spacing(12)
                    .css_classes(["sc-cmd-row"])
                    .build();
                row_box.append(&left);
                if !item.shortcut.is_empty() {
                    row_box.append(&gtk::Label::builder().label(&item.shortcut).css_classes(["sc-kbd"]).valign(gtk::Align::Center).build());
                }
                let row = gtk::ListBoxRow::builder().child(&row_box).build();
                list.append(&row);
            }
            *shown_ids.borrow_mut() = ids;
            let has_items = !filtered.is_empty();
            scroller.set_visible(has_items);
            empty_label.set_visible(!has_items);
            if let Some(first) = list.row_at_index(0) {
                list.select_row(Some(&first));
            }
        }
    };

    populate("");

    {
        let populate = populate.clone();
        entry.connect_changed(move |e| {
            populate(&e.text());
        });
    }

    {
        let dlg = dialog.downgrade();
        esc_btn.connect_clicked(move |_| {
            if let Some(d) = dlg.upgrade() {
                d.close();
            }
        });
    }

    let activate_idx = {
        let shown_ids = shown_ids.clone();
        let on_run = on_run.clone();
        let dlg = dialog.downgrade();
        Rc::new(move |idx: usize| {
            let cmd_id = shown_ids.borrow().get(idx).cloned();
            if let Some(d) = dlg.upgrade() {
                d.close();
            }
            if let Some(id) = cmd_id {
                on_run(&id);
            }
        })
    };

    {
        let activate_idx = activate_idx.clone();
        list.connect_row_activated(move |_, row| {
            if let Ok(idx) = usize::try_from(row.index()) {
                activate_idx(idx);
            }
        });
    }

    {
        let activate_idx = activate_idx.clone();
        let list = list.clone();
        entry.connect_activate(move |_| {
            let idx = list.selected_row().and_then(|r| usize::try_from(r.index()).ok()).unwrap_or(0);
            activate_idx(idx);
        });
    }

    // Arrow key navigation directly from the search entry.
    let key_ctl = gtk::EventControllerKey::new();
    {
        let list = list.clone();
        let shown_ids = shown_ids.clone();
        key_ctl.connect_key_pressed(move |_, key, _, _| {
            let len = shown_ids.borrow().len() as i32;
            if len <= 0 {
                return glib::Propagation::Proceed;
            }
            let cur = list.selected_row().map_or(0, |r| r.index());
            let next = match key {
                gdk::Key::Down => Some((cur + 1).min(len - 1)),
                gdk::Key::Up => Some((cur - 1).max(0)),
                _ => None,
            };
            if let Some(idx) = next {
                if let Some(row) = list.row_at_index(idx) {
                    list.select_row(Some(&row));
                }
                return glib::Propagation::Stop;
            }
            glib::Propagation::Proceed
        });
    }
    entry.add_controller(key_ctl);

    dialog.present(Some(parent));
    entry.grab_focus();
}

/// Presents a compact, truthful Endpoint Telemetry modal.
pub fn present_telemetry_dialog(parent: &impl IsA<gtk::Widget>, rows: &[(&str, String)]) {
    let dialog = adw::Dialog::builder()
        .content_width(450)
        .can_close(true)
        .build();

    let title_row = gtk::Box::builder().spacing(10).build();
    let title_ic = ModernIcon::new(IconKind::Smartphone, 18);
    title_row.append(title_ic.widget());
    title_row.append(&gtk::Label::builder().label("Endpoint Telemetry").xalign(0.0).hexpand(true).css_classes(["heading"]).build());
    let close_ic = ModernIcon::new(IconKind::Close, 14);
    let close_btn = gtk::Button::builder()
        .child(close_ic.widget())
        .css_classes(["sc-btn-ghost"])
        .tooltip_text("Close")
        .build();
    title_row.append(&close_btn);

    let table = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .css_classes(["sc-bento"])
        .build();
    for (idx, (key, value)) in rows.iter().enumerate() {
        if idx > 0 {
            table.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
        }
        let k_lbl = gtk::Label::builder()
            .label(&key.to_uppercase())
            .xalign(0.0)
            .css_classes(["sc-overline"])
            .build();
        let v_lbl = gtk::Label::builder()
            .label(value)
            .xalign(1.0)
            .hexpand(true)
            .ellipsize(gtk::pango::EllipsizeMode::Middle)
            .selectable(true)
            .css_classes(["sc-mono", "caption"])
            .build();
        let r = gtk::Box::builder()
            .spacing(16)
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(14)
            .margin_end(14)
            .build();
        r.append(&k_lbl);
        r.append(&v_lbl);
        table.append(&r);
    }

    let done_btn = gtk::Button::builder()
        .label("Close")
        .halign(gtk::Align::End)
        .css_classes(["sc-btn-primary"])
        .build();

    let content = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(16)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();
    content.append(&title_row);
    content.append(&table);
    content.append(&done_btn);

    let dlg = dialog.downgrade();
    close_btn.connect_clicked(move |_| {
        if let Some(d) = dlg.upgrade() {
            d.close();
        }
    });
    let dlg = dialog.downgrade();
    done_btn.connect_clicked(move |_| {
        if let Some(d) = dlg.upgrade() {
            d.close();
        }
    });

    dialog.set_child(Some(&content));
    dialog.present(Some(parent));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_filter_matches_words_and_limits_idle() {
        let entries = vec![
            CommandEntry::new("nav:phone", "Go to Phone — Overview", "Navigation", "^1", ""),
            CommandEntry::new("nav:apps", "Go to Apps", "Navigation", "^2", ""),
            CommandEntry::new("app:org.example.music", "Launch Echo Music", "App Stream", "Stream", "org.example.music"),
        ];
        assert_eq!(filter_commands(&entries, "", 2).len(), 2);
        let m = filter_commands(&entries, "music stream", 10);
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].id, "app:org.example.music");
    }
}
