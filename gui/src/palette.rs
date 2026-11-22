//! Command Palette (`Ctrl+K`), Endpoint Telemetry, and GitHub-backed About Developer modal dialogs.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::rc::Rc;

use adw::prelude::*;
use gtk::{gdk, gio, glib};
use serde::{Deserialize, Serialize};

use crate::apps::App;
use crate::components::{
    empty_state, ghost_button, notice_banner, secondary_button, skeleton, status_dot,
    ConnectionState, DialogPrimitive, DialogSize,
};
use crate::theme::{IconKind, ModernIcon};
use crate::tokens::*;

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
        .content_width(540)
        .content_height(360)
        .can_close(true)
        .css_classes(["sc-dialog"])
        .build();

    let search_icon = ModernIcon::new(IconKind::Search, 16);
    search_icon.add_css_class("sc-secondary");
    let entry = gtk::Entry::builder()
        .hexpand(true)
        .placeholder_text("Type a command, switch views, or launch a phone app…")
        .has_frame(false)
        .css_classes(["sc-palette-search-entry"])
        .build();
    let esc_btn = gtk::Button::builder()
        .label("ESC")
        .css_classes(["sc-kbd"])
        .valign(gtk::Align::Center)
        .tooltip_text("Close command palette (Esc)")
        .build();
    let header_bar = gtk::Box::builder()
        .spacing(8)
        .valign(gtk::Align::Center)
        .css_classes(["sc-search-pill-container"])
        .build();
    header_bar.append(search_icon.widget());
    header_bar.append(&entry);
    header_bar.append(&esc_btn);

    let list = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::Single)
        .css_classes(["sc-cmd-list"])
        .activate_on_single_click(true)
        .margin_top(2)
        .margin_bottom(8)
        .margin_start(10)
        .margin_end(10)
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
                    .css_classes(["sc-cmd-title"])
                    .build();
                let cat = gtk::Label::builder()
                    .label(&item.category)
                    .xalign(0.0)
                    .ellipsize(gtk::pango::EllipsizeMode::End)
                    .css_classes(["caption", "sc-muted"])
                    .build();
                let left = gtk::Box::builder()
                    .orientation(gtk::Orientation::Vertical)
                    .spacing(2)
                    .hexpand(true)
                    .valign(gtk::Align::Center)
                    .build();
                left.append(&title);
                left.append(&cat);

                let row_box = gtk::Box::builder()
                    .spacing(12)
                    .css_classes(["sc-cmd-row"])
                    .valign(gtk::Align::Center)
                    .build();
                row_box.append(&left);
                if !item.shortcut.is_empty() {
                    row_box.append(&gtk::Label::builder().label(&item.shortcut).css_classes(["sc-kbd"]).valign(gtk::Align::Center).build());
                }
                let row = gtk::ListBoxRow::builder().child(&row_box).css_classes(["sc-cmd-list-row"]).build();
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
        .css_classes(["sc-dialog"])
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

const GITHUB_LOGIN: &str = "Lalitmukesh69";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct DeveloperRepo {
    pub name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stargazers_count: u64,
}

