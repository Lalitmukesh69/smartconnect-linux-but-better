# Smart Connect for Linux Documentation

Welcome to the documentation for **Smart Connect for Linux**. This guide provides comprehensive information for users, system administrators, and developers.

---

## 📖 For Users

| Document | Description |
|---|---|
| **[Installation Guide](installation.md)** | Step-by-step setup for Flatpak, AppImage, and distribution packages (Arch, Fedora, Ubuntu/Debian). Covers firewall setup and systemd user services. |
| **[Software Updates & Releases](updater.md)** | In-app GitHub update checker, release changelog modal, and binary auto-replacement. |
| **[Usage & Features](usage.md)** | Complete walkthrough of pairing, auto-connect, screen mirroring, second monitor setup, Cross Control, and file sharing. |
| **[Feature Status & Compatibility](feature-status.md)** | Exhaustive matrix of implemented features, hardware verification status, and tested phone models. |
| **[System & Hardware Requirements](requirements.md)** | Operating system, Wayland compositor, kernel module, and network requirements. |
| **[Troubleshooting & Diagnostics](troubleshooting.md)** | Common connection issues, firewall fixes, Bluetooth LE wake tips, and log locations. |

---

## 🛠️ For Developers & Advanced Users

| Document | Description |
|---|---|
| **[Architecture & Components](components.md)** | Overview of the 8 modular binaries, daemon coordination, and process lifecycle. |
| **[Internal Architecture](architecture.md)** | Deep dive into the daemon's internal relay model, WebRTC pipeline, and FUSE mount implementation. |
| **[Notification Actions & Streaming](notification-actions.md)** | How background virtual displays and remote notification intents are dispatched. |
| **[Control Socket API](control-socket-api.md)** | Complete JSON-RPC specification for `$XDG_RUNTIME_DIR/smartconnectd.sock`. |
| **[Security & Privacy](security.md)** | Encryption standards, TLS certificates, credential storage, and privacy redaction rules. |
| **[Development Guide](development.md)** | Workspace layout, build instructions, unit tests, and adding new phone features. |
| **[Protocol Specifications](protocol/README.md)** | Reverse-engineered wire protocol specifications for every Motorola companion service. |

---

## 🔗 Project Links

- **Source Code**: [GitHub Repository](https://github.com/Lalitmukesh69/smartconnect-linux-but-better)
- **Issue Tracker**: [GitHub Issues](https://github.com/Lalitmukesh69/smartconnect-linux-but-better/issues)
- **Contributing Guidelines**: [CONTRIBUTING.md](../CONTRIBUTING.md)
- **Changelog**: [CHANGELOG.md](../CHANGELOG.md)
