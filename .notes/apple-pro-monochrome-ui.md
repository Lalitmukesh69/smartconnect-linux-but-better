# GTK app: Apple Pro Dark Monochrome redesign (Phase 1 — Shell, Palette, Phone)

## What
Began a staged redesign of `smartconnect-gui` to follow an Apple Pro Dark Monochrome / Sidebar Split-View Bento aesthetic while keeping 100% real `smartconnectd` and helper state (no fabricated mock telemetry):

- **Design system ([`gui/src/theme.rs`](../gui/src/theme.rs)):** Enforces dark polarity (`adw::ColorScheme::ForceDark`) and overrides libadwaita CSS variables (`--window-bg-color: #050506`, `--sidebar-bg-color: #0A0A0C`, `--card-bg-color: #0E0E11`, `--accent-bg-color: #FFFFFF`, `--accent-fg-color: #050506`), adding Bento surface rules (`.sc-bento`, `.sc-stat-tile`, `.sc-sidebar`, `.sc-nav-item.active` pure-white pill, `.sc-btn-primary`, `.sc-btn-secondary`, monochrome `.sc-seg-on`/`.sc-seg-off`, glowing `.sc-dot-online`, `.sc-kbd` keycaps). Avoids GTK4-unsupported web CSS (`backdrop-filter`, CSS flex `gap`). Also provides a custom Cairo `brand_mark()` (stacked white icon), `format_display_path()` (`$HOME` → `~`), and monochrome status helpers.
- **Spotlight Command Palette & Endpoint Telemetry ([`gui/src/palette.rs`](../gui/src/palette.rs)):**
  - `Ctrl+K` (or header / sidebar quick-command triggers) opens an `adw::Dialog` command palette searching navigation views, primary connect/mirror/mount/Cross-Control/pair actions, preferences/shortcuts, and live streamed phone apps (`smartconnect-mirror --list-apps`), with keyboard navigation (`Up`/`Down`/`Enter`/`Esc`) that keeps typing focus inside the search entry.
  - **Endpoint Telemetry** dialog presents truthful endpoint specs (`phoneAddress`, `pcAddress`, hardware model, link status, battery/radio summary, cached app/icon counts, FUSE mount path/state, and shared folder path/state).
- **272 px Split-View Shell & Sticky Workspace Header ([`gui/src/window.rs`](../gui/src/window.rs)):**
  - Replaced top `AdwViewSwitcher` + `AdwViewSwitcherBar` with `AdwOverlaySplitView` (`272 px` pinned sidebar collapsing via `AdwBreakpoint` below `780sp`), housing the brand mark + `Ctrl+K` pill in `gtk::WindowHandle`, device endpoint switcher popover (`Pair new phone…`, `Preferences`, `Connect/Disconnect`, `Start service`), grouped `WORKSPACE` (`Phone`, `Apps`, `Display`, `Files`) and `CONTINUITY` (`Notifications`) nav buttons with live `Apps` count and unread notification badges, and footer link summary + primary CTA.
  - Added a `56 px` sticky workspace `HeaderBar` (`title-widget` cleared so Adwaita never draws a centered duplicate title) showing sidebar collapse toggle when narrow, `<Device> / <View>` breadcrumb, status pill, `Quick Command… Ctrl+K`, Telemetry icon button, Connect/Disconnect CTA, and `open-menu-symbolic` menu.
  - Raised default `WindowPrefs` size in [`gui/src/prefs.rs`](../gui/src/prefs.rs) from `920×760` to `1120×760` and added optional `deviceAlias` (`device_alias`) persisted locally in `gui.json`.
- **Phone Bento Overview (`PhonePage` in [`gui/src/window.rs`](../gui/src/window.rs)):**
  - Endpoint Identity Hero Bento with live wallpaper thumbnail (or 54×54 monochrome monogram/icon tile), link status pill, hardware model badge, inline local nickname rename editor (`document-edit-symbolic` → `deviceAlias`), Mirror screen / Telemetry / Connect / Pair controls, and unread notifications shortcut.
  - 4-column Telemetry Bento grid (`Battery` with 5-segment progress bar, `Wi-Fi`, `Cellular` with 4 monochrome signal bars, `Bluetooth`) driven strictly by live/cached daemon data.
  - `Cross Control · Edge Continuity` quick-launch Bento reflecting live `CrossControlPage` active mode badges and edge anchor, and clicking straight into the Display studio.
  - 3-column `System Continuity` Bento row wired to [`AppsPage`](../gui/src/apps.rs), [`FilesPage`](../gui/src/files.rs), and [`ShareSection`](../gui/src/share.rs) change callbacks (`Recent App Stream`, `Mount Location`, `Shared Directory`).

## How
- [`gui/src/theme.rs`](../gui/src/theme.rs), [`gui/src/palette.rs`](../gui/src/palette.rs), [`gui/src/main.rs`](../gui/src/main.rs), [`gui/src/daemon.rs`](../gui/src/daemon.rs) (`pc_address` / `phone_address` deserialization), [`gui/src/prefs.rs`](../gui/src/prefs.rs) (`WindowPrefs` 1120×760 + `device_alias`), [`gui/src/window.rs`](../gui/src/window.rs), plus summary/signal hooks on [`gui/src/apps.rs`](../gui/src/apps.rs), [`gui/src/crosscontrol.rs`](../gui/src/crosscontrol.rs), [`gui/src/files.rs`](../gui/src/files.rs), and [`gui/src/share.rs`](../gui/src/share.rs).

## Notes for Remaining Phases
- `Apps`, `Display / Cross Control`, `Files`, and `Notifications` page internals keep existing layouts inside the dark monochrome shell until Phases 2–5 restyle each page individually.
- Fedora's default Adwaita symbolic icon theme does not include `layers-symbolic`, so the sidebar brand mark renders via Cairo (`brand_mark()`).