impl Default for DeveloperRepo {
    fn default() -> Self {
        Self {
            name: String::new(),
            html_url: String::new(),
            description: None,
            language: None,
            stargazers_count: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct DeveloperProfile {
    pub login: String,
    pub name: String,
    pub avatar_url: String,
    pub html_url: String,
    pub bio: String,
    pub company: String,
    pub location: String,
    pub blog: String,
    pub twitter_username: String,
    pub public_repos: u64,
    pub followers: u64,
    pub following: u64,
    pub updated_at: String,
    pub repos: Vec<DeveloperRepo>,
}

impl Default for DeveloperProfile {
    fn default() -> Self {
        Self::snapshot()
    }
}

impl DeveloperProfile {
    /// Baseline snapshot populated from `https://api.github.com/users/Lalitmukesh69`.
    pub fn snapshot() -> Self {
        Self {
            login: GITHUB_LOGIN.into(),
            name: "Lalit mukesh".into(),
            avatar_url: "https://avatars.githubusercontent.com/u/123398933?v=4".into(),
            html_url: "https://github.com/Lalitmukesh69".into(),
            bio: "I'm a Ai Software engineer and full stack developer based in Berlin. I love building web apps, AI agent products, automations, and open-source projects.".into(),
            company: "IRM".into(),
            location: "Berlin".into(),
            blog: "m.lalitm.me".into(),
            twitter_username: "lalit73483".into(),
            public_repos: 17,
            followers: 5,
            following: 3,
            updated_at: String::new(),
            repos: vec![
                DeveloperRepo {
                    name: "smartconnect-linux-but-better".into(),
                    html_url: "https://github.com/Lalitmukesh69/smartconnect-linux-but-better".into(),
                    description: Some("Enhanced GTK4/libadwaita user interface and desktop experience for smartconnect-linux".into()),
                    language: Some("Rust".into()),
                    stargazers_count: 1,
                },
                DeveloperRepo {
                    name: "doxy-lubricants-cfo-ai-Agent".into(),
                    html_url: "https://github.com/Lalitmukesh69/doxy-lubricants-cfo-ai-Agent".into(),
                    description: Some("Motor-oil logistics AI agent with data science forecasting and CFO analytics suite.".into()),
                    language: Some("TypeScript".into()),
                    stargazers_count: 1,
                },
                DeveloperRepo {
                    name: "DuoLauncher".into(),
                    html_url: "https://github.com/Lalitmukesh69/DuoLauncher".into(),
                    description: Some("An experimental Android launcher for foldables, with a right-side dock, native widgets, and overlapping unfolded Home pages.".into()),
                    language: Some("Kotlin".into()),
                    stargazers_count: 0,
                },
            ],
        }
    }

    pub fn website_url(&self) -> Option<String> {
        normalize_web_url(&self.blog)
    }

    pub fn twitter_url(&self) -> Option<String> {
        let handle = self.twitter_username.trim().trim_start_matches('@');
        (!handle.is_empty()).then(|| format!("https://x.com/{handle}"))
    }
}

#[derive(Deserialize)]
struct GitHubUserApi {
    login: String,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    avatar_url: Option<String>,
    #[serde(default)]
    html_url: Option<String>,
    #[serde(default)]
    bio: Option<String>,
    #[serde(default)]
    company: Option<String>,
    #[serde(default)]
    location: Option<String>,
    #[serde(default)]
    blog: Option<String>,
    #[serde(default)]
    twitter_username: Option<String>,
    #[serde(default)]
    public_repos: u64,
    #[serde(default)]
    followers: u64,
    #[serde(default)]
    following: u64,
    #[serde(default)]
    updated_at: Option<String>,
}

#[derive(Deserialize)]
struct GitHubRepoApi {
    name: String,
    html_url: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    language: Option<String>,
    #[serde(default)]
    stargazers_count: u64,
    #[serde(default)]
    fork: bool,
}

pub fn normalize_web_url(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.starts_with("https://") || trimmed.starts_with("http://") {
        Some(trimmed.to_owned())
    } else {
        Some(format!("https://{trimmed}"))
    }
}

pub fn parse_github_response(user_json: &str, repos_json: Option<&str>) -> Result<DeveloperProfile, String> {
    let user: GitHubUserApi = serde_json::from_str(user_json).map_err(|e| format!("invalid GitHub user JSON: {e}"))?;
    let clean = |s: Option<String>, fallback: &str| -> String {
        s.map(|v| v.trim().to_owned()).filter(|v| !v.is_empty()).unwrap_or_else(|| fallback.to_owned())
    };
    let clean_opt = |s: Option<String>| -> String {
        s.map(|v| v.trim().to_owned()).unwrap_or_default()
    };

    let mut repos: Vec<DeveloperRepo> = repos_json
        .and_then(|raw| serde_json::from_str::<Vec<GitHubRepoApi>>(raw).ok())
        .map(|items| {
            let mut filtered: Vec<DeveloperRepo> = items
                .into_iter()
                .filter(|r| !r.fork)
                .map(|r| DeveloperRepo {
                    name: r.name,
                    html_url: r.html_url,
                    description: r.description.map(|d| d.trim().to_owned()).filter(|d| !d.is_empty()),
                    language: r.language.map(|l| l.trim().to_owned()).filter(|l| !l.is_empty()),
                    stargazers_count: r.stargazers_count,
                })
                .collect();
            filtered.sort_by_key(|r| if r.name == "smartconnect-linux-but-better" { 0 } else { 1 });
            filtered.truncate(3);
            filtered
        })
        .unwrap_or_else(|| DeveloperProfile::snapshot().repos);

    if repos.is_empty() {
        repos = DeveloperProfile::snapshot().repos;
    }

    let login = if user.login.trim().is_empty() { GITHUB_LOGIN.to_owned() } else { user.login.trim().to_owned() };
    Ok(DeveloperProfile {
        name: clean(user.name, &login),
        avatar_url: clean_opt(user.avatar_url),
        html_url: clean(user.html_url, &format!("https://github.com/{login}")),
        bio: clean_opt(user.bio),
        company: clean_opt(user.company),
        location: clean_opt(user.location),
        blog: clean_opt(user.blog),
        twitter_username: clean_opt(user.twitter_username),
        login,
        public_repos: user.public_repos,
        followers: user.followers,
        following: user.following,
        updated_at: clean_opt(user.updated_at),
        repos,
    })
}

fn developer_cache_dir() -> PathBuf {
    std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .filter(|p| p.is_absolute())
        .unwrap_or_else(|| PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".cache"))
        .join("smartconnect-linux")
}

fn developer_profile_cache_path() -> PathBuf {
    developer_cache_dir().join("developer-github.json")
}

fn developer_avatar_cache_path() -> PathBuf {
    developer_cache_dir().join("developer-avatar.png")
}

fn load_cached_developer() -> DeveloperProfile {
    std::fs::read_to_string(developer_profile_cache_path())
        .ok()
        .and_then(|raw| serde_json::from_str::<DeveloperProfile>(&raw).ok())
        .unwrap_or_else(DeveloperProfile::snapshot)
}

fn write_atomic_bytes(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, bytes)?;
    std::fs::rename(tmp, path)
}

fn curl_text(url: &str) -> Result<String, String> {
    let out = Command::new("curl")
        .args([
            "-fsSL",
            "-m",
            "8",
            "-H",
            "Accept: application/vnd.github+json",
            "-H",
            "User-Agent: smartconnect-gui",
            url,
        ])
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("could not run curl: {e}"))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_owned();
        return Err(if stderr.is_empty() { format!("HTTP status {}", out.status) } else { stderr });
    }
    String::from_utf8(out.stdout).map_err(|e| format!("invalid UTF-8 response: {e}"))
}

