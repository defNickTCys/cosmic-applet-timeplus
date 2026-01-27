# Release v0.1.4 - Phase 3.8: SSoT & Universal Portability

## 🎯 Overview
This release finalizes **Phase 3.8**, achieving **100% Single Source of Truth (SSoT)** for assets and **Universal Portability** via XDG compliance. It introduces a production-grade **build system (justfile)** and upgrades the codebase to **Rust 2024**, setting a solid foundation for system integration.

## ✨ New Features - Phase 3.8

**🎨 Single Source of Truth (SSoT)**
- **Centralized Icons**: New `src/icons.rs` module manages all icon names.
- **Zero Hardcoded Strings**: Eliminated all 12 hardcoded strings from UI modules.
- **Smart Tab Methods**: `Tab::icon_name()` and `Tab::label()` (now fully localized!).
- **Audio Standard**: Standardized on `.ogg` (FreeDesktop spec) replacing `.wav`.

**🏗️ Universal Portability & Build**
- **XDG Compliance**: New `src/paths.rs` implements stricter XDG Base Directory logic.
  - Dev override: `COSMIC_APPLET_TIMEPLUS_DATA`
  - User: `$XDG_DATA_HOME` or `~/.local/share`
  - System: `/usr/share`
- **Production Build System**: Adopted `just` (Justfile) following System76 standards.
  - Supports `PREFIX` and `DESTDIR` for packaging.
  - Automatic `.tar.gz` generation for releases.
  - Targets: `install`, `install-user`, `uninstall`, `dist`.

**📦 Installation Improvements**
- **Icon Cache**: Auto-update trigger after installation.
- **Theme Integration**: Added symbolic icon variants.
- **Clean Uninstall**: `uninstall` targets remove all traces.

## 🔄 Changed

**Codebase Upgrades**
- **Rust 2024**: Edition upgrade for modern language features.
- **Safety**: Correct implementation of `env::set_var` (unsafe) for 2024 compliance.
- **Localization**: Fixed `Tab::label()` using `fl!` macro for proper translation.

## 🐛 Fixed
- **Tab Localization**: Tabs (Calendar, Weather, Timer) now respect system locale.
- **Tests**: Fixed doctests import and audio format assertions.
- **Documentation**: Updated roadmap references (3.7 → 3.9).

## 🏗️ Architecture Impacts

**New Components**:
- `src/paths.rs`: Portable asset resolution.
- `src/icons.rs`: Icon constant definitions.

**Refactored**:
- `src/lib.rs`: `Tab` enum now handles localization logic directly.
- `src/window.rs`, `calendar.rs`, `timer.rs`, `weather.rs`: Consuming SSoT.

## 📊 Quality Assurance
- ✅ **100% Test Pass Rate** (3/3 tests)
- ✅ **Zero Clippy Warnings**
- ✅ **Verified Clean Install** (System & User)
- ✅ **Rust 2024 Compliant**

---

## 📥 Installation

### � Binary Release (Recommended for Users)
No Rust or Cargo required.

1. Download the `.tar.gz` from Releases.
2. Extract and install:

```bash
tar -xzf cosmic-applet-timeplus-0.1.4.tar.gz
cd cosmic-applet-timeplus
sudo ./install.sh
# Check with: killall cosmic-panel
```

### �️ From Source (Developers)
Requires Rust and Cargo.

```bash
git clone https://github.com/defNickTCys/cosmic-applet-timeplus
cd cosmic-applet-timeplus
git checkout v0.1.4

# Build and Install
cargo build --release
sudo -E env PATH=$PATH just install
```

---

## 🗑️ Uninstall
```bash
# System
sudo -E env PATH=$PATH just uninstall

# User
just uninstall-user
```
