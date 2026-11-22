# Feature Status & Compatibility Matrix

This document tracks the implementation and verification status of every Smart Connect feature on Linux.

> **Key**:  
> ✅ **Verified**: Tested and working on a physical Motorola phone.  
> 🟡 **Implemented**: Code written and tested with test harnesses / mocks; awaiting physical phone verification.  
> ❌ **Not Supported**: Planned or reverse-engineered, not yet fully functional.

---

## Tested Hardware & Environments

- **Phones**: Motorola edge 50 series, edge 60 stylus running stock Smart Connect (versions 09.0.3+).
- **Desktops / Compositors**:
  - GNOME 46+ (Wayland, InputCapture portal, Mutter virtual monitors)
  - KDE Plasma 6+ (KWin Wayland)
  - Hyprland / sway (wlroots-based, headless outputs)
  - niri (with `vkms` virtual monitor module)
  - Noctalia Shell & standard FreeDesktop notification daemons

---

## Feature Matrix

### 1. Connection & Pairing

| Feature | Status | Technical Details |
|---|---|---|
| **Pairing via QR Code** | ✅ | Scanned from the stock Motorola Smart Connect app. PC registers with name and TLS fingerprint. |
| **Wi-Fi & Bluetooth LE Reconnect** | ✅ | Dual-channel reconnect: listens on Wi-Fi (mDNS) and sends Bluetooth LE wake packets when phone app is backgrounded. |
| **Automatic Reconnection** | ✅ | Exponential backoff with LAN awareness. Automatically reconnects when network comes back up or Bluetooth activates. |
| **Feature Tiles on Phone** | ✅ | Publishes capabilities to the phone: App Streaming, Cross Control, PC Lock, Hotspot, and Webcam. |
| **PC Lock Tile** | 🟡 | Triggers desktop lock via logind, Noctalia lock screen, or custom `--lock-command`. |
| **Hotspot Tile** | 🟡 | NetworkManager D-Bus integration to toggle phone tethering hotspot. |

### 2. Notifications, Messages & Clipboard

| Feature | Status | Technical Details |
|---|---|---|
| **Phone Notifications & Quick Reply** | ✅ | Bridges phone notifications to desktop FreeDesktop daemon. Inline reply window sends text directly to phone. |
| **Notification Action Buttons** | 🟡 | Executes remote actions ("Mark as read", "Archive", "Pause") via ephemeral background app streams. See [Notification Actions](notification-actions.md). |
| **Per-App Notification Mute** | 🟡 | Mute noisy apps from notification banners, GTK app, or Noctalia panel. Stored in desktop prefs. |
| **Two-Way Text Clipboard** | ✅ | Synchronizes plain text both ways using `wl-clipboard` / data-control. Sensitive passwords (from password managers) are filtered. |
| **Image & File Clipboard** | ❌ | Wire protocol reverse-engineered. See [File Exchange Protocol](protocol/file-exchange.md) and [Clipboard Protocol](protocol/clipboard.md). |
| **SMS Management** | 🟡 | Read conversation threads, send SMS, and mark as read via control socket API. |

### 3. Screen Mirroring & App Streaming

| Feature | Status | Technical Details |
|---|---|---|
| **Full Screen Mirroring** | ✅ | Low-latency WebRTC H.264 stream with pointer/mouse input forwarding. |
| **Keyboard & Special Keys** | 🟡 | Keyboard navigation: `Esc` for Back, `Ctrl+Alt+H` for Home, `Ctrl+Alt+R` for App Switcher. |
| **Individual App Streaming** | ✅ | Streams individual phone apps into separate native Linux desktop windows. |
| **High-Res App Icons** | ✅ | Automatically downloaded from the phone on connection and cached in `~/.cache/smartconnect-linux/app-icons/`. |
| **Dynamic App Window Resize** | 🟡 | Opt-in (`SMARTCONNECT_MIRROR_RESIZE=1`): phone adapts its virtual display dimensions to the desktop window. |
| **Mobile Desktop Tile** | 🟡 | Launches Motorola's desktop mode interface on the PC. |

