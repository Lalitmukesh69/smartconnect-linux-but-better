# GitHub Release Draft

> Use the fields below when creating the release at
> `https://github.com/Lalitmukesh69/smartconnect-linux-but-better/releases/new`
>
> - **Tag:** `v0.1.0-ui.1` *(or `v0.1.0` if this is the first public tag)*
> - **Target:** `main` (`e879e5a`)
> - **Release Title:** `Smart Connect UI v0.1.0-ui.1 — Apple Pro Dark Monochrome (Phase 1)`
> - **Pre-release:** Checked *(recommended while Phases 2–5 are in progress; uncheck if publishing as stable)*
> - **Attach Binary Assets:**
>   - `Smart_Connect-0.1.0-x86_64.flatpak` *(Single-file Flatpak bundle for one-click installation on Fedora, Ubuntu, Arch, etc.)*
>   - `Smart_Connect-0.1.0-anylinux-x86_64.AppImage` *(Portable AppImage)*
>   - `target/release/smartconnect-gui` *(Standalone compiled GTK4 binary)*

---

<!-- Copy everything below this line into the "Describe this release" box on GitHub -->

# Smart Connect UI `v0.1.0-ui.1` — Apple Pro Dark Monochrome (Phase 1)

First compiled release of the **Apple Pro Dark Monochrome** UI redesign for `smartconnect-gui`, built against live `smartconnectd` telemetry and helpers while keeping 100% real pairing, screen mirroring, Cross Control, file sharing, and notification sync behavior intact.

---

## ✨ What's New

### 🎨 Monochrome Dark Design System & Helvetica Typography
- **High-Contrast Dark Palette:** Deep obsidian canvas (`#050506`), `280 px` split sidebar (`#0A0A0C`), Bento cards (`#0E0E11`), pure-white active pills and primary CTAs, and subtle hairline borders rendered above custom user GTK themes (`STYLE_PROVIDER_PRIORITY_USER + 50`).
- **Global Helvetica Typography:** Switched the entire GTK4/libadwaita UI, popovers, dialogs, Cairo canvases, and setup/display hints to `"Helvetica", "Nimbus Sans", "Liberation Sans", sans-serif` with tabular numerals (`tnum`) for crisp telemetry alignment.
- **Custom Vector Iconography (`ModernIcon`):** Replaced host-theme symbolic PNG icons across navigation, telemetry tiles, popovers, and Bento actions with crisp Cairo-drawn stroke icons and a stacked white brand emblem.

### 🧭 280 px Split-View Shell & Sticky Workspace Header
- **Adaptive Split Navigation:** Replaced the top tab switcher with an `AdwOverlaySplitView` sidebar (auto-collapses below `780sp`) featuring grouped **Workspace** (`Phone`, `Apps`) and **Continuity** (`Display`, `Files`, `Notifications`) views, live app/unread counters, endpoint device switcher popover, and footer status CTA.
- **Sticky `56 px` Workspace Header:** Displays live `<Device> / <View>` breadcrumbs, status indicator, **Command Palette (`Ctrl+K`)** trigger, **About Developer** button, **Telemetry** button, primary **Connect / Disconnect** CTA, and overflow application menu.
- **Persistent Window State:** Remembers window size (default `1120×760`), maximized state, active view, background-close preference, and optional local device nickname (`deviceAlias`) in `gui.json`.

### 📱 Redesigned Phone Bento Overview
- **Endpoint Identity Hero Bento:** Centered card (`1020 px` max width) showing live wallpaper thumbnail (when available), hardware model badge, inline local nickname rename editor (`Edit`), quick **Mirror** toggle, **Telemetry**, **Connect / Disconnect**, and **Pair new phone…** controls.
- **4-Column Telemetry Bento Grid:** Live **Battery** (5-segment indicator), **Wi-Fi Link** (4-bar signal meter), **Cellular** (4-bar meter), and **Bluetooth** cards driven strictly by live/cached `smartconnectd` state.
- **Quick Continuity Row:** Dedicated **Cross Control** quick-launch card plus symmetrical **Recent App Stream**, **Mount Location**, and **Shared Directory** Bento cards wired directly to Apps, Files, and Folder Share actions.

