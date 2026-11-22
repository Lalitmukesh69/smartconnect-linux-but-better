# Smart Connect for Linux v0.2.0

Compiled release of **Smart Connect for Linux** (v0.2.0), bringing Motorola Smart Connect functionality to Linux desktops.

## Highlights
- **In-App Software Updates**: Check, download, and apply releases directly from within the app.
- **Command Palette (`Ctrl+K`)**: Fast Spotlight-style quick switcher and actions.
- **2026 Dark UI**: Modern dark theme tokens with Apple Pro monochrome aesthetic.
- **Live Telemetry**: Dynamic battery, cellular signal, and Wi-Fi link monitoring.

## What's Changed
- Added automatic and manual GitHub release updates (`gui/src/updater.rs`).
- Added Update preferences in Preferences > General.
- Fixed phone overview telemetry display during standby states.
- Re-architected documentation and feature matrices in `docs/`.

## Download & Verification
- **Binary:** `smartconnect-gui` · Linux x86_64 · ELF · Stripped & optimized
- **SHA-256:**
```text
<SHA256_HASH_HERE>
```

Verify checksum:
```sh
echo "<SHA256_HASH_HERE>  smartconnect-gui" | sha256sum -c -
```

## Installation
If running the app, click **Update Now** inside the **Software Update** dialog.

Manual installation:
```sh
chmod +x smartconnect-gui
install -m755 smartconnect-gui ~/.local/bin/smartconnect-gui
```
