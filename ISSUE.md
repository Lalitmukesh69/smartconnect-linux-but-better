# Known Issues

See [README.md](README.md) for project overview, installation, and documentation.
Tracked on GitHub under [Issues](https://github.com/Lalitmukesh69/smartconnect-linux-ui/issues).

- **Only one phone at a time.** ([#1](https://github.com/Lalitmukesh69/smartconnect-linux-ui/issues/1)) The daemon pairs and connects a single phone. While one phone is
  connected you cannot pair or connect a second one; disconnect the first, and later reconnect it
  only after disconnecting the other. Phones taking turns works, two at once does not.
  Multi-device support is tracked in [TODO.md](TODO.md).
- **Reconnecting while Smart Connect is in the background needs Bluetooth on the PC.** ([#2](https://github.com/Lalitmukesh69/smartconnect-linux-ui/issues/2)) A
  backgrounded phone app has no Wi-Fi listener and never reconnects to a PC by itself, so the daemon
  wakes it over Bluetooth LE, as the Windows app does. The daemon never switches Bluetooth on or off:
  with it off, open Smart Connect on the phone (or tap Connect there), or switch Bluetooth on and the
  daemon reconnects at once. `--no-ble-wake` disables the wake. The phone's background Bluetooth
  advert is weak, so a wake can take a few retries when the phone is far from the PC.
- **Messaging notifications show the app icon, not the sender's picture.** ([#3](https://github.com/Lalitmukesh69/smartconnect-linux-ui/issues/3)) With WhatsApp-style chat
  notifications the phone sends only its owner's own picture, never the sender's
  ([details](docs/protocol/notifications.md#whose-picture-is-person_icon)).
- **Browsing the phone's files can be slow.** ([#4](https://github.com/Lalitmukesh69/smartconnect-linux-ui/issues/4)) Every directory and file read is an NFS round-trip over
  TLS to the phone, so large folders and big files feel sluggish. Read caching is on the
  [TODO list](TODO.md).
- **Writing to the phone and the image/file clipboard aren't implemented yet.** ([#5](https://github.com/Lalitmukesh69/smartconnect-linux-ui/issues/5)) Sharing a PC folder
  *to* the phone is built but not yet verified with a phone
  ([file-exchange.md](docs/protocol/file-exchange.md)).
- **The webcam is 1280x720 at 30 fps.** ([#6](https://github.com/Lalitmukesh69/smartconnect-linux-ui/issues/6)) The phone app fixes this in this mode. Without
  `v4l2loopback` the camera is PipeWire-only: OBS, Firefox with
  `media.webrtc.camera.allow-pipewire`, Chromium with its PipeWire camera flag. Most other apps need
  `v4l2loopback`.
- **Extend on niri needs a virtual output.** ([#7](https://github.com/Lalitmukesh69/smartconnect-linux-ui/issues/7)) niri cannot create one on its own; use the `vkms`
  module (`docs/installation.md#display-virtual-output`) or an HDMI dummy plug. If
  `smartconnect-display` is killed instead of stopped, turn the output off by hand:
  `niri msg output Virtual-1 off` (or, on Hyprland, `hyprctl output remove SMARTCONNECT-1`).
- **KDE Plasma needs the installed `.desktop` file for the phone display** ([#8](https://github.com/Lalitmukesh69/smartconnect-linux-ui/issues/8)) (KWin only offers screen
  casting and input injection to trusted programs). Run the copy `install.sh` installed.
- **The PC lock tile can't lock GNOME started from SDDM or LightDM:** ([#9](https://github.com/Lalitmukesh69/smartconnect-linux-ui/issues/9)) GNOME Shell only has a lock
  screen under GDM. Other lockers that ignore logind need `--lock-command`.
- **Reconnects are accepted on the phone's device ID**; ([#10](https://github.com/Lalitmukesh69/smartconnect-linux-ui/issues/10)) it's still unknown whether the phone can
  prove it holds the pairing key. See [Security](docs/security.md).
- **Noctalia plugin stuck on "smartconnectd is not running" after a daemon restart.** ([#11](https://github.com/Lalitmukesh69/smartconnect-linux-ui/issues/11)) Fixed; restart
  Noctalia once after updating, since switching the plugin off and on reloads its panels but not its
  shared service.
- **Quickshell crashes after a Qt update.** ([#12](https://github.com/Lalitmukesh69/smartconnect-linux-ui/issues/12)) If `qs` warns that it "was built against Qt X but the
  system has updated to Qt Y", rebuild or reinstall your Quickshell package (for example
  `noctalia-qs`).
