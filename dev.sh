#!/bin/bash
# Development script for cosmic-applet-timeplus
# Usage: ./dev.sh [build|install|run|reload]

PROJECT_DIR="$HOME/Template Cosmic Applet/cosmic-applet-template/cosmic-applet-timeplus"
cd "$PROJECT_DIR" || exit 1

case "$1" in
    build)
        echo "🔨 Building release..."
        cargo build --release
        ;;
    install)
        echo "📦 Installing to ~/.cargo/bin/..."
        cargo install --path .
        echo "📋 Installing desktop file..."
        mkdir -p ~/.local/share/applications
        cp data/com.system76.CosmicAppletTimePlus.desktop ~/.local/share/applications/
        update-desktop-database ~/.local/share/applications/ 2>/dev/null || true
        ;;
    reload)
        echo "🔄 Reloading cosmic-panel..."
        killall cosmic-panel
        echo "✅ Panel reloaded!"
        ;;
    run)
        echo "🚀 Build, install, and reload..."
        cargo build --release && \
        cargo install --path . && \
        killall cosmic-panel
        echo "✅ Done!"
        ;;
    *)
        echo "Usage: $0 {build|install|run|reload}"
        echo ""
        echo "  build   - Build release binary"
        echo "  install - Install to ~/.cargo/bin/"
        echo "  reload  - Restart cosmic-panel"
        echo "  run     - Build + install + reload (one command)"
        exit 1
        ;;
esac
