#!/bin/bash
# Manual release tarball creator for cosmic-applet-timeplus
# Usage: ./create-release.sh

set -e

VERSION="v0.1.0"
RELEASE_DIR="cosmic-applet-timeplus-${VERSION}"
TARBALL="${RELEASE_DIR}-x86_64.tar.gz"

echo "🚀 Creating release tarball for ${VERSION}..."

# Build release binary
echo "→ Building release binary..."
cargo build --release

# Create release directory
echo "→ Creating release directory..."
rm -rf "${RELEASE_DIR}"
mkdir -p "${RELEASE_DIR}"

# Copy files
echo "→ Copying files..."
cp target/release/cosmic-applet-timeplus "${RELEASE_DIR}/"
cp data/com.system76.CosmicAppletTimeplus.desktop "${RELEASE_DIR}/"
cp data/com.system76.CosmicAppletTimeplus.svg "${RELEASE_DIR}/"
cp install.sh "${RELEASE_DIR}/"

# Create README for release
cat > "${RELEASE_DIR}/README.txt" << 'EOF'
cosmic-applet-timeplus v0.1.0
==============================

Installation:
  sudo ./install.sh

Manual Installation:
  sudo install -Dm755 cosmic-applet-timeplus /usr/bin/cosmic-applet-timeplus
  sudo install -Dm644 com.system76.CosmicAppletTimeplus.desktop /usr/share/applications/com.system76.CosmicAppletTimeplus.desktop
  sudo install -Dm644 com.system76.CosmicAppletTimeplus.svg /usr/share/icons/hicolor/scalable/apps/com.system76.CosmicAppletTimeplus.svg
  killall cosmic-panel

More info: https://github.com/defNickTCys/cosmic-applet-timeplus
EOF

# Create tarball
echo "→ Creating tarball..."
tar -czf "${TARBALL}" "${RELEASE_DIR}"

# Cleanup
rm -rf "${RELEASE_DIR}"

# Show result
echo "✅ Release tarball created: ${TARBALL}"
ls -lh "${TARBALL}"
echo ""
echo "Upload to GitHub Releases:"
echo "  https://github.com/defNickTCys/cosmic-applet-timeplus/releases/new"
