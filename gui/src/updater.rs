//! In-app software update checking, changelog modal, and binary updater.
//!
//! Queries the GitHub Releases API for the latest release, compares semantic versions,
//! presents a modern libadwaita modal dialog with release notes, and supports 1-click
//! in-app updating via atomic POSIX executable replacement.

use std::cell::{Cell, RefCell};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::time::Duration;

use adw::prelude::*;
use gtk::{gdk, gio, glib};
use serde::{Deserialize, Serialize};

use crate::components::{
    icon_button, notice_banner, primary_button, secondary_button, ghost_button,
    DialogPrimitive, DialogSize,
};
use crate::theme::{IconKind, ModernIcon};
use crate::tokens::*;

pub const GITHUB_REPO: &str = "Lalitmukesh69/smartconnect-linux-but-better";
pub const GITHUB_RELEASES_URL: &str = "https://api.github.com/repos/Lalitmukesh69/smartconnect-linux-but-better/releases";
pub const GITHUB_LATEST_URL: &str = "https://api.github.com/repos/Lalitmukesh69/smartconnect-linux-but-better/releases/latest";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseAsset {
    pub name: String,
    #[serde(default)]
    pub size: u64,
    pub browser_download_url: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseInfo {
    pub tag_name: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub published_at: String,
    #[serde(default)]
    pub html_url: String,
    #[serde(default)]
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InstallTarget {
    /// Portable Sharun distribution (~/.local/opt/smartconnect-linux/shared/bin/smartconnect-gui)
    SharunShared(PathBuf),
    /// Direct binary in user space (~/.local/bin/smartconnect-gui)
    DirectBinary(PathBuf),
    /// Standalone AppImage ($APPIMAGE)
    AppImage(PathBuf),
    /// Sandboxed Flatpak package (read-only container)
    Flatpak,
    /// System-wide read-only package (/usr/bin/smartconnect-gui)
    SystemReadOnly(PathBuf),
    /// Unknown or development build
    Unknown(PathBuf),
}

impl InstallTarget {
    pub fn label(&self) -> &'static str {
        match self {
            Self::SharunShared(_) => "Portable bundle",
            Self::DirectBinary(_) => "User binary",
            Self::AppImage(_) => "AppImage",
            Self::Flatpak => "Flatpak package",
            Self::SystemReadOnly(_) => "System package",
            Self::Unknown(_) => "Custom binary",
        }
    }

    pub fn path(&self) -> PathBuf {
        match self {
            Self::SharunShared(p) | Self::DirectBinary(p) | Self::AppImage(p) | Self::SystemReadOnly(p) | Self::Unknown(p) => p.clone(),
            Self::Flatpak => PathBuf::from("/app/bin/smartconnect-gui"),
        }
    }

    pub fn description(&self) -> String {
        match self {
            Self::SharunShared(p) => format!("Portable bundle ({})", format_path_display(p)),
            Self::DirectBinary(p) => format!("User binary ({})", format_path_display(p)),
            Self::AppImage(p) => format!("AppImage ({})", format_path_display(p)),
            Self::Flatpak => "Flatpak package (managed via flatpak update)".to_owned(),
            Self::SystemReadOnly(p) => format!("System package ({})", format_path_display(p)),
            Self::Unknown(p) => format!("Custom ({})", format_path_display(p)),
        }
    }

    pub fn is_updatable(&self) -> bool {
        matches!(self, Self::SharunShared(_) | Self::DirectBinary(_) | Self::AppImage(_))
    }
}

pub fn format_path_display(p: &Path) -> String {
    if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
        if let Ok(rel) = p.strip_prefix(&home) {
            return format!("~/{}", rel.display());
        }
    }
    p.display().to_string()
}

pub fn middle_truncate(text: &str, max_len: usize) -> String {
    if text.len() <= max_len || max_len < 8 {
        return text.to_owned();
    }
    let left = (max_len - 3) / 2;
    let right = max_len - 3 - left;
    format!("{}…{}", &text[..left], &text[text.len() - right..])
}