fn curl_bytes(url: &str) -> Result<Vec<u8>, String> {
    let out = Command::new("curl")
        .args(["-fsSL", "-m", "8", "-H", "User-Agent: smartconnect-gui", url])
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("could not download avatar: {e}"))?;
    if !out.status.success() {
        return Err(format!("avatar download failed ({})", out.status));
    }
    Ok(out.stdout)
}

fn fetch_developer_from_github() -> Result<(DeveloperProfile, Option<PathBuf>), String> {
    let user_url = format!("https://api.github.com/users/{GITHUB_LOGIN}");
    let repos_url = format!("https://api.github.com/users/{GITHUB_LOGIN}/repos?sort=updated&per_page=6&type=owner");
    let user_json = curl_text(&user_url)?;
    let repos_json = curl_text(&repos_url).ok();
    let profile = parse_github_response(&user_json, repos_json.as_deref())?;

    if let Ok(json) = serde_json::to_string_pretty(&profile) {
        let _ = write_atomic_bytes(&developer_profile_cache_path(), format!("{json}\n").as_bytes());
    }

    let avatar_path = if !profile.avatar_url.is_empty() {
        let dest = developer_avatar_cache_path();
        if let Ok(bytes) = curl_bytes(&profile.avatar_url) {
            if write_atomic_bytes(&dest, &bytes).is_ok() {
                Some(dest)
            } else {
                None
            }
        } else {
            dest.is_file().then_some(dest)
        }
    } else {
        None
    };

    Ok((profile, avatar_path))
}

