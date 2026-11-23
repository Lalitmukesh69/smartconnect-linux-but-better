# Smart Connect for Linux — Demo Video Recording & Setup Guide

This document is your step-by-step operational script and technical runbook for recording a demo video showing how to install **Smart Connect for Linux** directly from GitHub on a clean machine.

---

## 🎬 Video Overview & Outline

| Chapter / Scene | Target Duration | Focus |
|---|---|---|
| **Scene 1: Introduction** | 0:00 – 0:45 | Introducing the project, GitHub repository, and clean slate |
| **Scene 2: Prerequisites & Services** | 0:45 – 1:45 | Installing distro packages and enabling Avahi/mDNS |
| **Scene 3: Installation from GitHub** | 1:45 – 3:15 | Cloning the repo and running `./packaging/install.sh --enable` |
| **Scene 4: Firewall Setup** | 3:15 – 4:00 | Opening port 52020 for the phone to connect |
| **Scene 5: First Launch & Service Check** | 4:00 – 4:45 | Launching from the app menu and checking systemd service |
| **Scene 6: Live Phone Pairing** | 4:45 – 5:45 | Scanning the QR code with Motorola Smart Connect |
| **Scene 7: Feature Showcase** | 5:45 – 7:00 | 2026 Dark UI, live telemetry, Command Palette, and In-App Updater |

---

## 🧹 Phase 0: Clean Slate Verification (Before Recording)

Before hitting record, ensure any leftover test builds or running services from previous installations are completely cleared:

```bash
# 1. Stop and disable any running systemd user service
systemctl --user stop smartconnectd.service 2>/dev/null || true
systemctl --user disable smartconnectd.service 2>/dev/null || true

# 2. Kill any stray daemon or GUI process
killall -9 smartconnectd smartconnect-gui 2>/dev/null || true

# 3. Clean previous binaries and desktop launchers
rm -f ~/.local/bin/smartconnect*
rm -f ~/.local/share/applications/dev.smartconnect.Linux.desktop
rm -f ~/.local/share/applications/dev.smartconnect.Display.desktop
rm -f ~/.config/systemd/user/smartconnectd.service
systemctl --user daemon-reload
update-desktop-database -q ~/.local/share/applications 2>/dev/null || true
```

> [!NOTE]
> If you want to demonstrate pairing from scratch, also remove old pairing credentials:
> ```bash
> rm -rf ~/.local/share/smartconnect-linux ~/.config/smartconnect-linux ~/.cache/smartconnect
> ```
> *(Also remove the PC from the Motorola Smart Connect app on your phone under Settings > Connected Computers).*

---

## 🎥 Scene-by-Scene Video Recording Script

### Scene 1: Introduction & GitHub Repository (0:00 – 0:45)

**On-Screen Action:**
- Show web browser open to:  
  `https://github.com/Lalitmukesh69/smartconnect-linux-but-better`
- Scroll briefly through the README, highlighting:
  - Stock Motorola Smart Connect pairing (zero phone-side modifications).
  - Modern GTK4/libadwaita dark UI.
  - Latest **v0.2.0** release badge.

**Voiceover / Talking Points:**
> *"Hey everyone! In this video, I'll walk you through how to install Smart Connect for Linux directly from GitHub on a completely fresh Linux system. Smart Connect lets your Motorola phone seamlessly connect to your Linux PC for notifications, clipboard sync, app streaming, wireless display, and mouse-sharing. Let's jump right into the terminal."*

---

### Scene 2: Prerequisites & Background Services (0:45 – 1:45)

**On-Screen Action:**
- Open terminal with a clean, readable font size.
- Run the package installation command for your distribution.

#### A. Fedora Linux (Host Distro)
```bash
sudo dnf install -y \
  git gcc pkgconf-pkg-config dbus-devel wayland-devel \
  avahi avahi-tools bluez wl-clipboard NetworkManager fuse3 \
  gstreamer1-devel gstreamer1-plugins-base-devel gstreamer1-plugins-bad-free-devel \
  gstreamer1-plugins-good gstreamer1-plugins-bad-free libnice-gstreamer1 \
  gtk4-devel libadwaita-devel
```

