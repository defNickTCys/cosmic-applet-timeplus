#!/bin/bash
# Development script for cosmic-applet-timeplus
# Optimized version with smart Git update detection

PROJECT_DIR="$HOME/Template Cosmic Applet/cosmic-applet-template/cosmic-applet-timeplus"
cd "$PROJECT_DIR" || exit 1

UPDATE_MARKER=".last_git_update"

# Check if Git updates were done today
should_update_git() {
    if [ -f "$UPDATE_MARKER" ]; then
        last_update=$(cat "$UPDATE_MARKER")
        today=$(date +%Y-%m-%d)
        if [ "$last_update" = "$today" ]; then
            return 1  # Skip update
        fi
    fi
    return 0  # Do update
}

# Mark Git update as done today
mark_git_updated() {
    date +%Y-%m-%d > "$UPDATE_MARKER"
}

case "$1" in
    check)
        echo "🔍 Checking code (no compilation)..."
        cargo check
        ;;
    
    test)
        echo "🧪 Running tests..."
        cargo test
        ;;
    
    clippy)
        echo "📎 Running clippy linter..."
        cargo clippy -- -W clippy::all
        ;;
    
    dev)
        echo "⚡ Fast dev iteration (no Git updates)..."
        cargo build --release && \
        cargo install --path . --locked && \
        killall cosmic-panel
        echo "✅ Dev iteration complete!"
        ;;
    
    build)
        echo "🔨 Building release..."
        cargo build --release
        ;;
    
    install)
        echo "📦 Installing to ~/.cargo/bin/..."
        if should_update_git; then
            echo "📥 Updating dependencies (first run today)..."
            cargo install --path .
            mark_git_updated
        else
            echo "⚡ Using cached dependencies (already updated today)..."
            cargo install --path . --locked
        fi
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
        cargo build --release && {
            if should_update_git; then
                echo "📥 Full install with dependency updates (first run today)..."
                cargo install --path .
                mark_git_updated
            else
                echo "⚡ Fast install (dependencies already updated today)..."
                cargo install --path . --locked
            fi
        } && \
        killall cosmic-panel
        echo "✅ Done!"
        ;;
    
    clean)
        echo "🧹 Cleaning build artifacts..."
        cargo clean
        rm -f "$UPDATE_MARKER"
        echo "✅ Cleaned!"
        ;;
    
    force-update)
        echo "🔄 Forcing dependency update..."
        rm -f "$UPDATE_MARKER"
        cargo update
        mark_git_updated
        echo "✅ Dependencies updated!"
        ;;
    debug)
        echo "⚡ Fast Debug Build (mold enabled via config)..."
        cargo build && \
        cargo install --path . --debug --locked && \
        killall cosmic-panel
        echo "✅ Debug build reloaded!"
        ;;
    debug-panel)
        echo "⚡ Compilando DEBUG e injetando no Painel..."
        # Instala a versão de debug na pasta global (~/.cargo/bin)
        # O --debug diz para não otimizar (compilação rápida)
        cargo install --path . --debug --locked
        
        echo "🔄 Reiniciando painel..."
        killall cosmic-panel
        
        # Opcional: Já deixa o comando de log pronto para você copiar/rodar
        echo "✅ Painel reiniciado com Applet (Debug Mode)!"
        echo "📝 Para ver os logs em tempo real, rode em outra aba:"
        echo "   journalctl -f -b --user | grep cosmic-applet-timeplus"
        ;;
    
*)
        echo "Usage: $0 {check|test|clippy|dev|debug|debug-panel|build|install|run|reload|clean|force-update}"
        echo ""
        echo "🚀 Development commands:"
        echo "  check        - Quick code verification (no compilation)"
        echo "  test         - Run unit tests"
        echo "  clippy       - Run Rust linter"
        echo "  debug        - Fast compile & install (Debug Mode + mold)"
        echo "  debug-panel  - Debug + Restart Panel (View logs with journalctl)"
        echo "  dev          - Release Mode iteration (Optimized binary)"
        echo ""
        echo "📦 Release commands:"
        echo "  build        - Build optimized release binary"
        echo "  install      - Install to ~/.cargo/bin/ (smart Git updates)"
        echo "  run          - Build + install + reload (smart Git updates)"
        echo "  reload       - Restart cosmic-panel only"
        echo ""
        echo "🛠️  Utility:"
        echo "  clean        - Remove build artifacts and update cache"
        echo "  force-update - Force Git dependency update"
        echo ""
        exit 1
        ;;
esac