fn platform_link_button(
    platform_icon: IconKind,
    label: &str,
    url: &str,
    tooltip: &str,
    accessible_name: &str,
) -> gtk::Button {
    let row = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(SPACE_2)
        .valign(gtk::Align::Center)
        .build();
    let p_ic = ModernIcon::new(platform_icon, 16);
    let lbl = gtk::Label::builder().label(label).build();
    let ext_ic = ModernIcon::new(IconKind::ExternalLink, 12);
    ext_ic.add_css_class("sc-secondary");
    row.append(p_ic.widget());
    row.append(&lbl);
    row.append(ext_ic.widget());

    let btn = gtk::Button::builder()
        .child(&row)
        .css_classes(["sc-dialog-link-btn"])
        .tooltip_text(tooltip)
        .valign(gtk::Align::Center)
        .build();
    btn.update_property(&[gtk::accessible::Property::Label(accessible_name)]);

    let url_owned = url.to_owned();
    btn.connect_clicked(move |b| {
        let win = b.root().and_downcast::<gtk::Window>();
        gtk::UriLauncher::new(&url_owned).launch(win.as_ref(), gio::Cancellable::NONE, |_| {});
    });
    btn
}

#[derive(Clone, Debug)]
pub enum DeveloperDialogState {
    Loading,
    Loaded {
        profile: DeveloperProfile,
        avatar_path: Option<PathBuf>,
        updated_caption: String,
    },
    Error {
        message: String,
        cached: Option<(DeveloperProfile, Option<PathBuf>, String)>,
    },
}