### ⚡ Command Palette, Telemetry & GitHub-Backed About Developer
- **Spotlight Command Palette (`Ctrl+K`):** Instant fuzzy search across navigation tabs, daemon/mirror/Cross-Control/mount/pairing/preferences actions, keyboard shortcuts, and streamed phone apps (`smartconnect-mirror --list-apps`) with `Up`/`Down`/`Enter`/`Esc` keyboard navigation.
- **Endpoint Telemetry Dialog:** Live readouts for `phoneAddress`, `pcAddress`, device model, link state, cached app/icon counts, FUSE mount path, and shared directory state.
- **About Developer Modal (`@Lalitmukesh69`):** Displays developer bio, company/location, external links, public repository/follower/following stat tiles, avatar, and featured GitHub repositories—synced asynchronously from `api.github.com` into `$XDG_CACHE_HOME/smartconnect-linux/` with offline cache fallback and manual refresh. Reachable from the header User icon, device menu (`About Developer…`), overflow menu, **Preferences → Help & About**, or `Ctrl+K`.

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| `Ctrl+K` | Open Spotlight Command Palette |
| `Alt+1`–`5` / `Ctrl+1`–`5` | Switch between Phone, Apps, Display, Files, Notifications |
| `Ctrl+F` | Focus Apps search |
| `Ctrl+Shift+P` | Pair new phone |
| `Ctrl+,` | Open Preferences |
| `Ctrl+?` | Show Keyboard Shortcuts |
| `Ctrl+W` | Close / hide window to tray |
| `Ctrl+Q` | Quit GUI |

---

## 📦 Release Asset & Checksum

This release ships the standalone optimized GUI frontend binary (`smartconnect-gui`, `x86_64` Linux ELF, stripped):

| File | Architecture | Size | SHA-256 |
|---|---|---|---|
| `smartconnect-gui` | Linux `x86_64` | `5.5 MiB` (`5,766,864` bytes) | `7c178838d90d22e992f42996654f4e24702b8e2257eebb1e6015cb79d3ddd404` |

Verify after downloading:

```sh
echo "7c178838d90d22e992f42996654f4e24702b8e2257eebb1e6015cb79d3ddd404  smartconnect-gui" | sha256sum -c -
```

---

## 🚀 Installation & Usage

> **Note:** `smartconnect-gui` is the GTK4/libadwaita frontend binary. It communicates over the local socket with `smartconnectd` and launches the Smart Connect helper binaries (`smartconnect-mirror`, `smartconnect-crosscontrol`, `smartconnect-display`, `smartconnect-files`, `smartconnect-fex`) from `PATH` or alongside the binary.

### Option A — Drop-in GUI Upgrade (if `smartconnectd` & helpers are already installed)

```sh
chmod +x smartconnect-gui
# Preview right away without overwriting ~/.local/bin:
./smartconnect-gui

# Or replace your installed frontend in ~/.local/bin:
install -m755 smartconnect-gui ~/.local/bin/smartconnect-gui
```

### Option B — Build Full 8-Binary Workspace or AppImage from Source

```sh
git clone https://github.com/Lalitmukesh69/smartconnect-linux-but-better.git
cd smartconnect-linux-but-better

# Full native workspace install (requires GTK4, libadwaita, GStreamer, D-Bus, and Wayland dev headers):
./packaging/install.sh --enable

# Or build a self-contained AppImage via Podman/Docker:
DOCKER=podman ./packaging/appimage/make-appimage.sh
```

See `docs/installation.md` for full distribution dependency lists and firewall rules (`52020/tcp` + `mdns`).

---

## 🧩 System Requirements (`smartconnect-gui` Binary)

- **OS / Arch:** 64-bit Linux (`x86_64`), Wayland or X11
- **Runtime Libraries:**
  - `glibc >= 2.39`
  - `gtk4 >= 4.18` *(built against `4.22.5`)*
  - `libadwaita >= 1.7` *(built against `1.9.4`)*
- **Optional:** `curl` on `PATH` for refreshing live GitHub stats/avatar inside *About Developer* (works offline using cached/bundled profile data)
- **Backend:** Running `smartconnectd` user service and Smart Connect helper binaries installed on `PATH`

---

## 🗺️ Redesign Scope & Roadmap

- **Included in this release (Phase 1):** Global dark monochrome theme, Helvetica typography, split-view sidebar shell, Command Palette (`Ctrl+K`), Endpoint Telemetry dialog, GitHub-backed About Developer modal, and full **Phone** Bento overview.
- **Upcoming phases:** Individual monochrome restyles for **Apps Launchpad** (Phase 2), **Display & Cross Control Studio** (Phase 3), **Files & Share** (Phase 4), and **Notifications Relay Center** (Phase 5)—all existing page controls currently work inside the dark shell.

---

## ✅ Verification

- `cargo test --release -p smartconnect-gui` — **37 passed, 0 failed** (**0** compiler warnings)
- `cargo build --release -p smartconnect-gui` — **0** compiler warnings
- Verified live on Fedora Linux against an active `smartconnectd` session (`connected=true`, `paired=true`) with clean UI layout and zero Adwaita overflow warnings.

---

## 🙏 Credits & License

- **UI Redesign & Maintainer:** Lalit mukesh ([@Lalitmukesh69](https://github.com/Lalitmukesh69))
- **License:** GNU GPLv3 (`LICENSE`)

