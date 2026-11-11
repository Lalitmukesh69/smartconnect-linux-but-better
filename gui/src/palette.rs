//! Command Palette (`Ctrl+K`), Endpoint Telemetry, and GitHub-backed About Developer modal dialogs.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::rc::Rc;

use adw::prelude::*;
use gtk::{gdk, gio, glib};
use serde::{Deserialize, Serialize};

use crate::apps::App;
use crate::theme::{overline, IconKind, ModernIcon};

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
        .css_classes(["sc-dialog"])
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

fn open_url(widget: &impl IsA<gtk::Widget>, url: &str) {
    let win = widget.root().and_downcast::<gtk::Window>();
    gtk::UriLauncher::new(url).launch(win.as_ref(), gio::Cancellable::NONE, |_| {});
}

fn external_cta(label: &str, primary: bool, url: &str) -> gtk::Button {
    let ic = ModernIcon::new(IconKind::ExternalLink, 13);
    let inner = gtk::Box::builder().spacing(7).halign(gtk::Align::Center).valign(gtk::Align::Center).build();
    inner.append(&gtk::Label::new(Some(label)));
    inner.append(ic.widget());
    let btn = gtk::Button::builder()
        .child(&inner)
        .css_classes([if primary { "sc-btn-primary" } else { "sc-btn-secondary" }])
        .tooltip_text(url)
        .build();
    let u = url.to_owned();
    btn.connect_clicked(move |b| open_url(b, &u));
    btn
}

fn stat_bento(title: &str) -> (gtk::Box, gtk::Label) {
    let value = gtk::Label::builder().label("0").xalign(0.0).css_classes(["sc-metric"]).build();
    let tile = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .hexpand(true)
        .css_classes(["sc-stat-tile"])
        .build();
    tile.append(&overline(title));
    tile.append(&value);
    (tile, value)
}