/// Detects the target path to update depending on runtime environment.
pub fn detect_install_target() -> InstallTarget {
    if std::env::var_os("FLATPAK_ID").is_some() || Path::new("/.flatpak-info").exists() {
        return InstallTarget::Flatpak;
    }
    if let Some(appimage) = std::env::var_os("APPIMAGE") {
        let p = PathBuf::from(appimage);
        if p.exists() {
            return InstallTarget::AppImage(p);
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        // Detect Sharun bundle layout: <root>/bin/<exe> with <root>/shared/bin/<exe>
        if let Some(parent) = exe.parent() {
            if let Some(opt_root) = parent.parent() {
                if let Some(file_name) = exe.file_name() {
                    let shared_bin = opt_root.join("shared/bin").join(file_name);
                    if shared_bin.is_file() {
                        if is_writable(&shared_bin) {
                            return InstallTarget::SharunShared(shared_bin);
                        } else {
                            return InstallTarget::SystemReadOnly(shared_bin);
                        }
                    }
                }
            }
        }
        if is_writable(&exe) {
            return InstallTarget::DirectBinary(exe);
        } else {
            return InstallTarget::SystemReadOnly(exe);
        }
    }
    InstallTarget::Unknown(PathBuf::new())
}

fn is_writable(path: &Path) -> bool {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    if let Ok(c) = CString::new(path.as_os_str().as_bytes()) {
        unsafe { libc::access(c.as_ptr(), libc::W_OK) == 0 }
    } else {
        false
    }
}

/// Parses semver numbers `(major, minor, patch)` from a version string.
pub fn parse_semver(s: &str) -> Option<(u64, u64, u64)> {
    let clean = s.trim().trim_start_matches(|c: char| c == 'v' || c == 'V');
    let mut parts = clean.split('.');
    let major = parts.next()?.parse::<u64>().ok()?;
    let minor = parts.next().unwrap_or("0").split('-').next()?.parse::<u64>().ok()?;
    let patch = parts.next().unwrap_or("0").split('-').next()?.parse::<u64>().ok()?;
    Some((major, minor, patch))
}

/// Extracts semver from either tag_name or release name.
pub fn extract_release_version(tag: &str, name: &str) -> Option<(u64, u64, u64)> {
    if let Some(v) = parse_semver(tag) {
        return Some(v);
    }
    for word in name.split_whitespace() {
        if let Some(v) = parse_semver(word) {
            return Some(v);
        }
    }
    None
}

/// Compares current version against candidate release.
pub fn is_newer_version(current: &str, tag: &str, name: &str) -> bool {
    let current_semver = parse_semver(current);
    let release_semver = extract_release_version(tag, name);

    match (current_semver, release_semver) {
        (Some((c_maj, c_min, c_pat)), Some((r_maj, r_min, r_pat))) => {
            (r_maj, r_min, r_pat) > (c_maj, c_min, c_pat)
        }
        (Some(_), None) => false,
        _ => {
            let norm_curr = current.trim_start_matches(|c: char| c == 'v' || c == 'V');
            let norm_tag = tag.trim_start_matches(|c: char| c == 'v' || c == 'V');
            norm_curr != norm_tag && !tag.is_empty()
        }
    }
}

/// Selects the best compatible release asset for the target install environment.
pub fn select_asset<'a>(release: &'a ReleaseInfo, target: &InstallTarget) -> Option<&'a ReleaseAsset> {
    match target {
        InstallTarget::AppImage(_) => {
            release.assets.iter().find(|a| a.name.to_lowercase().ends_with(".appimage"))
        }
        InstallTarget::SharunShared(_) | InstallTarget::DirectBinary(_) => {
            release.assets.iter().find(|a| a.name == "smartconnect-gui")
                .or_else(|| release.assets.iter().find(|a| a.name.contains("smartconnect-gui") && !a.name.ends_with(".sha256")))
                .or_else(|| release.assets.iter().find(|a| a.name.ends_with(".tar.gz") || a.name.ends_with(".AppImage")))
        }
        _ => release.assets.first(),
    }
}

