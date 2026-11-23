---
name: website-release-publisher
description: >-
  Use this skill whenever publishing a new software version, updating the installation website,
  maintaining the GitHub Pages documentation, or ensuring mobile phone responsiveness and seamless
  one-click installation commands for Smart Connect for Linux.
---

# Website Release Publisher & Documentation Runbook

This skill provides the operational playbook for Antigravity agents and developers to update, verify, and publish the high-performance, mobile-responsive installation website hosted on GitHub Pages (`https://lalitmukesh69.github.io/smartconnect-linux-but-better/`).

---

## 🚀 When to Use This Skill

Activate this skill whenever:
1. Releasing a new version of Smart Connect for Linux (e.g., `v0.2.0` $\rightarrow$ `v0.3.0`).
2. Updating prerequisites, packaging scripts, or installation commands across distributions (Fedora, Ubuntu/Debian, Arch Linux).
3. Adding new features, troubleshooting steps, or AI agent installation prompts.
4. Ensuring optimal phone/mobile responsiveness, touch interactivity, and zero-freeze scrolling.

---

## 📋 The Release Website Update Playbook

### Step 1: Version & Metadata Synchronization
Update the version strings and download URLs in `Website/index.html`:
- **Title tag & description**: `<title>Install Smart Connect for Linux (vX.Y.Z) — Documentation</title>`
- **Header badge**: `<span class="version-badge">vX.Y.Z</span>`
- **Doc eyebrow**: `<span class="doc-eyebrow">Installation Guide · vX.Y.Z</span>`
- **Release download CTA**: Check the button pointing to `https://github.com/Lalitmukesh69/smartconnect-linux-but-better/releases/latest`
- **AI Agent prompts**: Update the version string in `AGENT_PROMPTS` within the `<script>` tag.

Always keep `Website/Install Smart Connect for Linux.html` identical to `Website/index.html`:
```bash
cp "Website/index.html" "Website/Install Smart Connect for Linux.html"
```

---

### Step 2: Maintain Easy Installation Snippets
Users install Smart Connect through three primary pathways provided on the site:
1. **⚡ Instant 1-Line Setup (Terminal)**:
   - Synchronized across Fedora, Ubuntu/Debian, and Arch Linux in `QUICK_COMMANDS`:
     ```javascript
     var QUICK_COMMANDS = {
       fedora: { label: 'Fedora · Terminal Install Command', code: '...' },
       ubuntu: { label: 'Ubuntu / Debian · Terminal Install Command', code: '...' },
       arch: { label: 'Arch Linux · Terminal Install Command', code: '...' }
     };
     ```
2. **🤖 AI Agent Prompts**:
   - Ready-to-copy prompts tailored for Claude Code, Antigravity, and terminal coding agents in `AGENT_PROMPTS`.
3. **📦 Manual Step-by-Step Guide**:
   - Explicit steps (Prerequisites $\rightarrow$ Automated Installer $\rightarrow$ Firewall port `52020` $\rightarrow$ Verification $\rightarrow$ Pairing).

---

### Step 3: Strict Mobile Responsiveness Standards
Every change to `Website/index.html` must follow these non-negotiable mobile guidelines:

1. **No Scroll Hijacking or Freezing**:
   - **Never** call `e.preventDefault()` on hash anchor navigation (`a[href^="#"]`).
   - Use native browser scrolling powered by CSS:
     ```css
     html { scroll-behavior: smooth; scroll-padding-top: 108px; }
     :target, .doc-section, .doc-intro, section[id] { scroll-margin-top: 108px; }
     ```
2. **Horizontal Overflow Protection**:
   - Root elements must contain horizontal overflow:
     ```css
     html, body { max-width: 100vw; overflow-x: hidden; touch-action: manipulation; }
     ```
   - Code blocks (`pre`), prompt blocks, and `table` elements must support horizontal touch scrolling without trapping vertical swipes:
     ```css
     .code-block pre, .prompt-block pre, .quick-terminal-code, table {
         max-width: 100%;
         overflow-x: auto;
         -webkit-overflow-scrolling: touch;
         touch-action: pan-x pan-y;
         overscroll-behavior-x: contain;
     }
     ```
3. **Comfortable Touch Targets**:
   - All interactive elements (buttons, distro tabs, copy buttons, checkboxes, mobile menu links) must have a touch target height of at least $\ge 40\text{px}$ (ideally $\ge 44\text{px}$).
4. **Mobile Navigation Drawer**:
   - Test that the hamburger button `#mobile-menu-toggle` opens the `#mobile-nav-drawer` overlay and that tapping any link closes the drawer automatically.

---

### Step 4: Automated Verification Checklist
Run this quick Python script to verify DOM integrity and link validity before committing:
```bash
python3 - << 'EOF'
import re

with open("Website/index.html", "r", encoding="utf-8") as f:
    html = f.read()

ids = re.findall(r'id="([^"]+)"', html)
duplicates = set([x for x in ids if ids.count(x) > 1])
hrefs = re.findall(r'href="#([^"]+)"', html)
missing = [h for h in hrefs if h not in set(ids)]

print("Validation results:")
print(f"  • Total DOM IDs: {len(ids)}")
print(f"  • Duplicate IDs: {len(duplicates)} {duplicates if duplicates else '✓ (None)'}")
print(f"  • Broken Internal Links: {len(missing)} {missing if missing else '✓ (None)'}")
assert len(duplicates) == 0 and len(missing) == 0, "Validation failed!"
EOF
```

---

### Step 5: Commit, Deploy & Verify
The repository has an automated GitHub Actions deployment workflow at `.github/workflows/deploy-pages.yml` that triggers on any push to `main` affecting `Website/**`.

1. Commit and push changes:
   ```bash
   git add Website/
   git commit -m "docs(website): update release documentation for vX.Y.Z"
   git push origin main
   ```

2. Monitor the GitHub Actions deployment:
   ```bash
   gh run watch $(gh run list --workflow="Deploy Website to GitHub Pages" --limit 1 --json databaseId -q '.[0].databaseId') --exit-status
   ```

3. Verify HTTP 200 response on the live domain:
   ```bash
   curl -ILs "https://lalitmukesh69.github.io/smartconnect-linux-but-better/" | head -n 5
   ```
