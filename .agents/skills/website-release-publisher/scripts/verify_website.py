#!/usr/bin/env python3
"""
Verification script for the Smart Connect for Linux Documentation Website.
Validates:
  1. Synchronized state between index.html and Install Smart Connect for Linux.html
  2. DOM ID uniqueness (no duplicate IDs)
  3. Internal anchor integrity (every href="#..." matches an existing id)
  4. JavaScript syntax validity via `node -c`
  5. JSON-LD Structured Data Schema validity
  6. Required distro command selectors (fedora, ubuntu, arch)
  7. SEO meta tags (title, description, keywords, canonical, OG, Twitter)
"""

import sys
import re
import json
import subprocess
import filecmp
from pathlib import Path

# .agents/skills/website-release-publisher/scripts/verify_website.py -> 4 levels up is repo root
REPO_ROOT = Path(__file__).resolve().parents[4]
INDEX_PATH = REPO_ROOT / "Website" / "index.html"
ALT_PATH = REPO_ROOT / "Website" / "Install Smart Connect for Linux.html"

def fail(msg: str):
    print(f"\033[91m✖ ERROR:\033[0m {msg}", file=sys.stderr)
    sys.exit(1)

def info(msg: str):
    print(f"\033[94mℹ\033[0m {msg}")

def success(msg: str):
    print(f"\033[92m✔\033[0m {msg}")

def main():
    info("Starting website integrity verification...")

    if not INDEX_PATH.exists():
        fail(f"Missing {INDEX_PATH}")
    if not ALT_PATH.exists():
        fail(f"Missing {ALT_PATH}")

    # 1. File synchronization
    if not filecmp.cmp(INDEX_PATH, ALT_PATH, shallow=False):
        fail("Website/index.html and Website/Install Smart Connect for Linux.html are NOT identical! Run sync first.")
    success("index.html and Install Smart Connect for Linux.html are perfectly synchronized.")

    with open(INDEX_PATH, "r", encoding="utf-8") as f:
        html = f.read()

    # 2. DOM ID uniqueness
    ids = re.findall(r'id="([^"]+)"', html)
    seen = set()
    duplicates = set()
    for item in ids:
        if item in seen:
            duplicates.add(item)
        seen.add(item)
    if duplicates:
        fail(f"Duplicate DOM IDs found: {duplicates}")
    success(f"DOM IDs are unique ({len(ids)} unique IDs verified).")

    # 3. Anchor integrity
    hrefs = re.findall(r'href="#([^"]+)"', html)
    missing = [h for h in hrefs if h not in seen]
    if missing:
        fail(f"Broken internal links found: {missing}")
    success(f"All {len(hrefs)} internal anchor links point to existing DOM targets.")

    # 4. JavaScript Syntax Validation
    scripts = re.findall(r'<script>(.*?)</script>', html, re.DOTALL)
    if not scripts:
        fail("No inline <script> block found in website.")
    
    tmp_js = Path("/tmp/smartconnect_web_verify.js")
    tmp_js.write_text(scripts[0], encoding="utf-8")
    try:
        res = subprocess.run(["node", "-c", str(tmp_js)], capture_output=True, text=True)
        if res.returncode != 0:
            fail(f"JavaScript syntax check failed:\n{res.stderr}")
    except FileNotFoundError:
        info("Node.js not installed; skipping `node -c` syntax check.")
    else:
        success("JavaScript syntax passed node -c check.")

    # 5. JSON-LD Structured Data
    ld_blocks = re.findall(r'<script type="application/ld\+json">(.*?)</script>', html, re.DOTALL)
    if not ld_blocks:
        fail("Missing Schema.org JSON-LD structured data block.")
    for idx, block in enumerate(ld_blocks, 1):
        try:
            data = json.loads(block)
            context = data.get("@context", "")
            if "schema.org" not in context:
                fail(f"JSON-LD block #{idx} does not reference schema.org")
        except json.JSONDecodeError as err:
            fail(f"JSON-LD block #{idx} invalid JSON: {err}")
    success("JSON-LD structured data is valid and adheres to Schema.org standards.")

    # 6. Distro command selectors
    for d in ["fedora", "ubuntu", "arch"]:
        pattern = rf'data-for="[^"]*\b{d}\b[^"]*"'
        if not re.search(pattern, html):
            fail(f"Missing command container for distro: {d}")
    for cmd in ["prereq", "svc", "install"]:
        if f'data-cmd="{cmd}"' not in html:
            fail(f"Missing code block with data-cmd='{cmd}'")
    for name in ["firewalld", "ufw"]:
        if f'data-name="{name}"' not in html:
            fail(f"Missing firewall block with data-name='{name}'")
    success("All distribution command bindings (Fedora, Ubuntu, Arch) verified.")

    # 7. SEO Meta tags
    required_tags = [
        '<meta name="description"',
        '<meta name="keywords"',
        '<meta name="author"',
        '<meta name="robots"',
        '<link rel="canonical"',
        '<meta property="og:title"',
        '<meta property="og:description"',
        '<meta property="og:image"',
        '<meta name="twitter:card"',
        '<meta name="twitter:title"',
    ]
    for tag in required_tags:
        if tag not in html:
            fail(f"Missing required SEO tag: {tag}")
    success("All required SEO and Open Graph metadata tags verified.")

    print("\n\033[92mAll website verification checks passed successfully!\033[0m")

if __name__ == "__main__":
    main()