fn build_loaded_profile_content(
    prim: &DialogPrimitive,
    profile: &DeveloperProfile,
    avatar_path: Option<&Path>,
) {
    // 1. Profile row: 72x72 avatar + text stack (left-aligned)
    let avatar_frame = gtk::Box::builder()
        .width_request(72)
        .height_request(72)
        .css_classes(["sc-dialog-avatar"])
        .overflow(gtk::Overflow::Hidden)
        .valign(gtk::Align::Center)
        .build();

    let avatar_stack = gtk::Stack::new();
    let fallback = adw::Avatar::builder()
        .size(72)
        .text(&profile.name)
        .show_initials(true)
        .build();
    let pic = gtk::Picture::builder()
        .content_fit(gtk::ContentFit::Cover)
        .width_request(72)
        .height_request(72)
        .css_classes(["sc-dialog-avatar-img"])
        .overflow(gtk::Overflow::Hidden)
        .build();

    avatar_stack.add_named(&fallback, Some("fallback"));
    avatar_stack.add_named(&pic, Some("image"));

    if let Some(Ok(tex)) = avatar_path.map(gdk::Texture::from_filename) {
        pic.set_paintable(Some(&tex));
        avatar_stack.set_visible_child_name("image");
    } else {
        avatar_stack.set_visible_child_name("fallback");
    }
    avatar_frame.append(&avatar_stack);

    // Text stack
    let name_lbl = gtk::Label::builder()
        .label(&profile.name)
        .xalign(0.0)
        .css_classes(["sc-dialog-dev-name"])
        .build();

    let handle_str = format!("@{}", profile.login);
    let handle_url = profile.html_url.clone();
    let handle_btn = gtk::Button::builder()
        .label(&handle_str)
        .css_classes(["flat", "sc-dialog-dev-handle"])
        .halign(gtk::Align::Start)
        .tooltip_text(format!("View GitHub profile ({})", profile.html_url))
        .build();
    handle_btn.update_property(&[gtk::accessible::Property::Label(
        &format!("GitHub handle @{}, opens in browser", profile.login),
    )]);
    handle_btn.connect_clicked(move |b| {
        let win = b.root().and_downcast::<gtk::Window>();
        gtk::UriLauncher::new(&handle_url).launch(win.as_ref(), gio::Cancellable::NONE, |_| {});
    });

    let meta_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(6)
        .valign(gtk::Align::Center)
        .build();

    if !profile.location.is_empty() {
        let pin = ModernIcon::new(IconKind::MapPin, 13);
        pin.add_css_class("sc-secondary");
        let loc_lbl = gtk::Label::builder().label(&profile.location).css_classes(["sc-dialog-dev-meta"]).build();
        meta_box.append(pin.widget());
        meta_box.append(&loc_lbl);
    }
    if !profile.location.is_empty() && !profile.company.is_empty() {
        let dot_sep = gtk::Label::builder().label("·").css_classes(["sc-dialog-dev-meta"]).build();
        meta_box.append(&dot_sep);
    }
    if !profile.company.is_empty() {
        let bld = ModernIcon::new(IconKind::Building, 13);
        bld.add_css_class("sc-secondary");
        let comp_lbl = gtk::Label::builder().label(&profile.company).css_classes(["sc-dialog-dev-meta"]).build();
        meta_box.append(bld.widget());
        meta_box.append(&comp_lbl);
    }

    let text_stack = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(2)
        .hexpand(true)
        .valign(gtk::Align::Center)
        .build();
    text_stack.append(&name_lbl);
    text_stack.append(&handle_btn);
    if !profile.location.is_empty() || !profile.company.is_empty() {
        text_stack.append(&meta_box);
    }

    let profile_row = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(16)
        .valign(gtk::Align::Center)
        .build();
    profile_row.append(&avatar_frame);
    profile_row.append(&text_stack);
    prim.body_box.append(&profile_row);

    // 2. Bio: 14/22, max 56ch, left-aligned, 16px below header row
    if !profile.bio.is_empty() {
        let bio_lbl = gtk::Label::builder()
            .label(&profile.bio)
            .xalign(0.0)
            .wrap(true)
            .wrap_mode(gtk::pango::WrapMode::WordChar)
            .max_width_chars(56)
            .css_classes(["sc-dialog-bio"])
            .build();
        prim.body_box.append(&bio_lbl);
    }

    // 3. Links row: secondary 36px link buttons, 8px gap
    let links_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(8)
        .build();

    // GitHub
    links_box.append(&platform_link_button(
        IconKind::Github,
        "GitHub",
        &profile.html_url,
        &profile.html_url,
        "GitHub profile (opens in browser)",
    ));

    // Blog / Website
    if let Some(site) = profile.website_url() {
        let label = if profile.blog.trim().is_empty() { "Website" } else { profile.blog.trim() };
        links_box.append(&platform_link_button(
            IconKind::Globe,
            label,
            &site,
            &site,
            "Personal website (opens in browser)",
        ));
    }

    // X / Twitter
    if let Some(tw) = profile.twitter_url() {
        let handle = profile.twitter_username.trim().trim_start_matches('@');
        let label = format!("X @{handle}");
        links_box.append(&platform_link_button(
            IconKind::XLogo,
            &label,
            &tw,
            &tw,
            "X profile (opens in browser)",
        ));
    }

    prim.body_box.append(&links_box);
}