#### B. Ubuntu / Debian (Reference for Viewers)
```bash
sudo apt update && sudo apt install -y \
  git build-essential pkg-config libdbus-1-dev libwayland-dev \
  avahi-daemon avahi-utils bluez wl-clipboard network-manager fuse3 \
  libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev libgstreamer-plugins-bad1.0-dev \
  gstreamer1.0-plugins-base gstreamer1.0-plugins-good gstreamer1.0-plugins-bad \
  gstreamer1.0-gl gstreamer1.0-nice gstreamer1.0-libav \
  libgtk-4-dev libadwaita-1-dev
```

#### C. Arch Linux (Reference for Viewers)
```bash
sudo pacman -S --needed \
  git base-devel pkgconf rust avahi bluez dbus wl-clipboard fuse3 \
  gstreamer gst-plugins-base gst-plugins-good gst-plugins-bad libnice gst-libav \
  gtk4 libadwaita
```

#### Enable mDNS & Bluetooth Services
```bash
# Avahi provides mDNS discovery so phone and PC discover each other over Wi-Fi
sudo systemctl enable --now avahi-daemon bluetooth
```

**Voiceover / Talking Points:**
> *"The only dependencies needed are standard GTK4 and GStreamer libraries for video mirroring, and Avahi for local network discovery. Once installed, make sure the `avahi-daemon` service is enabled so your phone and PC can find each other automatically over Wi-Fi."*

---

### Scene 3: Downloading & Installing from GitHub (1:45 – 3:15)

Show the automated 1-command installation from source.

#### Method A: Full Source Installation (Recommended for Complete Experience)
This compiles and installs all 8 binaries (`smartconnectd`, `smartconnect-gui`, mirror, display, webcam, files, crosscontrol, fex), configures the systemd service, and installs the desktop launcher with hicolor icons.

```bash
# 1. Clone the repository
git clone https://github.com/Lalitmukesh69/smartconnect-linux-but-better.git
cd smartconnect-linux-but-better

# 2. Run the automated installer with service enablement
./packaging/install.sh --enable
```

**Expected Terminal Output to Highlight:**
```text
==> Checking the desktop side (hints only)
[ok] wl-copy
[ok] avahi-publish
[ok] avahi-browse
[ok] gst-inspect-1.0
[ok] fusermount3
[ok] avahi-daemon running
==> Building the workspace (release)
    Finished `release` profile [optimized] target(s)
==> Installing binaries to /home/<user>/.local/bin
==> Installing the systemd user unit to /home/<user>/.config/systemd/user
==> Installing the GTK app launcher to /home/<user>/.local/share/applications
==> Starting the systemd user service smartconnectd
```

---

#### Method B: Fast Track (Using Precompiled Release Binary)
If your viewers want an instant setup without waiting for Rust compilation, show the direct download method:

```bash
# Download stripped release binary from v0.2.0
curl -fsSL -o ~/.local/bin/smartconnect-gui \
  https://github.com/Lalitmukesh69/smartconnect-linux-but-better/releases/download/v0.2.0/smartconnect-gui

chmod +x ~/.local/bin/smartconnect-gui
```

**Voiceover / Talking Points:**
> *"The installer handles everything automatically: it compiles the optimized Rust binaries, places them into `~/.local/bin`, sets up your desktop launcher with icons, and starts the background daemon as a systemd user service."*

---

### Scene 4: Firewall Setup (3:15 – 4:00)

**On-Screen Action:**
- Show opening the required port so the phone can reach the PC.

#### For firewalld (Fedora / RHEL / openSUSE):
```bash
# 1. Check your active Wi-Fi zone
firewall-cmd --get-active-zones

# 2. Allow port 52020 (TLS WebSocket) and mDNS service
sudo firewall-cmd --permanent --add-port=52020/tcp
sudo firewall-cmd --permanent --add-service=mdns
sudo firewall-cmd --reload
```

#### For ufw (Ubuntu / Debian / Mint):
```bash
sudo ufw allow 52020/tcp comment 'Smart Connect WebSocket'
sudo ufw allow 5353/udp comment 'mDNS discovery'
sudo ufw reload
```

**Voiceover / Talking Points:**
> *"Smart Connect communicates with your phone locally over encrypted TLS WebSockets on port 52020. We open port 52020 and mDNS on our local firewall so the phone can initiate the handshake seamlessly."*

---

### Scene 5: Starting the Application & Service Verification (4:00 – 4:45)

**On-Screen Action:**
- Check the background daemon service:
```bash
systemctl --user status smartconnectd
```
- Show the service is `active (running)`.
- Open the application launcher (press Super / Windows key).
- Type **Smart Connect** and click the squircle icon, or launch via terminal:
```bash
smartconnect-gui &
```