### 4. Cross Control (Seamless Input Sharing)

| Feature | Status | Technical Details |
|---|---|---|
| **Edge-Triggered Mouse Sharing** | ✅ | Moving mouse past screen edge smoothly transfers pointer control to the phone. |
| **Cross Control on GNOME** | ✅ | Implemented via the modern `xdg-desktop-portal` InputCapture portal. |
| **Cross Control on wlroots / niri** | ✅ | Uses `wlr-layer-shell` invisible edge barrier strips and pointer constraints. |
| **Visual Edge Position Editor** | 🟡 | Interactive arrangement card in GTK app to position phone along left, right, top, or bottom edges. |

### 5. Phone as a Second Monitor

| Feature | Status | Technical Details |
|---|---|---|
| **Extend Desktop** | ✅ | Phone acts as a high-resolution wireless secondary monitor. |
| **Duplicate / Mirror Desktop** | ✅ | Clones focused or selected desktop output onto the phone screen. |
| **Display Scaling** | ✅ | High-DPI 200% default scaling matches Motorola Windows companion behavior. |
| **On-Screen Virtual Keyboard** | ✅ | Launches `wvkbd` or native compositor keyboard on the phone display for direct touch typing. |
| **Touch Gestures** | 🟡 | Tap to click, drag to select, long-press for right-click, two-finger scrolling. |
| **Follow Phone Rotation** | 🟡 | Adapts virtual display resolution when rotating phone between portrait and landscape. |
| **PC Audio Redirection** | 🟡 | Streams desktop audio output to phone speakers via WebRTC audio track. |

### 6. Phone Camera as HD Webcam

| Feature | Status | Technical Details |
|---|---|---|
| **V4L2 Loopback Device** | ✅ | Registers as a standard virtual camera device (`/dev/video*`) visible in Zoom, Teams, Meet, OBS, Discord. |
| **PipeWire Native Camera** | ✅ | Exposes webcam directly through PipeWire camera portal for modern Flatpak apps without loopback kernel module. |
| **Front / Rear Camera Toggle** | ✅ | Switch cameras seamlessly in phone app without breaking video stream. |
| **Quality & Bitrate Profiles** | 🟡 | Configurable profiles: Low (2 Mbps), Medium (4 Mbps), High (8 Mbps), Max (15 Mbps). |

### 7. Files & Storage Sharing

| Feature | Status | Technical Details |
|---|---|---|
| **Mount Phone Storage** | ✅ | Mounts `/sdcard` as a user-space FUSE folder (`~/Phone`) via File Station protocol (NFSv3 over mutual TLS). |
| **Quick Access Folders** | 🟡 | One-click shortcuts for Camera, Screenshots, Downloads, and Documents. |
| **Share PC Folder to Phone** | ✅ | Exposes local PC folder (e.g. `~`) to the phone's Smart Connect Files tab and Android file picker. |
| **File Push / Pull CLI** | ❌ | Protocol analyzed in [File Exchange Protocol](protocol/file-exchange.md); pending CLI command integration. |

### 8. Desktop App & System Tray

| Feature | Status | Technical Details |
|---|---|---|
| **Modern GTK4 / Libadwaita UI** | ✅ | 2026 dark standard desktop UI with responsive 4-column app grid and phone screen mockup. |
| **Command Palette (`Ctrl+K`)** | ✅ | Fast fuzzy search for device commands, tabs, and developer links. |
| **System Tray Telemetry** | ✅ | StatusNotifierItem tooltip displays live battery %, charging status, Wi-Fi link, cellular signal, and Bluetooth state. |
| **System Tray Quick Actions** | ✅ | Context menu shortcuts to Phone, Apps, Display, Files, Notifications, Connect, Disconnect, and Pair. |