/// Runs curl to fetch text from GitHub API.
fn curl_json(url: &str) -> Result<String, String> {
    let out = Command::new("curl")
        .args([
            "-fsSL",
            "-m",
            "10",
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
        return Err(if stderr.is_empty() { format!("HTTP error {}", out.status) } else { stderr });
    }
    String::from_utf8(out.stdout).map_err(|e| format!("invalid UTF-8 response: {e}"))
}

/// Queries GitHub for the latest release.
pub fn fetch_latest_release() -> Result<ReleaseInfo, String> {
    match curl_json(GITHUB_LATEST_URL) {
        Ok(json_text) => {
            if let Ok(info) = serde_json::from_str::<ReleaseInfo>(&json_text) {
                return Ok(info);
            }
        }
        Err(_) => {}
    }

    let list_json = curl_json(GITHUB_RELEASES_URL)?;
    let releases: Vec<ReleaseInfo> = serde_json::from_str(&list_json)
        .map_err(|e| format!("could not parse releases from GitHub: {e}"))?;

    releases.into_iter().next().ok_or_else(|| "No releases found on GitHub".to_owned())
}

/// Cache folder for update downloads.
fn updates_cache_dir() -> PathBuf {
    PathBuf::from(std::env::var_os("HOME").unwrap_or_default())
        .join(".cache/smartconnect-linux/updates")
}

/// Applies the downloaded update file atomically into the target binary path.
pub fn apply_update(downloaded_temp: &Path, target: &InstallTarget) -> Result<(), String> {
    let dest = match target {
        InstallTarget::SharunShared(p) | InstallTarget::DirectBinary(p) | InstallTarget::AppImage(p) => p,
        InstallTarget::Flatpak => return Err("Updates inside Flatpak are managed by Flatpak".into()),
        InstallTarget::SystemReadOnly(p) => return Err(format!("Cannot write to system directory {}", p.display())),
        InstallTarget::Unknown(p) => p,
    };

    if let Some(parent) = dest.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(downloaded_temp)
        .map_err(|e| format!("Failed to read downloaded file metadata: {e}"))?
        .permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(downloaded_temp, perms)
        .map_err(|e| format!("Failed to set executable permissions: {e}"))?;

    std::fs::rename(downloaded_temp, dest)
        .map_err(|e| format!("Failed to atomically replace executable {}: {e}", dest.display()))?;

    Ok(())
}

/// Restarts the application cleanly.
pub fn restart_application() {
    if let Ok(exe) = std::env::current_exe() {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let _ = Command::new(exe).args(args).spawn();
    }
    std::process::exit(0);
}

// ── State Representation for Software Update Dialog ─────────────────────────

#[derive(Clone, Debug)]
pub enum UpdateDialogState {
    Checking,
    UpToDate {
        checked_caption: String,
    },
    UpdateAvailable {
        release: ReleaseInfo,
        matched_asset: Option<ReleaseAsset>,
    },
    Downloading {
        fraction: f64,
        cur_mb: f64,
        total_mb: f64,
    },
    ReadyToRestart,
    Error {
        message: String,
        details: String,
    },
}

fn center_tile_child(widget: &impl IsA<gtk::Widget>) {
    widget.set_halign(gtk::Align::Center);
    widget.set_valign(gtk::Align::Center);
    widget.set_hexpand(true);
    widget.set_vexpand(true);
}

fn kv_row(label: &str, value_widget: &impl IsA<gtk::Widget>) -> gtk::Box {
    let row = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(SPACE_3)
        .valign(gtk::Align::Center)
        .css_classes(["sc-dialog-row"])
        .build();
    let l = gtk::Label::builder().label(label).xalign(0.0).css_classes(["sc-dialog-row-label"]).build();
    let s = gtk::Box::builder().hexpand(true).build();
    row.append(&l);
    row.append(&s);
    row.append(value_widget.as_ref());
    row
}

/// Builds the redesigned Software Update Dialog for a specific state.
pub fn build_update_dialog_for_state(
    dialog: Option<adw::Dialog>,
    state: &UpdateDialogState,
    target: &InstallTarget,
    current_version: &str,
    on_check_again: Rc<dyn Fn()>,
    on_update_now: Rc<dyn Fn()>,
    on_cancel_download: Rc<dyn Fn()>,
    on_restart: Rc<dyn Fn()>,
) -> DialogPrimitive {
    let prim = match dialog {
        Some(d) => DialogPrimitive::with_dialog(d, DialogSize::Sm, "Software update", None),
        None => DialogPrimitive::new(DialogSize::Sm, "Software update", None),
    };

    // 1. Status block (left-aligned)
    let status_block = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(SPACE_4)
        .valign(gtk::Align::Center)
        .build();

    let status_tile = gtk::Box::builder()
        .width_request(48)
        .height_request(48)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .css_classes(["sc-dialog-status-tile"])
        .build();

    let text_stack = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(SPACE_1)
        .hexpand(true)
        .build();

    let headline_lbl = gtk::Label::builder()
        .xalign(0.0)
        .css_classes(["sc-dialog-headline"])
        .build();

    let subtitle_lbl = gtk::Label::builder()
        .xalign(0.0)
        .wrap(true)
        .css_classes(["sc-dialog-sub"])
        .build();

    text_stack.append(&headline_lbl);
    text_stack.append(&subtitle_lbl);
    status_block.append(&status_tile);
    status_block.append(&text_stack);

    prim.body_box.append(&status_block);

    // 2. Details Surface (ONE Surface with 44px key-value rows)
    let surface = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .css_classes(["sc-dialog-surface"])
        .build();

    // Row 1: Installed version
    let cur_v_lbl = gtk::Label::builder()
        .label(current_version)
        .css_classes(["sc-dialog-row-value"])
        .build();
    surface.append(&kv_row("Installed version", &cur_v_lbl));

    // In UpdateAvailable state: Installed -> New comparison row
    if let UpdateDialogState::UpdateAvailable { release, .. } = state {
        let new_ver_box = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).spacing(SPACE_2).build();
        let old_badge = gtk::Label::builder().label(current_version).css_classes(["sc-version-pill", "dim"]).build();
        let arr = ModernIcon::new(IconKind::ChevronRight, 11);
        let new_badge = gtk::Label::builder().label(release.tag_name.trim_start_matches('v')).css_classes(["sc-version-pill", "accent"]).build();
        new_ver_box.append(&old_badge);
        new_ver_box.append(arr.widget());
        new_ver_box.append(&new_badge);
        surface.append(&kv_row("Latest version", &new_ver_box));
    }

    // Row 2: Source
    let source_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(SPACE_2)
        .valign(gtk::Align::Center)
        .build();
    let source_lbl = gtk::Label::builder().label("GitHub releases").css_classes(["sc-dialog-row-value"]).build();
    let (source_link_btn, _) = icon_button(IconKind::ExternalLink, 12, "Open releases page on GitHub");
    source_link_btn.set_size_request(32, 32);
    source_link_btn.add_css_class("sc-btn-ghost");
    let d_weak = prim.dialog.downgrade();
    source_link_btn.connect_clicked(move |_| {
        let url = format!("https://github.com/{GITHUB_REPO}/releases");
        if let Some(dlg) = d_weak.upgrade() {
            let win = dlg.root().and_downcast::<gtk::Window>();
            gtk::UriLauncher::new(&url).launch(win.as_ref(), gio::Cancellable::NONE, |_| {});
        }
    });
    source_box.append(&source_lbl);
    source_box.append(&source_link_btn);
    surface.append(&kv_row("Source", &source_box));

    // Row 3: Install type
    let install_type_lbl = gtk::Label::builder()
        .label(target.label())
        .css_classes(["sc-dialog-row-value"])
        .build();
    surface.append(&kv_row("Install type", &install_type_lbl));

    // Row 4: Location (Stacked layout: label above, value below with middle-truncated monospace path + copy button)
    let loc_row = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(SPACE_1)
        .css_classes(["sc-dialog-row-stacked"])
        .build();
    let loc_top = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).build();
    let loc_label = gtk::Label::builder().label("Location").xalign(0.0).css_classes(["sc-dialog-row-label"]).build();
    loc_top.append(&loc_label);

    let loc_bottom = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(SPACE_2)
        .valign(gtk::Align::Center)
        .build();
    let full_path = format_path_display(&target.path());
    let trunc_path = middle_truncate(&full_path, 34);
    let path_lbl = gtk::Label::builder()
        .label(&trunc_path)
        .tooltip_text(&full_path)
        .xalign(0.0)
        .hexpand(true)
        .css_classes(["sc-dialog-path-val"])
        .build();

    let (copy_btn, _) = icon_button(IconKind::Copy, 13, "Copy path");
    copy_btn.set_size_request(32, 32);
    copy_btn.add_css_class("sc-btn-ghost");
    let fp = full_path.clone();
    let lr = prim.live_region.clone();
    let btn_weak = copy_btn.downgrade();
    copy_btn.connect_clicked(move |_| {
        if let Some(d) = gdk::Display::default() {
            d.clipboard().set_text(&fp);
        }
        lr.set_label("Copied path to clipboard");
        if let Some(b) = btn_weak.upgrade() {
            b.set_tooltip_text(Some("Copied!"));
            let b_done = b.clone();
            glib::timeout_add_local_once(Duration::from_millis(1500), move || {
                b_done.set_tooltip_text(Some("Copy path"));
            });
        }
    });

    loc_bottom.append(&path_lbl);
    loc_bottom.append(&copy_btn);
    loc_row.append(&loc_top);
    loc_row.append(&loc_bottom);
    surface.append(&loc_row);

    prim.body_box.append(&surface);

    // 3. State-specific content & footer configuration
    let caption_lbl = gtk::Label::builder()
        .xalign(0.0)
        .css_classes(["sc-dialog-footer-caption"])
        .build();
    prim.footer_left_caption.append(&caption_lbl);

    match state {
        UpdateDialogState::Checking => {
            let spin = adw::Spinner::builder().width_request(24).height_request(24).build();
            center_tile_child(&spin);
            status_tile.append(&spin);
            headline_lbl.set_label("Checking for updates…");
            subtitle_lbl.set_label("Querying GitHub releases repository…");
            caption_lbl.set_label("Checking…");

            let check_again_btn = secondary_button("Check again");
            check_again_btn.set_sensitive(false);
            let done_btn = secondary_button("Done");
            let d_weak = prim.dialog.downgrade();
            done_btn.connect_clicked(move |_| {
                if let Some(d) = d_weak.upgrade() { d.close(); }
            });

            prim.footer_actions.append(&check_again_btn);
            prim.footer_actions.append(&done_btn);
        }
        UpdateDialogState::UpToDate { checked_caption } => {
            let chk = ModernIcon::new(IconKind::Check, 24);
            center_tile_child(chk.widget());
            status_tile.append(chk.widget());
            headline_lbl.set_label("Smart Connect is up to date");
            subtitle_lbl.set_label(&format!("Version {current_version} is the latest version available."));
            caption_lbl.set_label(checked_caption);

            let check_again_btn = secondary_button("Check again");
            let on_ca = on_check_again.clone();
            check_again_btn.connect_clicked(move |_| on_ca());

            let done_btn = primary_button("Done");
            let d_weak = prim.dialog.downgrade();
            done_btn.connect_clicked(move |_| {
                if let Some(d) = d_weak.upgrade() { d.close(); }
            });

            prim.footer_actions.append(&check_again_btn);
            prim.footer_actions.append(&done_btn);
        }
        UpdateDialogState::UpdateAvailable { release, matched_asset } => {
            let arrow = ModernIcon::new(IconKind::ArrowDown, 24);
            center_tile_child(arrow.widget());
            status_tile.append(arrow.widget());
            headline_lbl.set_label(&format!("Version {} is available", release.tag_name));
            let pub_date = if release.published_at.len() >= 10 { &release.published_at[..10] } else { "recently" };
            subtitle_lbl.set_label(&format!("Published {pub_date} · GitHub"));
            caption_lbl.set_label("Update available");

            // Release notes scrolled box
            let notes_box = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(SPACE_2).build();
            let notes_text = format_changelog_text(&release.body);
            let notes_lbl = gtk::Label::builder()
                .label(&notes_text)
                .xalign(0.0)
                .wrap(true)
                .wrap_mode(gtk::pango::WrapMode::WordChar)
                .css_classes(["sc-dialog-notes-text"])
                .build();
            let notes_scroller = gtk::ScrolledWindow::builder()
                .hscrollbar_policy(gtk::PolicyType::Never)
                .propagate_natural_height(true)
                .max_content_height(160)
                .min_content_height(80)
                .css_classes(["sc-dialog-notes-box"])
                .child(&notes_lbl)
                .build();
            notes_box.append(&notes_scroller);

            let notes_url = release.html_url.clone();
            let view_notes_btn = ghost_button("View release notes");
            view_notes_btn.set_size_request(-1, 32);
            let d_weak = prim.dialog.downgrade();
            view_notes_btn.connect_clicked(move |_| {
                if let Some(dlg) = d_weak.upgrade() {
                    let win = dlg.root().and_downcast::<gtk::Window>();
                    gtk::UriLauncher::new(&notes_url).launch(win.as_ref(), gio::Cancellable::NONE, |_| {});
                }
            });
            notes_box.append(&view_notes_btn);
            prim.body_box.append(&notes_box);

            let later_btn = secondary_button("Later");
            let d_weak = prim.dialog.downgrade();
            later_btn.connect_clicked(move |_| {
                if let Some(d) = d_weak.upgrade() { d.close(); }
            });

            let update_now_btn = primary_button("Update now");
            if !target.is_updatable() {
                update_now_btn.set_sensitive(false);
                update_now_btn.set_label("Managed by system");
            } else if matched_asset.is_none() {
                update_now_btn.set_sensitive(false);
                update_now_btn.set_label("No binary asset");
            } else {
                let on_un = on_update_now.clone();
                update_now_btn.connect_clicked(move |_| on_un());
            }

            prim.footer_actions.append(&later_btn);
            prim.footer_actions.append(&update_now_btn);
        }
        UpdateDialogState::Downloading { fraction, cur_mb, total_mb } => {
            let arrow = ModernIcon::new(IconKind::ArrowDown, 24);
            center_tile_child(arrow.widget());
            status_tile.append(arrow.widget());
            headline_lbl.set_label("Downloading update…");
            subtitle_lbl.set_label("Streaming release binary from GitHub…");
            caption_lbl.set_label("Downloading…");

            let progress_box = gtk::Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .spacing(SPACE_2)
                .build();
            let progress_bar = gtk::ProgressBar::builder()
                .fraction(*fraction)
                .css_classes(["sc-dialog-progress"])
                .build();
            let dl_stat_lbl = gtk::Label::builder()
                .label(format!("Downloading… {:.1} MB / {:.1} MB ({}%)", cur_mb, total_mb, (*fraction * 100.0) as u32))
                .xalign(0.0)
                .css_classes(["caption", "sc-muted"])
                .build();
            progress_box.append(&progress_bar);
            progress_box.append(&dl_stat_lbl);
            prim.body_box.append(&progress_box);

            let cancel_btn = secondary_button("Cancel");
            let on_cancel = on_cancel_download.clone();
            cancel_btn.connect_clicked(move |_| on_cancel());
            prim.footer_actions.append(&cancel_btn);
        }
        UpdateDialogState::ReadyToRestart => {
            let chk = ModernIcon::new(IconKind::Check, 24);
            center_tile_child(chk.widget());
            status_tile.append(chk.widget());
            headline_lbl.set_label("Update ready to install");
            subtitle_lbl.set_label("Restart Smart Connect to finish updating.");
            caption_lbl.set_label("Ready");

            let later_btn = secondary_button("Later");
            let d_weak = prim.dialog.downgrade();
            later_btn.connect_clicked(move |_| {
                if let Some(d) = d_weak.upgrade() { d.close(); }
            });

            let restart_btn = primary_button("Restart to finish updating");
            let on_r = on_restart.clone();
            restart_btn.connect_clicked(move |_| on_r());

            prim.footer_actions.append(&later_btn);
            prim.footer_actions.append(&restart_btn);
        }
        UpdateDialogState::Error { message, details } => {
            status_tile.add_css_class("danger");
            let err_ic = ModernIcon::new(IconKind::WifiOff, 24);
            center_tile_child(err_ic.widget());
            status_tile.append(err_ic.widget());
            headline_lbl.set_label("Couldn't check for updates");
            subtitle_lbl.set_label(message);
            caption_lbl.set_label("Check failed");

            let notice = notice_banner(IconKind::WifiOff, message);
            prim.body_box.append(&notice);

            if !details.is_empty() {
                let exp = gtk::Expander::builder().label("Details").expanded(false).build();
                let det_lbl = gtk::Label::builder()
                    .label(details)
                    .xalign(0.0)
                    .wrap(true)
                    .css_classes(["caption", "sc-muted", "monospace"])
                    .margin_top(SPACE_2)
                    .build();
                exp.set_child(Some(&det_lbl));
                prim.body_box.append(&exp);
            }

            let try_again_btn = secondary_button("Try again");
            let on_ca = on_check_again.clone();
            try_again_btn.connect_clicked(move |_| on_ca());

            let done_btn = secondary_button("Done");
            let d_weak = prim.dialog.downgrade();
            done_btn.connect_clicked(move |_| {
                if let Some(d) = d_weak.upgrade() { d.close(); }
            });

            prim.footer_actions.append(&try_again_btn);
            prim.footer_actions.append(&done_btn);
        }
    }

    prim
}

