#!/usr/bin/env bash
# package-appimage.sh — package os-network-admin daemon as a portable AppImage.
#
# Usage:
#   ./scripts/package-appimage.sh <version>
#
# Prerequisites:
#   - appimagetool on PATH (download from https://github.com/AppImage/AppImageKit)
#   - cargo build --release --features daemon already run
#   - CARGO_TARGET_DIR set or cargo-target/mathew used by default
#
# Output:
#   os-network-admin-<version>-x86_64.AppImage
#
# Install on target (Linux Mint / Ubuntu / Debian):
#   chmod +x os-network-admin-<version>-x86_64.AppImage
#   sudo ./os-network-admin-<version>-x86_64.AppImage
#   # Requires: sudo apt install wireguard  (WireGuard kernel module)
#   # Requires: CAP_NET_ADMIN for wg set peer operations

set -euo pipefail

VERSION="${1:?Usage: $0 <version>}"
BINARY="${CARGO_TARGET_DIR:-/srv/foundry/cargo-target/mathew}/release/os-network-admin"
APPDIR="$(mktemp -d)/os-network-admin.AppDir"

if [[ ! -f "$BINARY" ]]; then
  echo "Binary not found at $BINARY" >&2
  echo "Run: cargo build --release --features daemon" >&2
  exit 1
fi

if ! command -v appimagetool &>/dev/null; then
  echo "appimagetool not found on PATH." >&2
  echo "Download from: https://github.com/AppImage/AppImageKit/releases" >&2
  exit 1
fi

mkdir -p "$APPDIR/usr/bin"
cp "$BINARY" "$APPDIR/usr/bin/os-network-admin"

# AppImage required files
cat > "$APPDIR/AppRun" << 'EOF'
#!/bin/sh
exec "$(dirname "$(readlink -f "$0")")/usr/bin/os-network-admin" "$@"
EOF
chmod +x "$APPDIR/AppRun"

cat > "$APPDIR/os-network-admin.desktop" << EOF
[Desktop Entry]
Type=Application
Name=os-network-admin
Exec=os-network-admin
Icon=os-network-admin
Categories=Network;
EOF

# Placeholder icon (1x1 PNG — real icon in future session)
printf '\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x02\x00\x00\x00\x90wS\xde\x00\x00\x00\x0cIDATx\x9cc\xf8\x0f\x00\x00\x01\x01\x00\x05\x18\xd8N\x00\x00\x00\x00IEND\xaeB`\x82' \
  > "$APPDIR/os-network-admin.png"

OUTPUT="os-network-admin-${VERSION}-x86_64.AppImage"
appimagetool "$APPDIR" "$OUTPUT"

echo "Created: $OUTPUT"