/// Builds a modern libadwaita "About the developer" dialog primitive for a given state.
pub fn build_developer_dialog_for_state(
    dialog: Option<adw::Dialog>,
    state: &DeveloperDialogState,
    on_refresh: Rc<dyn Fn()>,
    on_retry: Rc<dyn Fn()>,
) -> DialogPrimitive {
    let prim = match dialog {
        Some(d) => DialogPrimitive::with_dialog(d, DialogSize::Md, "About the developer", None),
        None => DialogPrimitive::new(DialogSize::Md, "About the developer", None),
    };
    prim.close_button.grab_focus();

    match state {
        DeveloperDialogState::Loading => {
            // Skeletons
            let avatar_skel = skeleton(72, 72);
            avatar_skel.set_css_classes(&["sc-skeleton", "sc-dialog-avatar"]);

            let name_skel = skeleton(150, 24);
            let handle_skel = skeleton(100, 14);
            let meta_skel = skeleton(120, 14);

            let text_stack = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(6)
                .hexpand(true)
                .valign(gtk::Align::Center)
                .build();
            text_stack.append(&name_skel);
            text_stack.append(&handle_skel);
            text_stack.append(&meta_skel);

            let profile_row = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .spacing(16)
                .valign(gtk::Align::Center)
                .build();
            profile_row.append(&avatar_skel);
            profile_row.append(&text_stack);
            prim.body_box.append(&profile_row);

            // Bio skeletons (3 lines)
            let bio_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(8)
                .margin_top(8)
                .build();
            bio_box.append(&skeleton(420, 14));
            bio_box.append(&skeleton(360, 14));
            bio_box.append(&skeleton(240, 14));
            prim.body_box.append(&bio_box);

            // Link skeletons (3 buttons)
            let links_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .spacing(8)
                .margin_top(8)
                .build();
            links_box.append(&skeleton(96, 36));
            links_box.append(&skeleton(108, 36));
            links_box.append(&skeleton(120, 36));
            prim.body_box.append(&links_box);

            // Footer
            let caption_lbl = gtk::Label::builder()
                .label("Connecting to GitHub…")
                .xalign(0.0)
                .css_classes(["sc-dialog-footer-caption"])
                .build();
            prim.footer_left_caption.append(&caption_lbl);

            let refresh_btn = ghost_button("Refresh");
            refresh_btn.set_sensitive(false);
            let spin = adw::Spinner::builder().width_request(16).height_request(16).build();
            let refresh_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .spacing(6)
                .valign(gtk::Align::Center)
                .build();
            refresh_box.append(&spin);
            refresh_box.append(&gtk::Label::new(Some("Refresh")));
            refresh_btn.set_child(Some(&refresh_box));
            prim.footer_actions.append(&refresh_btn);

            prim.announce("Loading developer profile from GitHub");
        }
        DeveloperDialogState::Loaded { profile, avatar_path, updated_caption } => {
            build_loaded_profile_content(&prim, profile, avatar_path.as_deref());

            // Footer
            let dot = status_dot(ConnectionState::Connected);
            let caption_lbl = gtk::Label::builder()
                .label(updated_caption)
                .xalign(0.0)
                .css_classes(["sc-dialog-footer-caption"])
                .tooltip_text(format!("https://api.github.com/users/{}", profile.login))
                .build();
            prim.footer_left_caption.append(&dot);
            prim.footer_left_caption.append(&caption_lbl);

            let refresh_btn = ghost_button("Refresh");
            let r_ic = ModernIcon::new(IconKind::Refresh, 14);
            let refresh_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Horizontal)
                .spacing(6)
                .valign(gtk::Align::Center)
                .build();
            refresh_box.append(r_ic.widget());
            refresh_box.append(&gtk::Label::new(Some("Refresh")));
            refresh_btn.set_child(Some(&refresh_box));
            let on_r = on_refresh.clone();
            refresh_btn.connect_clicked(move |_| on_r());
            prim.footer_actions.append(&refresh_btn);

            prim.announce(&format!("Loaded developer profile for {}", profile.name));
        }
        DeveloperDialogState::Error { message, cached } => {
            let notice = notice_banner(IconKind::WifiOff, message);
            prim.body_box.append(&notice);

            if let Some((cached_prof, cached_av, caption)) = cached {
                build_loaded_profile_content(&prim, cached_prof, cached_av.as_deref());

                let caption_lbl = gtk::Label::builder()
                    .label(caption)
                    .xalign(0.0)
                    .css_classes(["sc-dialog-footer-caption"])
                    .tooltip_text(format!("https://api.github.com/users/{}", cached_prof.login))
                    .build();
                prim.footer_left_caption.append(&caption_lbl);
            } else {
                let empty = empty_state(IconKind::User, "No cached developer profile available offline.");
                prim.body_box.append(&empty);

                let caption_lbl = gtk::Label::builder()
                    .label("Offline")
                    .xalign(0.0)
                    .css_classes(["sc-dialog-footer-caption"])
                    .build();
                prim.footer_left_caption.append(&caption_lbl);
            }

            let retry_btn = secondary_button("Retry");
            let on_ret = on_retry.clone();
            retry_btn.connect_clicked(move |_| on_ret());
            prim.footer_actions.append(&retry_btn);

            prim.announce("Couldn't load latest profile from GitHub");
        }
    }

    prim
}