/// Presents the Apple Pro Dark Monochrome "About Developer" modal populated from GitHub (`Lalitmukesh69`).
pub fn present_developer_dialog(parent: &impl IsA<gtk::Widget>) {
    let dialog = adw::Dialog::builder()
        .content_width(550)
        .can_close(true)
        .css_classes(["sc-dialog"])
        .build();

    let title_ic = ModernIcon::new(IconKind::User, 17);
    let title_lbl = gtk::Label::builder().label("About Developer").xalign(0.0).css_classes(["heading"]).build();
    let gh_tag = gtk::Label::builder().label(format!("@{GITHUB_LOGIN}")).css_classes(["sc-tag"]).valign(gtk::Align::Center).build();

    let sync_spinner = adw::Spinner::builder().visible(false).valign(gtk::Align::Center).build();
    let sync_ic = ModernIcon::new(IconKind::Refresh, 14);
    let sync_btn = gtk::Button::builder()
        .child(sync_ic.widget())
        .css_classes(["sc-btn-icon"])
        .tooltip_text("Pull latest profile from GitHub")
        .valign(gtk::Align::Center)
        .build();
    let close_ic = ModernIcon::new(IconKind::Close, 14);
    let close_btn = gtk::Button::builder()
        .child(close_ic.widget())
        .css_classes(["sc-btn-ghost"])
        .tooltip_text("Close")
        .valign(gtk::Align::Center)
        .build();

    let header_left = gtk::Box::builder().spacing(10).hexpand(true).valign(gtk::Align::Center).build();
    header_left.append(title_ic.widget());
    header_left.append(&title_lbl);
    header_left.append(&gh_tag);

    let title_row = gtk::Box::builder().spacing(8).valign(gtk::Align::Center).build();
    title_row.append(&header_left);
    title_row.append(&sync_spinner);
    title_row.append(&sync_btn);
    title_row.append(&close_btn);

    // Hero card
    let avatar_fallback = adw::Avatar::builder().size(74).text("Lalit mukesh").show_initials(true).build();
    let avatar_pic = gtk::Picture::builder()
        .content_fit(gtk::ContentFit::Cover)
        .width_request(74)
        .height_request(74)
        .css_classes(["sc-avatar"])
        .overflow(gtk::Overflow::Hidden)
        .build();
    let avatar_stack = gtk::Stack::new();
    avatar_stack.add_named(&avatar_fallback, Some("fallback"));
    avatar_stack.add_named(&avatar_pic, Some("image"));
    let avatar_frame = gtk::Box::builder()
        .width_request(76)
        .height_request(76)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Start)
        .css_classes(["sc-avatar-frame"])
        .overflow(gtk::Overflow::Hidden)
        .build();
    avatar_frame.append(&avatar_stack);

    let name_lbl = gtk::Label::builder().xalign(0.0).css_classes(["sc-view-title"]).wrap(true).build();
    let meta_lbl = gtk::Label::builder().xalign(0.0).css_classes(["sc-mono", "caption", "sc-muted"]).wrap(true).build();
    let bio_lbl = gtk::Label::builder()
        .xalign(0.0)
        .css_classes(["sc-secondary"])
        .wrap(true)
        .wrap_mode(gtk::pango::WrapMode::WordChar)
        .build();

    let hero_links = gtk::Box::builder().spacing(8).margin_top(6).build();

    let hero_col = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(6)
        .hexpand(true)
        .build();
    hero_col.append(&name_lbl);
    hero_col.append(&meta_lbl);
    hero_col.append(&bio_lbl);
    hero_col.append(&hero_links);

    let hero_row = gtk::Box::builder()
        .spacing(18)
        .margin_top(18)
        .margin_bottom(18)
        .margin_start(18)
        .margin_end(18)
        .build();
    hero_row.append(&avatar_frame);
    hero_row.append(&hero_col);

    let hero_card = gtk::Box::builder().orientation(gtk::Orientation::Vertical).css_classes(["sc-bento"]).build();
    hero_card.append(&hero_row);

    // 3-Column GitHub Stats Bento Grid
    let (repos_tile, repos_val) = stat_bento("REPOSITORIES");
    let (followers_tile, followers_val) = stat_bento("FOLLOWERS");
    let (following_tile, following_val) = stat_bento("FOLLOWING");
    let stats_row = gtk::Box::builder().spacing(10).homogeneous(true).build();
    stats_row.append(&repos_tile);
    stats_row.append(&followers_tile);
    stats_row.append(&following_tile);

    // Repositories Bento Card
    let repos_hdr = gtk::Box::builder().spacing(8).margin_start(4).margin_end(4).build();
    let repos_hdr_l = overline("Featured Repositories");
    repos_hdr_l.set_hexpand(true);
    repos_hdr.append(&repos_hdr_l);
    repos_hdr.append(&gtk::Label::builder().label("github.com").css_classes(["sc-mono", "caption", "sc-muted"]).build());

    let repos_list = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .css_classes(["sc-bento"])
        .overflow(gtk::Overflow::Hidden)
        .build();
    let repos_section = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(8).build();
    repos_section.append(&repos_hdr);
    repos_section.append(&repos_list);

    // Footer status + Done
    let sync_status = gtk::Label::builder()
        .label("Pulling profile from GitHub…")
        .xalign(0.0)
        .hexpand(true)
        .ellipsize(gtk::pango::EllipsizeMode::End)
        .css_classes(["caption", "sc-muted", "sc-mono"])
        .build();
    let done_btn = gtk::Button::builder().label("Close").css_classes(["sc-btn-primary"]).build();
    let footer = gtk::Box::builder().spacing(12).valign(gtk::Align::Center).build();
    footer.append(&sync_status);
    footer.append(&done_btn);

    let body = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(16)
        .margin_top(20)
        .margin_bottom(20)
        .margin_start(20)
        .margin_end(20)
        .build();
    body.append(&title_row);
    body.append(&hero_card);
    body.append(&stats_row);
    body.append(&repos_section);
    body.append(&footer);

    let scroller = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .propagate_natural_height(true)
        .max_content_height(660)
        .child(&body)
        .build();

    let apply_avatar = {
        let avatar_pic = avatar_pic.clone();
        let avatar_stack = avatar_stack.clone();
        move |path: Option<&Path>| {
            if let Some(Ok(tex)) = path.map(gdk::Texture::from_filename) {
                avatar_pic.set_paintable(Some(&tex));
                avatar_stack.set_visible_child_name("image");
            } else {
                avatar_stack.set_visible_child_name("fallback");
            }
        }
    };

    let populate: Rc<dyn Fn(&DeveloperProfile, Option<&Path>)> = {
        let name_lbl = name_lbl.clone();
        let meta_lbl = meta_lbl.clone();
        let bio_lbl = bio_lbl.clone();
        let hero_links = hero_links.clone();
        let repos_val = repos_val.clone();
        let followers_val = followers_val.clone();
        let following_val = following_val.clone();
        let repos_list = repos_list.clone();
        let avatar_fallback = avatar_fallback.clone();
        Rc::new(move |profile: &DeveloperProfile, avatar_path: Option<&Path>| {
            avatar_fallback.set_text(Some(&profile.name));
            apply_avatar(avatar_path);

            name_lbl.set_label(&profile.name);
            let mut parts = vec![format!("@{}", profile.login)];
            if !profile.location.is_empty() {
                parts.push(profile.location.clone());
            }
            if !profile.company.is_empty() {
                parts.push(profile.company.clone());
            }
            meta_lbl.set_label(&parts.join("  ·  "));

            bio_lbl.set_visible(!profile.bio.is_empty());
            bio_lbl.set_label(&profile.bio);

            while let Some(child) = hero_links.first_child() {
                hero_links.remove(&child);
            }
            hero_links.append(&external_cta("GitHub Profile", true, &profile.html_url));
            if let Some(site) = profile.website_url() {
                let label = if profile.blog.trim().is_empty() { "Website".to_owned() } else { profile.blog.trim().to_owned() };
                hero_links.append(&external_cta(&label, false, &site));
            }
            if let Some(tw) = profile.twitter_url() {
                hero_links.append(&external_cta(&format!("@{}", profile.twitter_username.trim().trim_start_matches('@')), false, &tw));
            }

            repos_val.set_label(&profile.public_repos.to_string());
            followers_val.set_label(&profile.followers.to_string());
            following_val.set_label(&profile.following.to_string());

            while let Some(child) = repos_list.first_child() {
                repos_list.remove(&child);
            }
            for (idx, repo) in profile.repos.iter().enumerate() {
                if idx > 0 {
                    repos_list.append(&gtk::Separator::new(gtk::Orientation::Horizontal));
                }
                let r_name = gtk::Label::builder()
                    .label(&repo.name)
                    .xalign(0.0)
                    .hexpand(true)
                    .ellipsize(gtk::pango::EllipsizeMode::End)
                    .css_classes(["heading"])
                    .build();
                let r_badges = gtk::Box::builder().spacing(6).valign(gtk::Align::Center).build();
                if let Some(lang) = &repo.language {
                    r_badges.append(&gtk::Label::builder().label(lang).css_classes(["sc-tag"]).build());
                }
                if repo.stargazers_count > 0 {
                    r_badges.append(&gtk::Label::builder().label(&format!("★ {}", repo.stargazers_count)).css_classes(["sc-tag"]).build());
                }
                let ext = ModernIcon::new(IconKind::ExternalLink, 13);
                ext.add_css_class("sc-muted");
                r_badges.append(ext.widget());

                let top_line = gtk::Box::builder().spacing(8).build();
                top_line.append(&r_name);
                top_line.append(&r_badges);

                let row_col = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(3).build();
                row_col.append(&top_line);
                if let Some(desc) = &repo.description {
                    row_col.append(
                        &gtk::Label::builder()
                            .label(desc)
                            .xalign(0.0)
                            .wrap(true)
                            .wrap_mode(gtk::pango::WrapMode::WordChar)
                            .css_classes(["caption", "sc-secondary"])
                            .build(),
                    );
                }
                let btn = gtk::Button::builder()
                    .child(&row_col)
                    .css_classes(["sc-repo-row"])
                    .tooltip_text(&repo.html_url)
                    .build();
                let repo_url = repo.html_url.clone();
                btn.connect_clicked(move |b| open_url(b, &repo_url));
                repos_list.append(&btn);
            }
        })
    };

    let cached_profile = load_cached_developer();
    let cached_avatar = developer_avatar_cache_path();
    populate(&cached_profile, cached_avatar.is_file().then_some(cached_avatar.as_path()));

    let syncing: Rc<RefCell<bool>> = Rc::default();
    let start_sync: Rc<dyn Fn()> = {
        let syncing = syncing.clone();
        let sync_btn = sync_btn.clone();
        let sync_spinner = sync_spinner.clone();
        let sync_status = sync_status.clone();
        let populate = populate.clone();
        let dialog_w = dialog.downgrade();
        Rc::new(move || {
            if *syncing.borrow() {
                return;
            }
            *syncing.borrow_mut() = true;
            sync_btn.set_sensitive(false);
            sync_spinner.set_visible(true);
            sync_status.set_label("Pulling latest profile from github.com…");

            let syncing_done = syncing.clone();
            let sync_btn_done = sync_btn.clone();
            let sync_spinner_done = sync_spinner.clone();
            let sync_status_done = sync_status.clone();
            let populate_done = populate.clone();
            let dialog_w = dialog_w.clone();

            glib::spawn_future_local(async move {
                let res = gio::spawn_blocking(fetch_developer_from_github)
                    .await
                    .unwrap_or_else(|_| Err("GitHub sync task failed".to_owned()));
                *syncing_done.borrow_mut() = false;
                if dialog_w.upgrade().is_none() {
                    return;
                }
                sync_btn_done.set_sensitive(true);
                sync_spinner_done.set_visible(false);
                match res {
                    Ok((profile, avatar_path)) => {
                        populate_done(&profile, avatar_path.as_deref());
                        sync_status_done.set_label("Live from api.github.com/users/Lalitmukesh69");
                    }
                    Err(err) => {
                        eprintln!("smartconnect-gui: GitHub profile fetch skipped ({err})");
                        sync_status_done.set_label("Offline · showing saved GitHub profile");
                    }
                }
            });
        })
    };

    {
        let start_sync = start_sync.clone();
        sync_btn.connect_clicked(move |_| start_sync());
    }
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

    dialog.set_child(Some(&scroller));
    dialog.present(Some(parent));
    start_sync();
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
