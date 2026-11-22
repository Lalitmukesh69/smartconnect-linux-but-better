---
name: github-desktop-updater
description: >-
  Use this skill whenever building, publishing, troubleshooting, or automating software updates
  for Smart Connect for Linux and Linux desktop applications. Contains the end-to-end operational
  runbook for creating GitHub releases, generating compatible binary assets, validating in-app
  changelog rendering, atomic executable replacement, and testing the in-app update modal.
---

# GitHub Desktop In-App Updater & Release Runbook

This skill is the operational playbook for Antigravity agents and developers to build, publish, and maintain seamless GitHub-based in-app updates for Smart Connect for Linux.

---

## 📋 Release Preparation Runbook

Whenever the user asks to **"publish a release"**, **"create an update"**, or **"bump version and release"**, follow this exact step-by-step workflow:

### Step 1: Pre-flight Checks
Verify the git working tree, current branch, and test suite:
```bash
git status
cargo test -p smartconnect-gui
```
Ensure all tests pass before proceeding.

### Step 2: Version Bumping
Determine the new semantic version (e.g., `0.2.0`). Update:
1. `Cargo.toml`: `version = "0.2.0"`
2. `gui/Cargo.toml`: `version = "0.2.0"`

### Step 3: Build & Strip Release Binaries
Compile the stripped release binary:
```bash
cargo build --release -p smartconnect-gui
strip target/release/smartconnect-gui
```

Calculate SHA-256 checksum:
```bash
sha256sum target/release/smartconnect-gui > target/release/smartconnect-gui.sha256
cat target/release/smartconnect-gui.sha256
```

### Step 4: Draft Standardized Release Notes
Format release notes using the structure in [`examples/release-template.md`](./examples/release-template.md).
The in-app changelog parser (`gui/src/updater.rs:format_changelog_text`) expects:
- Level-1 heading: `# Smart Connect for Linux vX.Y.Z`
- Level-2 sections: `## Highlights`, `## What's Changed`, `## Download & Verification`
- Bullet lists: `- ` or `* ` for feature items
- Code blocks: Fenced ``` blocks for checksums and commands

### Step 5: Publish Release via GitHub CLI (`gh`)
Use the automated helper script:
```bash
.agents/skills/github-desktop-updater/scripts/publish-release.sh <VERSION> [CHANGELOG_PATH]
```
Or run the manual CLI command:
```bash
gh release create "v<VERSION>" \
  target/release/smartconnect-gui \
  target/release/smartconnect-gui.sha256 \
  --repo Lalitmukesh69/smartconnect-linux-but-better \
  --title "Smart Connect for Linux v<VERSION>" \
  --notes-file <CHANGELOG_PATH>
```

---

## ⚙️ In-App Updater Mechanics & Architecture

### Target Asset Matching
When the app checks `https://api.github.com/repos/Lalitmukesh69/smartconnect-linux-but-better/releases/latest`, `updater::select_asset` matches the asset based on runtime environment:

| Runtime Environment | Target Asset Name | Target Destination |
|---|---|---|
| **Sharun Portable** (`~/.local/opt/smartconnect-linux/`) | `smartconnect-gui` | `~/.local/opt/smartconnect-linux/shared/bin/smartconnect-gui` |
| **User Direct Binary** (`~/.local/bin/`) | `smartconnect-gui` | `~/.local/bin/smartconnect-gui` |
| **AppImage** | `*.AppImage` | `$APPIMAGE` path |
| **Flatpak** | None (read-only) | Prompts user to run `flatpak update dev.smartconnect.Linux` |
| **System Package** (`/usr/bin/`) | None (read-only) | Prompts user to run distro package manager |

> [!IMPORTANT]
> **Asset Naming Convention**:
> The primary Linux binary asset attached to the GitHub Release MUST be named exactly `smartconnect-gui` (or contain `smartconnect-gui` without `.sha256` extension).

---

## 🔒 Atomic In-Place Binary Replacement

### Avoiding `ETXTBSY` ("Text file busy")
On Linux, opening an actively running executable file with write permissions causes kernel error `ETXTBSY`.
The updater handles this safely:
1. Downloads the new binary to `~/.cache/smartconnect-linux/updates/<asset>.download`.
2. Sets executable permissions (`chmod 0o755`).
3. Executes an atomic POSIX `rename(temp_file, target_binary)`.
4. The directory entry is repointed to the new inode; the running process continues uninterrupted on its old in-memory pages.
5. On clicking **Restart Smart Connect**, the app calls `std::process::Command::new(current_exe).spawn()` and exits cleanly.

---

## 🧪 Verification & Testing Runbook

### Testing Update Availability
1. Test GitHub API response:
   ```bash
   curl -fsSL -H "Accept: application/vnd.github+json" \
     https://api.github.com/repos/Lalitmukesh69/smartconnect-linux-but-better/releases/latest | jq '{tag_name, name, assets: [.assets[] | .name]}'
   ```
2. Verify In-App UI Dialog:
   - Launch: `smartconnect-gui`
   - Open **App Menu** (`...` top-right) -> **Check for Updates…**
   - Alternatively: Press `Ctrl + K` -> Type `update` -> Press Enter.
   - Alternatively: Open **Preferences** (`Ctrl + ,`) -> Click **Check Now**.
3. Verify Update States:
   - **Up to Date**: Green checkmark circle with message "Smart Connect is Up to Date".
   - **Update Available**: Displays new version tag, published date, formatted changelog card, and **Update Now** button.
   - **Downloading**: Smooth `gtk::ProgressBar` with downloaded MB / total MB and percentage.
   - **Complete**: "Update installed successfully! Restart to apply changes" with **Restart Smart Connect** button.

---

## 🚨 Troubleshooting

| Issue | Cause | Fix |
|---|---|---|
| In-app dialog says "Couldn't Check for Updates" | Offline or GitHub API rate limit (60 req/hr unauthenticated) | Check internet connectivity; wait for rate limit reset or use authenticated curl. |
| In-app dialog says "No Compatible Binary" | Release has no asset named `smartconnect-gui` | Ensure release assets include `smartconnect-gui` Linux x86_64 binary. |
| Button says "Managed by System" | App running from `/usr/bin/` or Flatpak | Normal behavior: read-only package managers must update through their own tooling. |
| Download fails midway | Network drop or timeout | Updater cleans partial temp file; clicking "Retry Download" resumes cleanly. |
| Binary not replaced in Sharun | Overwriting `bin/` instead of `shared/bin/` | `updater::detect_install_target` specifically targets `shared/bin/smartconnect-gui` to preserve Sharun wrapper. |
