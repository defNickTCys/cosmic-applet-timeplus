#!/bin/bash
# Script de instalação standalone para cosmic-applet-timeplus
# Não requer cargo ou just.
set -e

APPLET_NAME="cosmic-applet-timeplus"
DESKTOP_ID="com.system76.CosmicAppletTimePlus"

# Definir prefixo (padrão /usr se rodado como sudo, ou ~/.local se não)
if [ "$EUID" -eq 0 ]; then
    PREFIX="${PREFIX:-/usr}"
    echo "🔧 Instalando no sistema: $PREFIX"
else
    PREFIX="${PREFIX:-$HOME/.local}"
    echo "👤 Instalando para usuário: $PREFIX"
fi

BIN_DIR="$PREFIX/bin"
SHARE_DIR="$PREFIX/share"
APPS_DIR="$SHARE_DIR/applications"
ICONS_DIR="$SHARE_DIR/icons/hicolor/scalable/apps"
SOUNDS_DIR="$SHARE_DIR/cosmic-applet-timeplus/sounds"

# Criar diretórios
mkdir -p "$BIN_DIR"
mkdir -p "$APPS_DIR"
mkdir -p "$ICONS_DIR"

# Instalar binário
# Tenta encontrar o binário localmente (release tarball) ou em target/release (source)
if [ -f "$APPLET_NAME" ]; then
    install -Dm755 "$APPLET_NAME" "$BIN_DIR/$APPLET_NAME"
elif [ -f "target/release/$APPLET_NAME" ]; then
    install -Dm755 "target/release/$APPLET_NAME" "$BIN_DIR/$APPLET_NAME"
else
    echo "❌ Erro: Binário '$APPLET_NAME' não encontrado."
    echo "Se está instalando do código-fonte, execute 'cargo build --release' primeiro."
    exit 1
fi

# Instalar assets
install -Dm644 "data/$DESKTOP_ID.desktop" "$APPS_DIR/$DESKTOP_ID.desktop"
install -Dm644 "data/$DESKTOP_ID.svg" "$ICONS_DIR/$DESKTOP_ID.svg"
install -Dm644 "data/$DESKTOP_ID-symbolic.svg" "$ICONS_DIR/$DESKTOP_ID-symbolic.svg"

# Instalar sons
if [ -d "assets/sounds" ]; then
    mkdir -p "$SOUNDS_DIR"
    install -Dm644 assets/sounds/*.ogg "$SOUNDS_DIR/" 2>/dev/null || true
    # Compatibilidade com wav antigos se existirem
    install -Dm644 assets/sounds/*.wav "$SOUNDS_DIR/" 2>/dev/null || true
fi

# Atualizar cache de ícones (se possível)
if command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -f -t "$SHARE_DIR/icons/hicolor/" 2>/dev/null || true
fi

echo "✅ Instalação concluída!"
echo "Reinicie o painel com: killall cosmic-panel"
