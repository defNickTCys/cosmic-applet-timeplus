# Time Plus - Cosmic Applet

**A feature-rich time applet for [COSMIC Desktop](https://github.com/pop-os/cosmic-epoch)** that extends the default time/date/calendar functionality with integrated weather information and pomodoro timer.

<p align="center">
  <img src="https://img.shields.io/badge/COSMIC-Desktop-orange?style=for-the-badge" alt="COSMIC Desktop"/>
  <img src="https://img.shields.io/badge/License-GPL--3.0-blue?style=for-the-badge" alt="GPL-3.0 License"/>
  <img src="https://img.shields.io/badge/Rust-2021-orange?style=for-the-badge&logo=rust" alt="Rust 2021"/>
</p>

[🇧🇷 Leia em Português](README.pt-BR.md)

---

## ✨ Features

### 📅 Calendar (System Standard)
- Full calendar grid with proper localization
- Month navigation
- Today highlighting
- Matches default COSMIC time applet exactly

### 🌤️ Weather Integration *(Coming Soon)*
- Current weather display
- Temperature and conditions
- Location-based forecasts
- Configurable coordinates

### ⏱️ Pomodoro Timer *(Coming Soon)*
- Customizable work/break intervals
- Desktop notifications on completion
- Quick presets (5min, 25min, etc.)
- Persistent state across sessions

---

## 🚀 Installation

### Prerequisites
- COSMIC Desktop Environment
- Rust toolchain (1.70+)
- libcosmic dependencies

### From Source

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/cosmic-applet-timeplus
cd cosmic-applet-timeplus

# Build and install
cargo install --path .

# Restart COSMIC panel
killall cosmic-panel
```

### Adding to Panel

1. Open **COSMIC Settings**
2. Navigate to **Panel** → **Applets**
3. Find **"Time Plus"** in the list
4. Click **Add**

The applet will appear on your panel!

---

## ⚙️ Configuration

Configuration is stored in:
```
~/.config/cosmic/com.system76.CosmicAppletTimePlus/v1/
```

### Current Settings
- `show_date_in_top_panel`: Show date alongside time (default: `true`)
- `military_time`: Auto-detected from system locale
- `show_seconds`: Show seconds in time display (default: `false`)
- `first_day_of_week`: Calendar starting day (0=Sunday, 1=Monday)

---

## 🛠️ Development

### Quick Setup

```bash
cd cosmic-applet-timeplus

# Use the dev script for rapid testing
./dev.sh run    # Build, install, and reload panel
./dev.sh build  # Just build
./dev.sh reload # Just restart panel
```

### Project Structure

```
cosmic-applet-timeplus/
├── src/
│   ├── main.rs       # Entry point
│   ├── lib.rs        # Module declarations
│   ├── window.rs     # Main applet logic
│   ├── config.rs     # Configuration structs
│   ├── localize.rs   # i18n system
│   ├── time.rs       # Calendar helpers
│   ├── weather.rs    # Weather module (WIP)
│   └── timer.rs      # Timer module (WIP)
├── i18n/             # Translations (61 languages)
├── data/             # Desktop files
└── dev.sh            # Development helper script
```

### Adding Features

The applet is built on the official `cosmic-applet-time` foundation, ensuring compatibility and following COSMIC design patterns.

**To extend:**
1. Add new modules in `src/`
2. Update `Message` enum in `window.rs`
3. Implement view functions
4. Add translations to `i18n/`

---

## 🌍 Localization

Time Plus supports **61 languages** out of the box, using the same localization system as the official COSMIC time applet.

Translations are in FluentFormat (`.ftl` files) under `i18n/`.

To add or update translations:
```bash
# Edit the appropriate language file
nano i18n/pt-BR/cosmic_applet_timeplus.ftl

# Rebuild and test
./dev.sh run
```

---

## 📝 Roadmap

### Phase 1: Foundation ✅
- [x] Fork cosmic-applet-time
- [x] Proper project structure
- [x] Build system and dependencies
- [x] Desktop integration
- [x] Panel display with auto-locale

### Phase 2: Tab System 🚧
- [ ] Implement segmented tabs (Calendar | Weather | Timer)
- [ ] Extract calendar to dedicated view
- [ ] Ensure consistent height across tabs

### Phase 3: Weather Module 📍
- [ ] OpenWeatherMap API integration
- [ ] Location configuration
- [ ] Weather display in popup
- [ ] Mini weather widget on panel

### Phase 4: Timer Module ⏱️
- [ ] Countdown timer logic
- [ ] Preset management
- [ ] Desktop notifications
- [ ] Mini timer widget on panel

### Phase 5: Polish 💎
- [ ] Settings UI
- [ ] Keyboard shortcuts
- [ ] Accessibility improvements
- [ ] Performance optimization

---

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Code Style
- Follow Rust standard formatting (`cargo fmt`)
- Run clippy checks (`cargo clippy`)
- Ensure builds pass (`cargo build --release`)
- Test on actual COSMIC environment

---

## 📜 License

This project is licensed under the **GNU General Public License v3.0** - see [LICENSE](LICENSE) file for details.

Based on [cosmic-applet-time](https://github.com/pop-os/cosmic-applets) by System76.

---

## 🙏 Acknowledgments

- **System76** for COSMIC Desktop and the base time applet
- **Pop!_OS** team for libcosmic framework
- **Iced** for the GUI toolkit
- The **Rust** community

---

## 📫 Support & Contact

- **Issues**: [GitHub Issues](https://github.com/YOUR_USERNAME/cosmic-applet-timeplus/issues)
- **Discussions**: [GitHub Discussions](https://github.com/YOUR_USERNAME/cosmic-applet-timeplus/discussions)

---

<p align="center">
Made with ❤️ for the COSMIC Desktop community
</p>