/// Presents the redesigned "About Developer" modal populated from GitHub (`Lalitmukesh69`).
pub fn present_developer_dialog(parent: &impl IsA<gtk::Widget>) {
    let state_cell: Rc<RefCell<DeveloperDialogState>> = Rc::new(RefCell::new(DeveloperDialogState::Loading));

    let dialog = adw::Dialog::builder()
        .content_width(DIALOG_WIDTH_MD)
        .can_close(true)
        .css_classes(["sc-dialog"])
        .build();

    let redraw: Rc<RefCell<Option<Rc<dyn Fn()>>>> = Rc::default();

    let redraw_fn: Rc<dyn Fn()> = {
        let dialog = dialog.clone();
        let state_cell = state_cell.clone();
        let redraw = redraw.clone();

        Rc::new(move || {
            let current_state = state_cell.borrow().clone();

            let on_refresh: Rc<dyn Fn()> = {
                let state_cell = state_cell.clone();
                let redraw = redraw.clone();
                let dialog_weak = dialog.downgrade();
                Rc::new(move || {
                    *state_cell.borrow_mut() = DeveloperDialogState::Loading;
                    if let Some(r) = redraw.borrow().as_ref() {
                        r();
                    }

                    let state_cell_done = state_cell.clone();
                    let redraw_done = redraw.clone();
                    let dialog_weak_done = dialog_weak.clone();

                    glib::spawn_future_local(async move {
                        let res = gio::spawn_blocking(fetch_developer_from_github).await;
                        if dialog_weak_done.upgrade().is_none() {
                            return;
                        }
                        match res {
                            Ok(Ok((profile, avatar_path))) => {
                                *state_cell_done.borrow_mut() = DeveloperDialogState::Loaded {
                                    profile,
                                    avatar_path,
                                    updated_caption: "Live from GitHub · updated just now".to_owned(),
                                };
                            }
                            Ok(Err(err)) => {
                                let cached_profile = load_cached_developer();
                                let cached_avatar = developer_avatar_cache_path();
                                let cached_opt = if cached_avatar.is_file() {
                                    Some((cached_profile, Some(cached_avatar), "Offline · showing cached profile".to_owned()))
                                } else {
                                    None
                                };
                                *state_cell_done.borrow_mut() = DeveloperDialogState::Error {
                                    message: format!("Couldn't load the latest profile: {err}"),
                                    cached: cached_opt,
                                };
                            }
                            Err(_) => {
                                *state_cell_done.borrow_mut() = DeveloperDialogState::Error {
                                    message: "Background GitHub task failed".to_owned(),
                                    cached: None,
                                };
                            }
                        }
                        if let Some(r) = redraw_done.borrow().as_ref() {
                            r();
                        }
                    });
                })
            };

            let on_retry = on_refresh.clone();

            build_developer_dialog_for_state(Some(dialog.clone()), &current_state, on_refresh, on_retry);
        })
    };

    *redraw.borrow_mut() = Some(redraw_fn.clone());
    redraw_fn();

    dialog.present(Some(parent));

    // Fetch initial profile
    let state_cell_done = state_cell.clone();
    let redraw_done = redraw.clone();
    let dialog_weak_done = dialog.downgrade();

    glib::spawn_future_local(async move {
        let res = gio::spawn_blocking(fetch_developer_from_github).await;
        if dialog_weak_done.upgrade().is_none() {
            return;
        }
        match res {
            Ok(Ok((profile, avatar_path))) => {
                *state_cell_done.borrow_mut() = DeveloperDialogState::Loaded {
                    profile,
                    avatar_path,
                    updated_caption: "Live from GitHub · updated 2 min ago".to_owned(),
                };
            }
            Ok(Err(err)) => {
                let cached_profile = load_cached_developer();
                let cached_avatar = developer_avatar_cache_path();
                let cached_opt = if cached_avatar.is_file() {
                    Some((cached_profile, Some(cached_avatar), "Offline · showing cached profile".to_owned()))
                } else {
                    None
                };
                *state_cell_done.borrow_mut() = DeveloperDialogState::Error {
                    message: format!("Couldn't load the latest profile: {err}"),
                    cached: cached_opt,
                };
            }
            Err(_) => {
                *state_cell_done.borrow_mut() = DeveloperDialogState::Error {
                    message: "Background GitHub task failed".to_owned(),
                    cached: None,
                };
            }
        }
        if let Some(r) = redraw_done.borrow().as_ref() {
            r();
        }
    });
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

    #[test]
    fn parses_github_profile_and_normalizes_links() {
        let user = r#"{
            "login": "Lalitmukesh69",
            "name": "Lalit mukesh ",
            "avatar_url": "https://avatars.githubusercontent.com/u/123398933?v=4",
            "html_url": "https://github.com/Lalitmukesh69",
            "bio": "AI software engineer in Berlin.",
            "company": "IRM",
            "location": "Berlin",
            "blog": "m.lalitm.me",
            "twitter_username": "lalit73483",
            "public_repos": 17,
            "followers": 5,
            "following": 3
        }"#;
        let repos = r#"[
            {"name":"other","html_url":"https://github.com/Lalitmukesh69/other","description":null,"language":"TypeScript","stargazers_count":0,"fork":false},
            {"name":"smartconnect-linux-but-better","html_url":"https://github.com/Lalitmukesh69/smartconnect-linux-but-better","description":"UI redesign","language":"Rust","stargazers_count":1,"fork":false},
            {"name":"forked-repo","html_url":"https://github.com/Lalitmukesh69/forked-repo","description":"Fork","language":"C","stargazers_count":9,"fork":true}
        ]"#;
        let parsed = parse_github_response(user, Some(repos)).unwrap();
        assert_eq!(parsed.name, "Lalit mukesh");
        assert_eq!(parsed.location, "Berlin");
        assert_eq!(parsed.company, "IRM");
        assert_eq!(parsed.public_repos, 17);
        assert_eq!(parsed.website_url().as_deref(), Some("https://m.lalitm.me"));
        assert_eq!(parsed.twitter_url().as_deref(), Some("https://x.com/lalit73483"));
        assert_eq!(parsed.repos.len(), 2);
        assert_eq!(parsed.repos[0].name, "smartconnect-linux-but-better");
    }

    #[test]
    fn handles_null_optional_github_fields() {
        let user = r#"{
            "login": "Lalitmukesh69",
            "name": null,
            "bio": null,
            "company": null,
            "location": null,
            "blog": "",
            "twitter_username": null,
            "public_repos": 4,
            "followers": 1,
            "following": 2
        }"#;
        let parsed = parse_github_response(user, None).unwrap();
        assert_eq!(parsed.name, "Lalitmukesh69");
        assert!(parsed.bio.is_empty());
        assert_eq!(parsed.website_url(), None);
        assert_eq!(parsed.twitter_url(), None);
        assert!(!parsed.repos.is_empty());
    }
}
