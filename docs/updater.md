# Software Updates & GitHub Releases

Smart Connect for Linux includes an in-app software update system that directly checks, downloads, and applies releases from the official GitHub repository.

---

## ⚡ Overview

The update system queries the GitHub Releases API for:
`https://api.github.com/repos/Lalitmukesh69/smartconnect-linux-but-better/releases/latest`

When a new version tag (e.g. `v0.2.0`) is published with compiled binaries, the app:
1. Notifies the user via an in-app banner or modal dialog.
2. Displays the full release notes and changelog.
3. Automatically downloads and installs the matching binary.
4. Atomically replaces the local executable using POSIX `rename()` (avoiding `ETXTBSY`).
5. Prompts the user to restart the application to apply the changes.

---

## 🚀 How to Check for Updates

You can check for updates through three primary entry points:

1. **Application Menu**:
   - Click the overflow menu (`...`) in the top-right header.
   - Select **Check for Updates…**.

2. **Preferences**:
   - Open **Preferences** (`Ctrl + ,`).
   - Under **General**, view the current version and click **Check Now**.
   - Toggle **Automatically check for updates** on or off.

3. **Command Palette (`Ctrl + K`)**:
   - Press `Ctrl + K`.
   - Type `update` and press **Enter**.

---

## 📦 Supported Packaging Environments

The updater inspects its execution environment and applies the appropriate strategy:

| Environment | Update Mechanism |
|---|---|
| **Portable Sharun Bundle** (`~/.local/opt/smartconnect-linux/`) | Downloads `smartconnect-gui` and atomically replaces `shared/bin/smartconnect-gui`. |
| **User Direct Binary** (`~/.local/bin/`) | Downloads `smartconnect-gui` and atomically replaces `~/.local/bin/smartconnect-gui`. |
| **AppImage** | Detects `$APPIMAGE` environment variable and replaces the running `.AppImage` file. |
| **Flatpak** | Container is read-only: displays guidance to run `flatpak update dev.smartconnect.Linux`. |
| **System Package** (`/usr/bin/`) | Root-owned binary: displays guidance to use your system package manager (e.g. `yay -S smartconnect-linux`). |

---

## 🔒 Security & Verification

- **Atomic Replacement**: On Linux, replacing an active binary with `rename()` updates the directory entry without disrupting the in-memory execution of the running process.
- **Cache Isolation**: Downloaded updates are stored in `~/.cache/smartconnect-linux/updates/` before validation and atomic replacement.
- **Permissions**: Binaries are verified with executable permissions (`0o755`) before installation.