/// Presents the modern libadwaita Software Update dialog.
pub fn present_update_dialog(parent: &impl IsA<gtk::Widget>, _auto_mode: bool) {
    let target = detect_install_target();
    let current_version = env!("CARGO_PKG_VERSION").to_owned();

    let state_cell: Rc<RefCell<UpdateDialogState>> = Rc::new(RefCell::new(UpdateDialogState::Checking));
    let current_release: Rc<RefCell<Option<ReleaseInfo>>> = Rc::default();
    let current_asset: Rc<RefCell<Option<ReleaseAsset>>> = Rc::default();
    let is_download_cancelled = Rc::new(Cell::new(false));

    // Dialog instance
    let dialog = adw::Dialog::builder()
        .content_width(DIALOG_WIDTH_SM)
        .can_close(true)
        .css_classes(["sc-dialog"])
        .build();

    let redraw: Rc<RefCell<Option<Rc<dyn Fn()>>>> = Rc::default();

    let redraw_fn: Rc<dyn Fn()> = {
        let dialog = dialog.clone();
        let state_cell = state_cell.clone();
        let target = target.clone();
        let current_version = current_version.clone();
        let current_release = current_release.clone();
        let current_asset = current_asset.clone();
        let redraw = redraw.clone();
        let is_download_cancelled = is_download_cancelled.clone();

        Rc::new(move || {
            let current_state = state_cell.borrow().clone();

            let on_check_again: Rc<dyn Fn()> = {
                let state_cell = state_cell.clone();
                let current_version = current_version.clone();
                let target = target.clone();
                let current_release = current_release.clone();
                let current_asset = current_asset.clone();
                let redraw = redraw.clone();
                Rc::new(move || {
                    *state_cell.borrow_mut() = UpdateDialogState::Checking;
                    if let Some(f) = redraw.borrow().as_ref() { f(); }

                    let state_cell = state_cell.clone();
                    let current_version = current_version.clone();
                    let target = target.clone();
                    let current_release = current_release.clone();
                    let current_asset = current_asset.clone();
                    let redraw = redraw.clone();

                    glib::spawn_future_local(async move {
                        let res = gio::spawn_blocking(fetch_latest_release)
                            .await
                            .unwrap_or_else(|_| Err("Background task crashed".to_owned()));

                        match res {
                            Ok(rel) => {
                                let is_new = is_newer_version(&current_version, &rel.tag_name, &rel.name);
                                if is_new {
                                    let matched = select_asset(&rel, &target).cloned();
                                    *current_release.borrow_mut() = Some(rel.clone());
                                    *current_asset.borrow_mut() = matched.clone();
                                    *state_cell.borrow_mut() = UpdateDialogState::UpdateAvailable {
                                        release: rel,
                                        matched_asset: matched,
                                    };
                                } else {
                                    *state_cell.borrow_mut() = UpdateDialogState::UpToDate {
                                        checked_caption: "Checked just now".to_owned(),
                                    };
                                }
                            }
                            Err(e) => {
                                *state_cell.borrow_mut() = UpdateDialogState::Error {
                                    message: "Network offline or GitHub unreachable.".to_owned(),
                                    details: e,
                                };
                            }
                        }
                        if let Some(f) = redraw.borrow().as_ref() { f(); }
                    });
                })
            };

            let on_update_now: Rc<dyn Fn()> = {
                let current_asset = current_asset.clone();
                let state_cell = state_cell.clone();
                let redraw = redraw.clone();
                let target = target.clone();
                let is_download_cancelled = is_download_cancelled.clone();

                Rc::new(move || {
                    let Some(asset) = current_asset.borrow().clone() else { return };
                    is_download_cancelled.set(false);
                    let total_mb = if asset.size > 0 { asset.size as f64 / 1_048_576.0 } else { 5.5 };
                    *state_cell.borrow_mut() = UpdateDialogState::Downloading {
                        fraction: 0.05,
                        cur_mb: 0.2,
                        total_mb,
                    };
                    if let Some(f) = redraw.borrow().as_ref() { f(); }

                    let cache_dir = updates_cache_dir();
                    let _ = std::fs::create_dir_all(&cache_dir);
                    let download_file = cache_dir.join(format!("{}.download", asset.name));
                    let _ = std::fs::remove_file(&download_file);

                    let url = asset.browser_download_url.clone();
                    let dest_path = download_file.clone();
                    let total_size = asset.size;
                    let target_done = target.clone();
                    let state_cell_done = state_cell.clone();
                    let redraw_done = redraw.clone();
                    let is_canc = is_download_cancelled.clone();

                    let (prog_tx, prog_rx) = async_channel::bounded::<f64>(32);

                    let dl_task = gio::spawn_blocking(move || -> Result<(), String> {
                        let out = Command::new("curl")
                            .args(["-fL", "-m", "180", "-o", dest_path.to_str().unwrap_or_default(), &url])
                            .stdin(Stdio::null())
                            .output()
                            .map_err(|e| format!("curl execution failed: {e}"))?;

                        if !out.status.success() {
                            let err = String::from_utf8_lossy(&out.stderr).trim().to_owned();
                            return Err(if err.is_empty() { format!("Download failed ({})", out.status) } else { err });
                        }
                        Ok(())
                    });

                    let dest_for_poll = download_file.clone();
                    let poll_tx = prog_tx.clone();
                    let poll_source = glib::timeout_add_local(Duration::from_millis(150), move || {
                        if let Ok(meta) = std::fs::metadata(&dest_for_poll) {
                            let cur_len = meta.len();
                            if total_size > 0 {
                                let frac = (cur_len as f64) / (total_size as f64);
                                let _ = poll_tx.try_send(frac.clamp(0.05, 0.98));
                            }
                        }
                        glib::ControlFlow::Continue
                    });

                    let state_cell_prog = state_cell_done.clone();
                    let redraw_prog = redraw_done.clone();
                    let is_canc_prog = is_canc.clone();
                    glib::spawn_future_local(async move {
                        while let Ok(frac) = prog_rx.recv().await {
                            if is_canc_prog.get() { break; }
                            *state_cell_prog.borrow_mut() = UpdateDialogState::Downloading {
                                fraction: frac,
                                cur_mb: frac * total_mb,
                                total_mb,
                            };
                            if let Some(f) = redraw_prog.borrow().as_ref() { f(); }
                        }
                    });

                    glib::spawn_future_local(async move {
                        let res = dl_task.await.unwrap_or_else(|_| Err("Download thread panicked".to_owned()));
                        poll_source.remove();

                        if is_canc.get() {
                            let _ = std::fs::remove_file(&download_file);
                            return;
                        }

                        match res {
                            Ok(()) => {
                                match apply_update(&download_file, &target_done) {
                                    Ok(()) => {
                                        *state_cell_done.borrow_mut() = UpdateDialogState::ReadyToRestart;
                                    }
                                    Err(e) => {
                                        *state_cell_done.borrow_mut() = UpdateDialogState::Error {
                                            message: "Failed to apply update to system.".to_owned(),
                                            details: e,
                                        };
                                    }
                                }
                            }
                            Err(e) => {
                                *state_cell_done.borrow_mut() = UpdateDialogState::Error {
                                    message: "Download failed.".to_owned(),
                                    details: e,
                                };
                            }
                        }
                        if let Some(f) = redraw_done.borrow().as_ref() { f(); }
                    });
                })
            };

            let on_cancel_download: Rc<dyn Fn()> = {
                let is_download_cancelled = is_download_cancelled.clone();
                let on_check_again = on_check_again.clone();
                Rc::new(move || {
                    is_download_cancelled.set(true);
                    on_check_again();
                })
            };

            let on_restart: Rc<dyn Fn()> = Rc::new(move || {
                restart_application();
            });

            build_update_dialog_for_state(
                Some(dialog.clone()),
                &current_state,
                &target,
                &current_version,
                on_check_again,
                on_update_now,
                on_cancel_download,
                on_restart,
            );
        })
    };

    *redraw.borrow_mut() = Some(redraw_fn.clone());
    redraw_fn();

    dialog.present(Some(parent));

    // Trigger initial check
    {
        let state_cell = state_cell.clone();
        let target = target.clone();
        let current_version = current_version.clone();
        let current_release = current_release.clone();
        let current_asset = current_asset.clone();
        let redraw = redraw.clone();

        glib::spawn_future_local(async move {
            let res = gio::spawn_blocking(fetch_latest_release)
                .await
                .unwrap_or_else(|_| Err("Background task crashed".to_owned()));

            match res {
                Ok(rel) => {
                    let is_new = is_newer_version(&current_version, &rel.tag_name, &rel.name);
                    if is_new {
                        let matched = select_asset(&rel, &target).cloned();
                        *current_release.borrow_mut() = Some(rel.clone());
                        *current_asset.borrow_mut() = matched.clone();
                        *state_cell.borrow_mut() = UpdateDialogState::UpdateAvailable {
                            release: rel,
                            matched_asset: matched,
                        };
                    } else {
                        *state_cell.borrow_mut() = UpdateDialogState::UpToDate {
                            checked_caption: "Checked just now".to_owned(),
                        };
                    }
                }
                Err(e) => {
                    *state_cell.borrow_mut() = UpdateDialogState::Error {
                        message: "Network offline or GitHub unreachable.".to_owned(),
                        details: e,
                    };
                }
            }
            if let Some(f) = redraw.borrow().as_ref() { f(); }
        });
    }
}

