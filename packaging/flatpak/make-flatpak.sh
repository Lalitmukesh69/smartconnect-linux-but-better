#!/usr/bin/env bash
# Builds the Smart Connect standalone Flatpak bundle (.flatpak) and ostree repo.
#
#   packaging/flatpak/make-flatpak.sh
#
# Creates:
#   dist/Smart_Connect-<version>-x86_64.flatpak (standalone single-file bundle for easy download/install)
#   dist/dev.smartconnect.Linux.flatpak
#
# Anyone can install the resulting bundle easily with:
#   flatpak install dist/Smart_Connect-0.2.0-x86_64.flatpak
# Or double-click the file in GNOME Software / KDE Discover.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MANIFEST="$ROOT/packaging/flatpak/dev.smartconnect.Linux.json"
DIST_DIR="$ROOT/dist"
REPO_DIR="$DIST_DIR/flatpak-repo"
BUILD_DIR="$DIST_DIR/flatpak-build"
APP_ID="dev.smartconnect.Linux"
VERSION="0.2.0"
BUNDLE_NAME="Smart_Connect-${VERSION}-x86_64.flatpak"

mkdir -p "$DIST_DIR"

say() { printf '\033[1;34m==>\033[0m %s\n' "$*"; }
ok()  { printf '\033[1;32m[ok]\033[0m %s\n' "$*"; }
warn() { printf '\033[1;33m[!]\033[0m %s\n' "$*"; }
die() { printf '\033[1;31m[x]\033[0m %s\n' "$*" >&2; exit 1; }

# Check for local flatpak-builder
if command -v flatpak-builder >/dev/null 2>&1; then
  say "Building Flatpak locally using host flatpak-builder"
  
  # Ensure GNOME 47 Platform & Sdk are installed
  if ! flatpak info org.gnome.Platform//47 >/dev/null 2>&1; then
    say "Installing org.gnome.Platform//47 and Sdk..."
    flatpak install -y --user flathub org.gnome.Platform//47 org.gnome.Sdk//47 org.freedesktop.Sdk.Extension.rust-stable//24.08 || true
  fi

  flatpak-builder --force-clean --user --install-deps-from=flathub \
    --repo="$REPO_DIR" "$BUILD_DIR" "$MANIFEST"

  say "Exporting standalone Flatpak bundle to $DIST_DIR/$BUNDLE_NAME"
  flatpak build-bundle "$REPO_DIR" "$DIST_DIR/$BUNDLE_NAME" "$APP_ID"
  cp -f "$DIST_DIR/$BUNDLE_NAME" "$DIST_DIR/$APP_ID.flatpak"

  ok "Flatpak bundle successfully created:"
  echo "    $DIST_DIR/$BUNDLE_NAME"
  echo "    $DIST_DIR/$APP_ID.flatpak"
  echo ""
  echo "Users can download and install it with:"
  echo "    flatpak install $DIST_DIR/$BUNDLE_NAME"
  exit 0
fi

# Fallback to containerized build with Podman or Docker
CONTAINER_CMD=""
if command -v podman >/dev/null 2>&1; then
  CONTAINER_CMD="podman"
elif command -v docker >/dev/null 2>&1; then
  CONTAINER_CMD="docker"
fi

if [ -n "$CONTAINER_CMD" ]; then
  say "flatpak-builder not found on host; building inside $CONTAINER_CMD container"
  
  # Build inside container with rust and flatpak-builder
  exec "$CONTAINER_CMD" run --rm --privileged \
    -e HOST_UID="$(id -u)" -e HOST_GID="$(id -g)" \
    -v "$ROOT":/src:ro \
    -v "$DIST_DIR":/out \
    -v smartconnect-flatpak-cache:/cache \
    ghcr.io/flathub-infra/flatpak-builder:latest \
    bash -c "
      set -euo pipefail
      mkdir -p /build && cd /build
      cp -a /src/. /build/
      flatpak remote-add --if-not-exists flathub https://dl.flathub.org/repo/flathub.flatpakrepo
      flatpak-builder --force-clean --repo=/out/flatpak-repo /tmp/flatpak-build /build/packaging/flatpak/dev.smartconnect.Linux.json
      flatpak build-bundle /out/flatpak-repo /out/$BUNDLE_NAME $APP_ID
      cp -f /out/$BUNDLE_NAME /out/$APP_ID.flatpak
      chown -R \${HOST_UID}:\${HOST_GID} /out
    "
fi

die "Neither flatpak-builder nor podman/docker was found. Please install flatpak-builder or podman."
