# GTK app: shell, navigation, preferences, shortcuts and About

## What
Polished the top-level window shell and navigation ergonomics of `smartconnect-gui`:

- **Default window width & persistence (`WindowPrefs` in `gui.json`):** Default width raised from `600` to `920` (with `760` height) so the wide header `AdwViewSwitcher` fits without immediately triggering the `max-width: 780sp` bottom-bar breakpoint. Window geometry (`width`, `height`, `maximized`), closing behaviour (`hideOnClose`), and `lastTab` are persisted and sanitized on load.
- **Primary application menu (`open-menu-symbolic`):** Added a standard right-side header `MenuButton` holding *Pair new phone…*, *Preferences*, *Keyboard Shortcuts*, *About Smart Connect*, and *Quit*. Keeping *Quit* reachable from the window guarantees desktops without an AppIndicator/StatusNotifierHost tray (such as default GNOME) still have a direct exit path.
- **Device pill stays visible:** `DeviceMenu.button` now stays visible on the left of the header bar even when `smartconnectd` is stopped and no phone is cached yet, styled with `.flat.sc-device-btn`.
- **Keyboard shortcuts & Shortcuts dialog:** Added window `GAction`s + accelerators (`Alt+1..5` / `Ctrl+1..5` for tabs, `Ctrl+F` to jump to Apps search, `Ctrl+,` Preferences, `Ctrl+?` Keyboard Shortcuts, `Ctrl+Shift+P` Pair, `Ctrl+W` close/hide, `Ctrl+Q` Quit) plus a Shortcuts dialog built with `AdwPreferencesDialog` (avoiding `AdwShortcutsDialog` which requires libadwaita 1.8 beyond the pinned `v1_7` feature flag, and deprecated `GtkShortcutsWindow`).
- **Expanded Preferences & About:** Preferences now holds *General* (*Start at login*, *Keep running in background* with an adaptive subtitle noting whether a tray host was found), *Service & Diagnostics* (live `smartconnectd` status + *Start* button, app icon cache count + *Clear*, configuration folder path + *Open* via `GtkFileLauncher`), and *Help & About* links opening Shortcuts and `AdwAboutDialog`.
- **Theme-native CSS refresh:** Updated `gui/src/main.rs` CSS with libadwaita's `--accent-bg-color` focus outline on `.sc-mode:checked`, smooth transitions on tiles/modes, pill device button styling, and `.sc-kbd` keycap sizing without hardcoded colours.

## How
- [`gui/src/prefs.rs`](../gui/src/prefs.rs): added `WindowPrefs` (`Default` 920×760, `hide_on_close: true`, `last_tab: "phone"`), `WindowPrefs::sanitized()`, `config_dir()`, and `save_gui()` (writes only `gui.json` without rewriting `display.json`).
- [`gui/src/apps.rs`](../gui/src/apps.rs): added `AppsPage::focus_search()`.
- [`gui/src/main.rs`](../gui/src/main.rs): updated structural/variable-only CSS (`.sc-card`, `.sc-mode:checked`, `.sc-tile`, `.sc-device-btn`, `.sc-kbd`).
- [`gui/src/window.rs`](../gui/src/window.rs): wired `build_app_menu()`, `setup_actions()`, `save_window_prefs()`, `on_tab_visible()`, `open_preferences()`, `open_shortcuts()`, `open_about()`, `format_display_path()`, icon-cache cleanup helpers, and `close_request` handling `hide_on_close`.

## Why
- **Tray-less desktops:** On stock GNOME without an AppIndicator extension, hiding on close and keeping *Quit* solely in ksni left users without a UI quit control. Exposing *Quit* (`Ctrl+Q`) in the top-right menu plus the *Keep running in background* switch solves this while preserving background-tray mode by default.
- **`AdwPreferencesDialog` for shortcuts:** Keeping the crate's `adw` `v1_7` feature floor maintains compatibility with packaged distributions; `.card.sc-kbd` suffix labels inside `AdwActionRow`s render theme-native keycaps without deprecated widgets.
