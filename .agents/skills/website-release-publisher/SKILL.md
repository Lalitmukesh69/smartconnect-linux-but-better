---
name: website-release-publisher
description: >-
  Use this skill whenever publishing a new software version, updating the installation website,
  maintaining the GitHub Pages documentation, running automated integrity verification, or ensuring
  high-performance Apple-style documentation for Smart Connect for Linux.
---

# Website Release Publisher & Documentation System

This skill defines the end-to-end operational system and automated tooling for maintaining, updating, and publishing the official **Smart Connect for Linux** installation website hosted on GitHub Pages:
`https://lalitmukesh69.github.io/smartconnect-linux-but-better/`

---

## 🏗️ Architecture & Design System

The documentation site uses a high-performance, single-file minimalist Apple aesthetic:

### 1. Palette & Typography Tokens
- **Backgrounds**: `--bg: #ffffff`, `--alt: #f5f5f7` (subtle inset cards, code headers).
- **Text**: `--fg: #1d1d1f` (near-black high contrast), `--mut: #6e6e73` (secondary/subtitles).
- **Accents**: `--acc: #0066cc` (Apple link & active blue), `--soft: #eef5fd` (selected states).
- **Warnings**: `--warn: #fff8e6`, `--warnl: #d9b94a` (callout notices).
- **Typography**: `-apple-system, BlinkMacSystemFont, "SF Pro Text", "Inter", system-ui, sans-serif`.
- **Monospace**: `"SF Mono", ui-monospace, "JetBrains Mono", Menlo, Consolas, monospace`.

### 2. Core Interactive Systems
1. **Dynamic Distribution Switcher (`data-distro`)**:
   - Distro radio options: `fedora`, `ubuntu`, `arch`.
   - CSS-driven visibility:
     ```css
     html[data-distro] [data-for] { display: none; }
     html[data-distro=fedora] [data-for~=fedora],
     html[data-distro=ubuntu] [data-for~=ubuntu],
     html[data-distro=arch] [data-for~=arch] { display: block; }
     ```
   - Persists selection to `localStorage` under key `sc_distro`.

2. **Step Progress Checklist**:
   - Steps 1 through 5 contain `<input type="checkbox" data-done="N">`.
   - The desktop/mobile TOC displays `0 of 5 steps done` with a dynamic progress bar `#bar`.
   - Persists state in `localStorage` under key `sc_done`.

3. **Dynamic Zero-Hallucination AI Prompt Generator**:
   - `buildPrompt(distro)` scrapes the exact commands directly from the active DOM code blocks (`prereq`, `svc`, `install`, `firewalld`/`ufw`).
   - Generates perfectly tailored prompts for Claude Code, Antigravity, and terminal agents.
   - Includes accessible ARIA tabs: *Install prompt*, *Troubleshoot prompt*, and *Clean slate / uninstall*.

4. **Resilient Touchscreen Copy Fallback**:
   - Evaluates `navigator.clipboard.writeText(text)` inside secure contexts.
   - Automatically falls back to an offscreen `textarea` with `document.execCommand('copy')` for Android/iOS mobile browsers and webviews where clipboard permissions might be blocked.

5. **Search Engine & Social Discovery (SEO)**:
   - Includes Open Graph (`og:title`, `og:image`, `og:url`), Twitter summary cards, and canonical links.
   - Employs Schema.org JSON-LD Structured Data: `WebSite`, `SoftwareApplication`, and `HowTo` schemas for Google Search rich cards.

---

## ⚡ The Release Automation System

The repository includes pre-built Python automation scripts inside this skill:
- **`scripts/update_website_version.py`**: Updates version strings, SHA-256 binary checksums, and synchronizes files.
- **`scripts/verify_website.py`**: Executes automated integrity checks across HTML, JS, JSON-LD, anchor links, and SEO tags.
- **`resources/website-template.html`**: Master template with variable substitution tokens (`{{VERSION}}`, `{{SHA256}}`, etc.).

---

## 🚀 Step-by-Step Version Release Procedure

Follow this runbook every time a new version is released:

### Step 1: Update the Website Version & Checksum
Run the automated version updater:
```bash
python3 .agents/skills/website-release-publisher/scripts/update_website_version.py \
  --version v0.X.Y \
  --sha256 <64_character_sha256_hash>
```

This single command automatically updates:
1. `<title>` and Open Graph / Twitter card titles.
2. The sticky header brand badge (`<span>v0.X.Y</span>`).
3. Software version in Schema.org JSON-LD.
4. AI agent install prompt generator.
5. Standalone GUI curl download URL and SHA-256 verification line.
6. Synchronizes `Website/index.html` $\leftrightarrow$ `Website/Install Smart Connect for Linux.html`.
7. Executes `verify_website.py`.

### Step 2: Run Verification Checks
Ensure all integrity assertions pass:
```bash
python3 .agents/skills/website-release-publisher/scripts/verify_website.py
```

Checks enforced:
- [x] `index.html` and `Install Smart Connect for Linux.html` are 100% byte-identical.
- [x] Zero duplicate DOM IDs.
- [x] All internal anchor links (`href="#..."`) match existing DOM targets.
- [x] JavaScript syntax passes `node -c`.
- [x] Schema.org JSON-LD structured data is valid JSON.
- [x] Distro commands (`fedora`, `ubuntu`, `arch`) and firewall bindings exist.
- [x] Required SEO meta tags are present.

### Step 3: Synchronize GitHub Repository Metadata
Verify that the GitHub repository homepage and SEO topics are up to date:
```bash
gh repo edit \
  --homepage "https://lalitmukesh69.github.io/smartconnect-linux-but-better/" \
  --add-topic "smart-connect" \
  --add-topic "motorola" \
  --add-topic "motorola-ready-for" \
  --add-topic "linux" \
  --add-topic "gtk4" \
  --add-topic "rust" \
  --add-topic "wayland" \
  --add-topic "screen-mirroring" \
  --add-topic "android-mirroring" \
  --add-topic "virtual-display" \
  --add-topic "gstreamer"
```

### Step 4: Commit, Deploy & Watch
Commit the synchronized website files and monitor GitHub Actions:
```bash
git add Website/ README.md
git commit -m "docs(website): publish documentation for v0.X.Y"
git push origin main
```

Monitor the deployment run until completion:
```bash
gh run watch $(gh run list --workflow="Deploy Website to GitHub Pages" --limit 1 --json databaseId -q '.[0].databaseId') --exit-status
```

Verify live HTTP response:
```bash
curl -ILs "https://lalitmukesh69.github.io/smartconnect-linux-but-better/" | head -n 10
```

---

## 🛠️ Adding New Content or Distributions

When adding new sections or distributions to the website:

1. **Adding a Distribution**:
   - Add a radio button under `<fieldset class="seg">`:
     ```html
     <label><input type="radio" name="distro" value="opensuse">openSUSE</label>
     ```
   - Add a code block with `data-for="opensuse"` and `data-cmd="prereq"`.
   - Update CSS: `html[data-distro=opensuse] [data-for~=opensuse] { display: block }`.
   - Update `names` and `buildPrompt` in the `<script>` block.

2. **File Synchronization Rule**:
   - Never edit only one HTML file.
   - Always run the synchronizer:
     ```bash
     python3 .agents/skills/website-release-publisher/scripts/update_website_version.py --sync-only
     ```