**Voiceover / Talking Points:**
> *"You can see our `smartconnectd` background service is already active and listening. Now we can launch Smart Connect right from our desktop application menu."*

---

### Scene 6: Live Phone Pairing (4:45 – 5:45)

**On-Screen Action:**
- In the Smart Connect Linux window, click **Pair new phone…**.
- A QR code dialog appears on the PC screen.
- Cut to camera / split-screen showing your Motorola phone:
  1. Open the stock **Smart Connect** app on the phone.
  2. Tap **+** (or **Add device**) → **Scan QR Code**.
  3. Point the phone's camera at the PC monitor.
- Show the instant connection!
- The dialog closes, and the main window lights up:
  - Phone name appears (e.g. *"motorola edge"*).
  - Battery percentage with charging status.
  - Cellular signal bars & Wi-Fi indicator.

**Voiceover / Talking Points:**
> *"Now for the magic part: pairing! In the app, click 'Pair new phone' to generate a secure QR code. On your Motorola phone, open Smart Connect, tap the plus icon, and scan the screen. Within a second, the cryptographic handshake completes, and your phone is officially paired and connected!"*

---

### Scene 7: Post-Install Showcase & Verification (5:45 – 7:00)

Showcase the new features in **v0.2.0** to finish the video with high production value:

1. **2026 Apple Pro Dark UI**:
   - Show the smooth dark surface hierarchy and clean typography.
2. **Command Palette (`Ctrl+K`)**:
   - Press `Ctrl + K`.
   - Show the Spotlight-style launcher, search for features, and view the developer card.
3. **App Grid View**:
   - Click the **Apps** tab.
   - Switch between 4-column responsive grid view and list stream.
4. **Live System Tray & Telemetry**:
   - Hover over the system tray icon in your top bar or system tray to show live phone battery and connection details.
5. **In-App Software Updates**:
   - Click the **`...`** menu in top right → click **Check for Updates…**.
   - Show the frosted dialog checking GitHub releases in real time, displaying:
     - *"Smart Connect is Up to Date (v0.2.0)"* with the green checkmark.
   - Mention that future updates can be installed with a single click right here without touching the terminal again!

**Voiceover / Closing:**
> *"And that's all it takes! You now have full Motorola Smart Connect running natively on Linux with live battery telemetry, phone screen mirroring, cross-device mouse control, and built-in automatic updates. If you found this helpful, drop a like, check out the GitHub link in the description, and feel free to star the repo. Thanks for watching!"*

---

## 📋 Quick Copy-Paste Cheatsheet (Keep Open While Recording)

```bash
# --- 1. FEDORA PREREQUISITES ---
sudo dnf install -y git gcc pkgconf-pkg-config dbus-devel wayland-devel avahi avahi-tools bluez wl-clipboard NetworkManager fuse3 gstreamer1-devel gstreamer1-plugins-base-devel gstreamer1-plugins-bad-free-devel gstreamer1-plugins-good gstreamer1-plugins-bad-free libnice-gstreamer1 gtk4-devel libadwaita-devel
sudo systemctl enable --now avahi-daemon bluetooth

# --- 2. CLONE & INSTALL ---
git clone https://github.com/Lalitmukesh69/smartconnect-linux-but-better.git
cd smartconnect-linux-but-better
./packaging/install.sh --enable

# --- 3. FIREWALL ---
sudo firewall-cmd --permanent --add-port=52020/tcp
sudo firewall-cmd --permanent --add-service=mdns
sudo firewall-cmd --reload

# --- 4. VERIFY & LAUNCH ---
systemctl --user status smartconnectd
smartconnect-gui &
```

---

## 🛠️ On-Camera Troubleshooting & Quick Fixes

| If this happens on camera... | Quick fix to run: |
|---|---|
| Phone cannot find PC during pairing | Run `sudo firewall-cmd --reload` and check that PC and phone are on the exact same Wi-Fi SSID. |
| Port 52020 already in use | `systemctl --user restart smartconnectd` (do not run `smartconnectd` manually while the systemd service is active). |
| Desktop icon not showing in app launcher | Run `update-desktop-database ~/.local/share/applications`. |
| Clipboard sync not reflecting | Make sure `wl-clipboard` is installed (`wl-copy --version`). |
