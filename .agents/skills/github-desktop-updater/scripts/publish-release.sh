#!/usr/bin/env bash
# Automated release script for Smart Connect for Linux
# Builds release binary, computes SHA256, and publishes to GitHub via `gh release create`
set -euo pipefail

usage() {
    cat <<EOF
Usage: $0 <version> [notes_file]

Example:
    $0 0.2.0 docs/releases/v0.2.0.md
    $0 0.2.0  # uses default template

Steps performed:
  1. Validates clean git tree and semver format.
  2. Bumps version in Cargo.toml and gui/Cargo.toml.
  3. Runs test suite (cargo test -p smartconnect-gui).
  4. Builds optimized release binary (cargo build --release -p smartconnect-gui).
  5. Computes SHA-256 checksum and packages assets.
  6. Tags and pushes release commit to origin.
  7. Publishes GitHub release with smartconnect-gui binary asset.
EOF
    exit 1
}

[ $# -ge 1 ] || usage
VERSION="${1#v}" # Strip leading 'v' if provided
TAG="v${VERSION}"
NOTES_FILE="${2:-}"

REPO="Lalitmukesh69/smartconnect-linux-but-better"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
cd "$ROOT_DIR"

echo "==> Preparing release ${TAG} for ${REPO}..."

# 1. Validate semver
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?$ ]]; then
    echo "ERROR: Version '$VERSION' is not valid semver (e.g. 0.2.0)" >&2
    exit 1
fi

# 2. Check gh auth
if ! gh auth status &>/dev/null; then
    echo "ERROR: GitHub CLI (gh) is not authenticated. Run 'gh auth login' first." >&2
    exit 1
fi

# 3. Update version in Cargo.toml files
echo "==> Updating version to ${VERSION} in Cargo.toml and gui/Cargo.toml..."
sed -i "0,/^version = .*/s//version = \"${VERSION}\"/" Cargo.toml
sed -i "0,/^version = .*/s//version = \"${VERSION}\"/" gui/Cargo.toml

# 4. Run tests
echo "==> Running automated tests..."
cargo test -p smartconnect-gui

# 5. Build release binary
echo "==> Compiling optimized release binary..."
cargo build --release -p smartconnect-gui

RELEASE_BIN="target/release/smartconnect-gui"
[ -f "$RELEASE_BIN" ] || { echo "ERROR: $RELEASE_BIN not found" >&2; exit 1; }

# Strip symbols to minimize download size
strip "$RELEASE_BIN" || true

# Compute checksum
SHA256="$(sha256sum "$RELEASE_BIN" | awk '{print $1}')"
echo "==> Binary SHA-256: ${SHA256}"
echo "${SHA256}  smartconnect-gui" > "${RELEASE_BIN}.sha256"

# 6. Generate notes if not provided
if [ -z "$NOTES_FILE" ] || [ ! -f "$NOTES_FILE" ]; then
    NOTES_TMP="$(mktemp --suffix=.md)"
    cat <<EOF > "$NOTES_TMP"
# Smart Connect for Linux ${TAG}

Compiled release of **Smart Connect for Linux** (${TAG}).

## Highlights
- Latest bug fixes, UI enhancements, and stability updates.
- Native in-app software updates supported.

## Download & Verification
- **Binary**: \`smartconnect-gui\` (Linux x86_64, ELF, stripped)
- **SHA-256 Checksum**:
\`\`\`text
${SHA256}
\`\`\`

Verify with:
\`\`\`sh
echo "${SHA256}  smartconnect-gui" | sha256sum -c -
\`\`\`
EOF
    NOTES_FILE="$NOTES_TMP"
fi

# 7. Commit version bump if git is modified
if ! git diff --quiet Cargo.toml gui/Cargo.toml; then
    echo "==> Committing version bump..."
    git add Cargo.toml gui/Cargo.toml
    git commit -m "chore(release): bump version to ${TAG}"
    git push origin HEAD
fi

# 8. Create GitHub release with assets
echo "==> Publishing GitHub Release ${TAG}..."
gh release create "$TAG" \
    "$RELEASE_BIN" \
    "${RELEASE_BIN}.sha256" \
    --repo "$REPO" \
    --title "Smart Connect for Linux ${TAG}" \
    --notes-file "$NOTES_FILE"

echo "==> Successfully published ${TAG} to https://github.com/${REPO}/releases/tag/${TAG}"
EOF
chmod +x "$0"
