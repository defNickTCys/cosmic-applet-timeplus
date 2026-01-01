# 🎉 Time Plus v0.1.0 - Tab System Complete

First official release of **cosmic-applet-timeplus**! A feature-rich time applet for COSMIC Desktop with integrated calendar, weather, and timer functionality.

## ✨ What's New

### 🏗️ Three-Tab Interface
- **📅 Calendar** - Full month grid with ICU formatters, today highlighting, and month navigation
- **🌤️ Weather** - Modular placeholder ready for OpenWeatherMap API integration
- **⏱️ Timer** - Modular placeholder ready for Pomodoro countdown logic

### 🎨 Modular Architecture
- Clean separation: `time.rs`, `weather.rs`, `timer.rs` modules
- COSMIC HIG compliant design with standard patterns
- Content-driven height (no fixed dimensions)
- Optimized rendering with ICU formatter caching (~94% faster)

### 🌍 Internationalization
- **61 languages** supported out of the box
- **8 fully translated**: German, Spanish, French, Italian, Russian, Japanese, Chinese, Korean
- **53 with English fallback** - community contributions welcome!

### 🛠️ Developer Experience
- Optimized `dev.sh` script with smart Git dependency updates
- Comprehensive documentation in English and Portuguese
- Zero compilation warnings
- Clean, well-documented codebase

## 📥 Installation

### Quick Install (Recommended)

```bash
# Download and extract
tar -xzf cosmic-applet-timeplus-v0.1.0-x86_64.tar.gz
cd cosmic-applet-timeplus-v0.1.0

# Run installer
sudo ./install.sh

# Restart COSMIC panel
killall cosmic-panel
```

### Adding to Panel

1. Open **COSMIC Settings**
2. Navigate to **Panel** → **Applets**
3. Find **"Time Plus"** in the list
4. Click **Add**

### From Source

```bash
git clone https://github.com/defNickTCys/cosmic-applet-timeplus
cd cosmic-applet-timeplus
git checkout v0.1.0
cargo build --release
sudo install -Dm755 target/release/cosmic-applet-timeplus /usr/bin/cosmic-applet-timeplus
```

See [README.md](https://github.com/defNickTCys/cosmic-applet-timeplus#installation) for detailed instructions.

## 🎯 What's Next

### v0.2.0 - Weather Module (Planned)
- OpenWeatherMap API integration
- Location configuration
- Current weather display
- 5-day forecast

### v0.3.0 - Timer Module (Planned)
- Countdown timer with presets
- Pomodoro technique support
- Desktop notifications
- Persistent state

### v0.4.0 - Reminders (Planned)
- Date-based reminder system
- Calendar integration
- Notification system

## 📊 Project Stats

- **141 files changed**
- **2,277 insertions**
- **Zero warnings**
- **61 language files**
- **~94% performance improvement** in calendar rendering

## 🤝 Contributing

We welcome contributions! See [TRANSLATIONS.md](https://github.com/defNickTCys/cosmic-applet-timeplus/blob/main/TRANSLATIONS.md) for translation guidelines.

## 📜 License

GPL-3.0-only - Based on [cosmic-applet-time](https://github.com/pop-os/cosmic-applets) by System76

---

**Full Changelog**: https://github.com/defNickTCys/cosmic-applet-timeplus/blob/main/CHANGELOG.md

**Tested on**: Fedora Linux 43 (COSMIC)
