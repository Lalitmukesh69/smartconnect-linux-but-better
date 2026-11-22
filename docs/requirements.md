# System & Hardware Requirements

This page details the hardware, kernel, desktop environment, and network requirements for running Smart Connect for Linux.

---

## 1. Phone Requirements

- **Supported Phone**: Any Motorola phone with the stock **Smart Connect** app preinstalled or installed from Google Play (formerly known as *Ready For*).
- **Network**: The phone must be connected to the same Wi-Fi network and local subnet as your PC.
- **No Modifications Required**: No root access, no custom ROM, no developer options, and no third-party APKs needed.

---

## 2. PC System Requirements

### Desktop Environment & Compositor
- **Wayland Compositor**: Wayland is strongly recommended for all modern features:
  - **GNOME** (version 45+) — Uses the FreeDesktop `InputCapture` portal for Cross Control and Mutter virtual monitors for display expansion.
  - **KDE Plasma** (version 6+) — Supported via KWin screen casting and virtual display outputs.
  - **Hyprland / sway** — Supported natively via `wlr-layer-shell`, `wlr-virtual-pointer`, and headless outputs.
  - **niri** — Supported with `wlr-layer-shell`; second monitor extension requires the `vkms` kernel module or an HDMI dummy plug.

### Core Software Packages
- **D-Bus & systemd**: A standard systemd user session (`systemctl --user`).
- **mDNS / DNS-SD**: `avahi-daemon` enabled and running for local device discovery.
- **Bluetooth**: `bluez` service active with Bluetooth LE support for waking backgrounded phone apps.
- **Clipboard**: `wl-clipboard` (Wayland) or Xwayland clipboard bridge.
- **Media Engine**: GStreamer 1.22+ with `webrtcbin`, `libnice`, `gst-plugins-good`, and `gst-plugins-bad`.
- **FUSE**: `fuse3` (specifically the `fusermount3` binary) for mounting phone storage.

---

## 3. Network & Firewall Configuration

Smart Connect communicates locally over your private Wi-Fi network. Ensure your firewall (e.g. `firewalld` or `ufw`) allows the following traffic within your home/local network zone:

| Port / Protocol | Direction | Purpose | Required By |
|---|---|---|---|
| **TCP 52020** | Inbound | Control channel (TLS WebSocket) | Core daemon (`smartconnectd`) |
| **UDP 5353** | Inbound / Outbound | Device discovery (mDNS / Avahi) | Discovery service |
| **TCP 2049** (or custom) | Inbound | Folder sharing to phone (File Station) | `smartconnect-files serve` |
| **Ephemeral UDP ports** | Inbound / Outbound | WebRTC audio & video streaming | Mirroring, Display & Webcam |

### Firewall Setup Examples

#### firewalld (Fedora / RHEL / openSUSE)
```bash
sudo firewall-cmd --permanent --add-port=52020/tcp
sudo firewall-cmd --permanent --add-service=mdns
sudo firewall-cmd --permanent --add-port=2049/tcp  # Optional: for PC folder sharing
sudo firewall-cmd --reload
```

#### ufw (Ubuntu / Debian / Linux Mint)
```bash
sudo ufw allow 52020/tcp comment 'Smart Connect Daemon'
sudo ufw allow 5353/udp comment 'mDNS Avahi'
sudo ufw allow 2049/tcp comment 'Smart Connect File Sharing' # Optional
```

---

## 4. Optional Hardware & Kernel Modules

| Feature | Requirement | Fallback Behavior |
|---|---|---|
| **HD Webcam** | `v4l2loopback` kernel module (`sudo modprobe v4l2loopback exclusive_caps=1 card_label="Smart Connect Phone Camera"`) | Falls back to native PipeWire virtual camera (compatible with OBS, Firefox, and Chromium). |
| **Second Monitor on niri** | `vkms` kernel module or HDMI dummy plug | Mirroring/Duplicate display works out of the box; Extend requires a virtual KMS output. |
| **On-Screen Keyboard** | `wvkbd` installed in `PATH` | Direct touch input still works; typing on phone display uses compositor keyboard if available. |
| **Background Reconnect** | Powered-on Bluetooth LE adapter | If Bluetooth is off or unavailable, opening the Smart Connect app on the phone immediately reconnects. |
