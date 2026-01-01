#!/bin/bash
# Installation script for cosmic-applet-timeplus
# Usage: sudo ./install.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo -e "${RED}Error: This script must be run as root${NC}"
    echo "Usage: sudo ./install.sh"
    exit 1
fi

echo -e "${GREEN}Installing cosmic-applet-timeplus v0.1.0...${NC}"

# Install binary
echo -e "${YELLOW}→${NC} Installing binary to /usr/bin..."
install -Dm755 cosmic-applet-timeplus /usr/bin/cosmic-applet-timeplus

# Install desktop file
echo -e "${YELLOW}→${NC} Installing desktop file..."
install -Dm644 com.system76.CosmicAppletTimeplus.desktop \
    /usr/share/applications/com.system76.CosmicAppletTimeplus.desktop

# Install icon
echo -e "${YELLOW}→${NC} Installing icon..."
install -Dm644 com.system76.CosmicAppletTimeplus.svg \
    /usr/share/icons/hicolor/scalable/apps/com.system76.CosmicAppletTimeplus.svg

# Update icon cache
if command -v gtk-update-icon-cache &> /dev/null; then
    echo -e "${YELLOW}→${NC} Updating icon cache..."
    gtk-update-icon-cache -f -t /usr/share/icons/hicolor/ 2>/dev/null || true
fi

echo -e "${GREEN}✅ Installation complete!${NC}"
echo ""
echo "To use the applet:"
echo "  1. Restart COSMIC panel: killall cosmic-panel"
echo "  2. Open COSMIC Settings → Panel → Applets"
echo "  3. Add 'Time Plus' to your panel"
echo ""
echo "For development, see: https://github.com/defNickTCys/cosmic-applet-timeplus"
