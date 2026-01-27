# Instalação do Cosmic Applet Time Plus

## 📋 Pré-requisitos

- Rust 1.70 ou superior
- COSMIC Desktop Environment
- `just` command runner: `cargo install just`

## 🚀 Instalação Rápida

### Instalação Sistema (Recomendado)

```bash
# 1. Compilar
cargo build --release

# 2. Instalar (requer sudo)
sudo -E env PATH=$PATH just install

# 3. Reiniciar painel COSMIC
killall cosmic-panel
```

### Instalação Usuário (~/.local)

```bash
# 1. Compilar
cargo build --release

# 2. Instalar em ~/.local
just install-user

# 3. Adicionar ~/.local/bin ao PATH (se necessário)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

# 4. Reiniciar painel COSMIC
killall cosmic-panel
```

## 🎯 Verificação

```bash
# Ver se o binário está disponível
which cosmic-applet-timeplus

# Ver se o .desktop foi instalado
ls /usr/share/applications/com.system76.CosmicAppletTimePlus.desktop
# ou para instalação de usuário:
ls ~/.local/share/applications/com.system76.CosmicAppletTimePlus.desktop

# Ver ícones instalados
ls /usr/share/icons/hicolor/scalable/apps/com.system76.CosmicAppletTimePlus*.svg
```

## 🎨 Adicionar ao Painel

1. Abrir **COSMIC Settings**
2. Ir em **Panel** → **Applets**
3. Procurar por **"Time Plus"**
4. Clicar para adicionar ao painel

## 🧪 Modo Desenvolvimento

Para testar sem instalar:

```bash
# Executar direto do código fonte
just run
```

O comando `just run` automaticamente:
- Define `COSMIC_APPLET_TIMEPLUS_DATA` para `./assets`
- Executa `cargo run`

## 🔧 Comandos Úteis

```bash
# Compilar versão release
just build-release

# Executar testes
just test

# Verificar código (sem compilar)
just check

# Linting com clippy
just clippy

# Formatar código
just fmt

# Pipeline CI completo
just ci

# Desinstalar
sudo -E env PATH=$PATH just uninstall
# ou para usuário:
just uninstall-user
```

## ⚠️ Problemas Comuns

### `just: comando não encontrado`

**Problema:** `just` não está instalado

**Solução:**
```bash
cargo install just
```

### `sudo: just: comando não encontrado`

**Problema:** `sudo` não herda o PATH do usuário

**Solução:** Use um dos métodos:
```bash
# Método 1: Preservar PATH
sudo -E env PATH=$PATH just install

# Método 2: Caminho completo
sudo $(which just) install
```

### Applet não aparece no painel

**Problema:** Cache do painel COSMIC

**Solução:**
```bash
killall cosmic-panel
# Aguardar ~2 segundos
# O painel reinicia automaticamente
```

### Binário não encontrado após install-user

**Problema:** `~/.local/bin` não está no PATH

**Solução:**
```bash
# Adicionar ao shell (zsh)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

# Ou (bash)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## 📦 Estrutura de Instalação

### Sistema (`/usr`)

```
/usr/
├── bin/cosmic-applet-timeplus
└── share/
    ├── applications/com.system76.CosmicAppletTimePlus.desktop
    ├── icons/hicolor/scalable/apps/
    │   ├── com.system76.CosmicAppletTimePlus.svg
    │   └── com.system76.CosmicAppletTimePlus-symbolic.svg
    └── cosmic-applet-timeplus/
        └── sounds/
            └── *.wav (futuros arquivos de áudio)
```

### Usuário (`~/.local`)

```
~/.local/
├── bin/cosmic-applet-timeplus
└── share/
    ├── applications/com.system76.CosmicAppletTimePlus.desktop
    └── icons/hicolor/scalable/apps/
        ├── com.system76.CosmicAppletTimePlus.svg
        └── com.system76.CosmicAppletTimePlus-symbolic.svg
```

## 🎨 Sobre os Ícones

O applet inclui dois ícones:

- **com.system76.CosmicAppletTimePlus.svg**: Ícone colorido completo
- **com.system76.CosmicAppletTimePlus-symbolic.svg**: Ícone simbólico (adapta ao tema)

O arquivo `.desktop` usa a versão `-symbolic` para melhor integração com temas do COSMIC.

## 📝 Licença

GPL-3.0-only - Copyright 2023 System76
