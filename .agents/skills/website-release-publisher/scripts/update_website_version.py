#!/usr/bin/env python3
"""
Automated version updater and website synchronizer for Smart Connect for Linux.
Usage:
  python3 update_website_version.py --version v0.3.0 [--sha256 <hex_hash>]
  python3 update_website_version.py --sync-only
"""

import argparse
import re
import shutil
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[4]
INDEX_PATH = REPO_ROOT / "Website" / "index.html"
ALT_PATH = REPO_ROOT / "Website" / "Install Smart Connect for Linux.html"
VERIFY_SCRIPT = Path(__file__).resolve().parent / "verify_website.py"

def main():
    parser = argparse.ArgumentParser(description="Update website release version and synchronize files.")
    parser.add_argument("--version", type=str, help="Release version (e.g., v0.3.0 or 0.3.0)")
    parser.add_argument("--sha256", type=str, help="Optional SHA-256 checksum for standalone GUI binary")
    parser.add_argument("--sync-only", action="store_true", help="Only synchronize index.html to Install Smart Connect for Linux.html")
    args = parser.parse_args()

    if not INDEX_PATH.exists():
        print(f"Error: {INDEX_PATH} not found", file=sys.stderr)
        sys.exit(1)

    if args.sync_only:
        print("Synchronizing index.html to Install Smart Connect for Linux.html...")
        shutil.copyfile(INDEX_PATH, ALT_PATH)
        print("✓ Files synchronized.")
        return

    if not args.version:
        parser.print_help()
        sys.exit(1)

    clean_v = args.version.strip()
    v_with_prefix = clean_v if clean_v.startswith("v") else f"v{clean_v}"
    v_num = clean_v.lstrip("v")

    print(f"Updating website version to {v_with_prefix} ({v_num})...")

    with open(INDEX_PATH, "r", encoding="utf-8") as f:
        html = f.read()

    # 1. Update <title>
    html = re.sub(
        r'<title>Install Smart Connect for Linux \(v[^)]+\)',
        f'<title>Install Smart Connect for Linux ({v_with_prefix})',
        html
    )

    # 2. Update brand span
    html = re.sub(
        r'(<a class="brand"[^>]*>Smart Connect for Linux<span>)v[^<]+(</span></a>)',
        rf'\g<1>{v_with_prefix}\g<2>',
        html
    )

    # 3. Update OG title
    html = re.sub(
        r'(<meta property="og:title" content="Install Smart Connect for Linux \()v[^)]+(\)")',
        rf'\g<1>{v_with_prefix}\g<2>',
        html
    )

    # 4. Update Twitter title
    html = re.sub(
        r'(<meta name="twitter:title" content="Install Smart Connect for Linux \()v[^)]+(\)")',
        rf'\g<1>{v_with_prefix}\g<2>',
        html
    )

    # 5. Update JSON-LD softwareVersion
    html = re.sub(
        r'("softwareVersion":\s*")[^"]+(")',
        rf'\g<1>{v_num}\g<2>',
        html
    )

    # 6. Update AI Prompt default text
    html = re.sub(
        r'(Install and set up \\"Smart Connect for Linux\\" \()v[^)]+(\))',
        rf'\g<1>{v_with_prefix}\g<2>',
        html
    )

    # 7. Update download URL and Checksum if provided
    if args.sha256:
        clean_hash = args.sha256.strip().lower()
        print(f"Updating SHA-256 checksum to {clean_hash}...")
        html = re.sub(
            r'releases/download/v[^/]+/smartconnect-gui',
            f'releases/download/{v_with_prefix}/smartconnect-gui',
            html
        )
        html = re.sub(
            r'echo "[a-f0-9]{64}\s+(\$HOME|\~)/\.local/bin/smartconnect-gui"',
            f'echo "{clean_hash}  $HOME/.local/bin/smartconnect-gui"',
            html
        )

    # Write updated index.html
    with open(INDEX_PATH, "w", encoding="utf-8") as f:
        f.write(html)

    # Mirror to Install Smart Connect for Linux.html
    shutil.copyfile(INDEX_PATH, ALT_PATH)
    print("✓ Updated Website/index.html and synced Website/Install Smart Connect for Linux.html")

    # Run verification
    import subprocess
    print("\nRunning verification...")
    res = subprocess.run([sys.executable, str(VERIFY_SCRIPT)])
    sys.exit(res.returncode)

if __name__ == "__main__":
    main()