/// Formats raw markdown or HTML changelog body into clean readable text for GTK.
pub fn format_changelog_text(raw: &str) -> String {
    let mut out = String::new();
    let mut in_code_block = false;

    let text = raw.replace("<span>", "")
        .replace("</span>", "")
        .replace("<p>", "")
        .replace("</p>", "\n\n")
        .replace("<h1>", "## ")
        .replace("</h1>", "\n")
        .replace("<h2>", "### ")
        .replace("</h2>", "\n")
        .replace("<ul>", "")
        .replace("</ul>", "")
        .replace("<li>", "• ")
        .replace("</li>", "\n")
        .replace("<pre><code>", "```\n")
        .replace("</code></pre>", "```\n");

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        if in_code_block {
            out.push_str("    ");
            out.push_str(line);
            out.push('\n');
            continue;
        }
        if trimmed.starts_with("### ") {
            out.push('\n');
            out.push_str(trimmed.trim_start_matches('#').trim());
            out.push_str(":\n");
        } else if trimmed.starts_with("## ") || trimmed.starts_with("# ") {
            out.push_str("\n▶ ");
            out.push_str(trimmed.trim_start_matches('#').trim());
            out.push('\n');
        } else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            out.push_str("  • ");
            out.push_str(&trimmed[2..]);
            out.push('\n');
        } else if !trimmed.is_empty() {
            out.push_str(trimmed);
            out.push('\n');
        }
    }

    if out.trim().is_empty() {
        out = "Performance enhancements, stability improvements, and bug fixes.".to_owned();
    }
    out.trim().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semver_parsing_handles_prefixes_and_suffixes() {
        assert_eq!(parse_semver("0.1.0"), Some((0, 1, 0)));
        assert_eq!(parse_semver("v0.1.0"), Some((0, 1, 0)));
        assert_eq!(parse_semver("V1.2.3"), Some((1, 2, 3)));
        assert_eq!(parse_semver("v0.2.0-beta.1"), Some((0, 2, 0)));
        assert_eq!(parse_semver("2.0"), Some((2, 0, 0)));
        assert_eq!(parse_semver("invalid"), None);
    }

    #[test]
    fn version_comparison_logic() {
        assert!(is_newer_version("0.1.0", "v0.2.0", "Smart Connect v0.2.0"));
        assert!(is_newer_version("0.1.0", "v0.1.1", "Bugfix"));
        assert!(is_newer_version("0.1.0", "v1.0.0", "Major"));
        assert!(!is_newer_version("0.1.0", "v0.1.0", "Same"));
        assert!(!is_newer_version("0.2.0", "v0.1.0", "Older"));

        assert!(!is_newer_version("0.1.0", "Major_update", "Smart Connect for Linux v0.1.0"));
        assert!(is_newer_version("0.1.0", "Major_update", "Smart Connect for Linux v0.2.0"));
    }

    #[test]
    fn changelog_formatter_strips_html_and_formats_bullets() {
        let sample = "<h1>Title</h1><p>Description</p><ul><li>Feature 1</li><li>Feature 2</li></ul>";
        let formatted = format_changelog_text(sample);
        assert!(formatted.contains("Title"));
        assert!(formatted.contains("Description"));
        assert!(formatted.contains("• Feature 1"));
    }

    #[test]
    fn asset_selection_matches_target() {
        let release = ReleaseInfo {
            tag_name: "v0.2.0".into(),
            name: "Release 0.2.0".into(),
            body: "".into(),
            published_at: "".into(),
            html_url: "".into(),
            assets: vec![
                ReleaseAsset { name: "smartconnect-gui".into(), size: 5000, browser_download_url: "http://example.com/bin".into() },
                ReleaseAsset { name: "SmartConnect.AppImage".into(), size: 8000, browser_download_url: "http://example.com/appimage".into() },
            ],
        };

        let direct = InstallTarget::DirectBinary(PathBuf::from("/usr/local/bin/smartconnect-gui"));
        assert_eq!(select_asset(&release, &direct).unwrap().name, "smartconnect-gui");

        let appimage = InstallTarget::AppImage(PathBuf::from("/home/user/SmartConnect.AppImage"));
        assert_eq!(select_asset(&release, &appimage).unwrap().name, "SmartConnect.AppImage");
    }

    #[test]
    fn middle_truncate_test() {
        let path = "~/.local/opt/smartconnect-linux/shared/bin/smartconnect-gui";
        let t = middle_truncate(path, 30);
        assert!(t.contains('…'));
        assert!(t.len() <= 32);
    }
}
