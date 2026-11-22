<div align="center">

<img src="packaging/dev.smartconnect.Linux.png" alt="Smart Connect for Linux Logo" width="96" height="96">

# Smart Connect for Linux

**Seamlessly connect your Motorola phone to your Linux PC — without root, extra phone apps, or developer mode.**

[![GitHub Release](https://img.shields.io/github/v/release/Lalitmukesh69/smartconnect-linux-but-better?style=flat-square&color=blue)](https://github.com/Lalitmukesh69/smartconnect-linux-but-better/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg?style=flat-square)](LICENSE)
[![Platform: Linux](https://img.shields.io/badge/Platform-Linux%20%7C%20Wayland-informational?style=flat-square)](docs/requirements.md)
[![Rust](https://img.shields.io/badge/Built%20with-Rust%20%7C%20GTK4-orange.svg?style=flat-square)](Cargo.toml)

</div>

---

**Smart Connect for Linux** enables your Linux computer to pair directly with the official, stock **Motorola Smart Connect** app (formerly *Ready For*). 

Your phone communicates with your Linux desktop just like it does with a Windows PC. Once paired with a quick QR code scan, your phone and computer work together seamlessly over your local Wi-Fi and Bluetooth.

---

## ✨ Key Features

- 📱 **Notifications & Quick Reply** — View phone notifications on your desktop with one-click direct replies and action buttons (mark as read, archive, pause).
- 📋 **Two-Way Clipboard** — Copy text on your phone and paste it on your PC (and vice versa) instantly and securely.
- 🪟 **App Streaming & Screen Mirroring** — Launch individual phone apps in their own desktop windows, or mirror your full phone screen with smooth mouse and keyboard control.
- 🖥️ **Wireless Second Monitor** — Use your phone or tablet as an extended or duplicated desktop display with touch support and an on-screen keyboard.
- 🖱️ **Cross Control** — Move your mouse cursor across your monitor's edge right onto your phone screen to type and navigate seamlessly.
- 📷 **HD Phone Webcam** — Transform your phone's high-resolution front or rear cameras into a crisp webcam for Zoom, Teams, Meet, and OBS.
- 📁 **Files & Storage** — Browse phone storage directly in your Linux file manager (`~/Phone`), or share PC folders with your phone.
- 🎨 **Modern Desktop App & Tray** — Refined 2026 dark UI with a responsive 4-column app grid, command palette (`Ctrl+K`), and live phone telemetry (battery %, charging, Wi-Fi, cellular, Bluetooth) in your system tray.

---

## 🚀 Quick Start

### 1. Installation

Choose the package that works best for your Linux distribution:

#### Option A: Flatpak (Recommended)
Works on any Linux distribution with Flatpak installed:
```bash
flatpak install Smart_Connect-0.1.0-x86_64.flatpak
flatpak run dev.smartconnect.Linux
```

#### Option B: AppImage (Portable)
No installation required — download, make executable, and run:
```bash
chmod +x Smart_Connect-0.1.0-x86_64.AppImage
./Smart_Connect-0.1.0-x86_64.AppImage
```

#### Option C: Build from Source
```bash
git clone https://github.com/Lalitmukesh69/smartconnect-linux-but-better.git
cd smartconnect-linux-but-better
./packaging/install.sh --enable
```
*See the full [Installation Guide](docs/installation.md) for dependencies on Arch, Fedora, and Ubuntu/Debian.*

---

### 2. Pairing Your Phone (3 Simple Steps)

1. **Open Smart Connect on Linux** and click **Pair new phone…** to generate a secure pairing QR code.
2. **Open Smart Connect on your Motorola phone**, tap **+** → **Scan QR code**, and point your camera at the PC screen.
3. **Connected!** Your phone and Linux PC are now paired. Future connections happen automatically when both devices are on the same Wi-Fi network.

---

## 📚 Documentation

Detailed documentation is available in the [`docs/`](docs/) directory:

- 📊 **[Feature Status & Compatibility](docs/feature-status.md)** — Verified feature matrix and tested phone models.
- ⚙️ **[System & Network Requirements](docs/requirements.md)** — Compositor details (GNOME, KDE, Hyprland, niri) and firewall ports.
- 📖 **[User Guide & Features](docs/usage.md)** — Step-by-step feature walkthroughs and CLI commands.
- 🏗️ **[Architecture & Crates](docs/components.md)** — Overview of daemon architecture and helper programs.
- 🔧 **[Troubleshooting](docs/troubleshooting.md)** — Solutions for common connection and firewall questions.
- 🔒 **[Security & Privacy](docs/security.md)** — Encryption standards, credential management, and privacy safeguards.
- 🔌 **[Control Socket API](docs/control-socket-api.md)** — JSON-RPC socket API for custom integrations and scripting.

---

## ⚠️ Disclaimer

Smart Connect for Linux is an independent, unofficial open-source project. It is not affiliated with, endorsed by, or sponsored by Motorola Mobility LLC or Lenovo. Motorola, Smart Connect, Ready For, and associated trademarks belong to their respective owners and are used strictly for compatibility identification.

---

## 📄 License

Distributed under the [MIT License](LICENSE).
