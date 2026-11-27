<div align="center">

<img src="packaging/dev.smartconnect.Linux.png" alt="Smart Connect for Linux Logo" width="96" height="96">

# Smart Connect for Linux

Connect a Motorola phone to a Linux desktop using the stock Motorola Smart Connect app, without root, extra phone apps, or developer mode.

[![GitHub Release](https://img.shields.io/github/v/release/Lalitmukesh69/smartconnect-linux-but-better?style=flat-square&color=blue)](https://github.com/Lalitmukesh69/smartconnect-linux-but-better/releases)
[![Installation Guide](https://img.shields.io/badge/Docs-Installation%20Website-0066cc?style=flat-square&logo=safari&logoColor=white)](https://lalitmukesh69.github.io/smartconnect-linux-but-better/)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL%20v3-blue.svg?style=flat-square)](LICENSE)
[![Platform: Linux](https://img.shields.io/badge/Platform-Linux%20%7C%20Wayland-informational?style=flat-square)](docs/requirements.md)
[![Rust](https://img.shields.io/badge/Built%20with-Rust%20%7C%20GTK4-orange.svg?style=flat-square)](Cargo.toml)

</div>

Smart Connect for Linux pairs directly with the stock Motorola Smart Connect app (formerly Ready For) over local Wi-Fi and Bluetooth. Your phone sees your Linux PC the same way it sees a Windows PC running Motorola's companion app.

See the [installation website](https://lalitmukesh69.github.io/smartconnect-linux-but-better/) for Fedora, Ubuntu, Debian, and Arch Linux commands and copyable install prompts.

## Features

- Phone notifications on the desktop, with inline quick replies and action buttons (mark as read, archive, pause).
- Two-way plain-text clipboard sync between phone and PC.
- Full-screen phone mirroring and individual phone apps streamed into separate desktop windows, with mouse and keyboard input.
- Wireless second monitor mode (extended or duplicated display) with touch input and an on-screen keyboard.
- Cross Control lets you move your mouse pointer past a monitor edge onto your phone screen and type there.
- Phone front or rear camera exposed as a Linux webcam for Zoom, Teams, Meet, and OBS.
- Phone storage mounted in your Linux file manager (`~/Phone`), plus sharing local PC folders back to the phone.
- GTK4 desktop app with a 4-column app grid, `Ctrl+K` command palette, and system tray status showing battery, charging state, Wi-Fi, cellular signal, and Bluetooth.

## Installation

GitHub Releases does not include prebuilt `.flatpak` or `.AppImage` bundles yet. Releases currently publish only the standalone `smartconnect-gui` binary, which expects `smartconnectd` and the helper binaries to already be installed. Install from source for a complete setup, or build a Flatpak or AppImage locally from this repository.

### Option 1: Install from source (recommended)

1. Install the build dependencies (Rust 1.88 or newer, GTK4, libadwaita, GStreamer with WebRTC/libnice, Avahi, BlueZ, `wl-clipboard`, and `fuse3`) listed for your distribution in the [installation guide](docs/installation.md#packages) or on the [installation website](https://lalitmukesh69.github.io/smartconnect-linux-but-better/).
2. Enable Avahi and Bluetooth:
   ```bash
   sudo systemctl enable --now avahi-daemon bluetooth
   ```
3. Clone the repository and run the installer, which builds all eight binaries, installs them under `~/.local`, adds the desktop launcher, and starts the `smartconnectd` user service:
   ```bash
   git clone https://github.com/Lalitmukesh69/smartconnect-linux-but-better.git
   cd smartconnect-linux-but-better
   ./packaging/install.sh --enable
   ```
4. Allow TCP port `52020` and mDNS through your firewall (`2049/tcp` is also needed if you share PC folders to your phone):
   ```bash
   # firewalld (Fedora)
   sudo firewall-cmd --permanent --add-port=52020/tcp
   sudo firewall-cmd --permanent --add-service=mdns
   sudo firewall-cmd --reload
   ```
   ```bash
   # ufw (Ubuntu / Debian)
   sudo ufw allow 52020/tcp
   sudo ufw allow 5353/udp
   sudo ufw reload
   ```

### Option 2: Build and install a Flatpak locally

Requires `flatpak-builder`, `podman`, or `docker` inside a clone of the repository:

```bash
./packaging/flatpak/make-flatpak.sh
flatpak install dist/Smart_Connect-0.2.0-x86_64.flatpak
flatpak run dev.smartconnect.Linux
```

### Option 3: Build an AppImage locally

Requires `docker` (or run with `DOCKER=podman`):

```bash
./packaging/appimage/make-appimage.sh
chmod +x dist/Smart_Connect-*.AppImage
./dist/Smart_Connect-*.AppImage
```

## Pair your phone

1. Open Smart Connect from your app launcher (or run `smartconnect-gui`) and click **Pair new phone...** to show a QR code.
2. Open the Motorola Smart Connect app on your phone, tap **+**, choose **Scan QR code**, and point the camera at your screen.
3. Wait for the pairing dialog to close. Once paired, both devices reconnect automatically when they are on the same Wi-Fi network.

## Documentation

- [Installation guide](docs/installation.md) for Arch, Fedora, Ubuntu, and Debian package lists, systemd service details, and firewall setup.
- [Feature status and compatibility](docs/feature-status.md) for tested phone models, Wayland compositors, and what is verified.
- [System and network requirements](docs/requirements.md) for compositor support, ports, and optional kernel modules (`v4l2loopback`, `vkms`).
- [User guide](docs/usage.md) for pairing, mirroring, second display, Cross Control, webcam, and CLI commands.
- [Architecture and components](docs/components.md) for how `smartconnectd` and the helper binaries fit together.
- [Software updates](docs/updater.md) for the in-app GitHub release checker and executable replacement.
- [Troubleshooting](docs/troubleshooting.md) for connection, mDNS, Bluetooth, and firewall fixes.
- [Security and privacy](docs/security.md) for TLS certificates, credential storage, and clipboard redaction.
- [Control socket API](docs/control-socket-api.md) for the JSON-RPC socket at `$XDG_RUNTIME_DIR/smartconnectd.sock`.

## Disclaimer

Smart Connect for Linux is an independent, unofficial open-source project. It is not affiliated with, endorsed by, or sponsored by Motorola Mobility LLC or Lenovo. Motorola, Smart Connect, Ready For, and associated trademarks belong to their respective owners and are used only to identify compatibility.

## License

Licensed under the [GNU General Public License v3.0](LICENSE).
