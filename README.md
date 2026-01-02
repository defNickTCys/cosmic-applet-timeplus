<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="com.system76.CosmicAppletTimePlusDark.svg">
    <source media="(prefers-color-scheme: light)" srcset="com.system76.CosmicAppletTimePlusLight.svg">
    <img src="com.system76.CosmicAppletTimePlusLight.svg" alt="Time Plus Logo" width="120">
  </picture>
</p>

# Time Plus - Cosmic Applet

**A feature-rich time applet for [COSMIC Desktop](https://github.com/pop-os/cosmic-epoch)** that extends the default time/date/calendar functionality with integrated weather information and pomodoro timer.

<p align="center">
  <img src="https://img.shields.io/badge/COSMIC-Desktop-orange?style=for-the-badge" alt="COSMIC Desktop"/>
  <img src="https://img.shields.io/badge/License-GPL--3.0-blue?style=for-the-badge" alt="GPL-3.0 License"/>
  <img src="https://img.shields.io/badge/Rust-2021-orange?style=for-the-badge&logo=rust" alt="Rust 2021"/>
  <img src="https://img.shields.io/badge/Start-Vibe%20Coding-purple?style=for-the-badge" alt="Vibe Coding"/>
</p>

[🇧🇷 Leia em Português](README.pt-BR.md)

---

## 📸 Screenshots

*All screenshots captured from **v0.1.1** running on COSMIC Desktop (Fedora Linux 43)*

<details>
<summary>🔲 Tab Navigation System</summary>

<p align="center">
  <img src="screenshots/tabs.png" alt="Tab Navigation" width="600"/>
</p>

Segmented button navigation showing Calendar, Weather, and Timer tabs.
</details>

<details>
<summary>📅 Calendar Tab</summary>

<p align="center">
  <img src="screenshots/calendar.png" alt="Calendar Tab" width="400"/>
</p>

Full calendar grid with month navigation and today highlighting.
</details>

<details>
<summary>🌤️ Weather Tab (Placeholder)</summary>

<p align="center">
  <img src="screenshots/weather.png" alt="Weather Tab" width="400"/>
</p>

Weather module ready for API integration.
</details>

<details>
<summary>⏱️ Timer Tab (Placeholder)</summary>

<p align="center">
  <img src="screenshots/timer.png" alt="Timer Tab" width="400"/>
</p>

Timer module ready for countdown logic.
</details>

---

## 🎨 Visual Anatomy

The **Time Plus** design strictly follows the **Human Interface Guidelines (HIG)** of COSMIC Desktop, ensuring a native and integrated look.

### 1. Panel Integration
The part of the applet that resides permanently on the top bar.

*   **Style:** Flat button (`Button::Text`) integrated into the panel surface.
    *   *Inactive:* Transparent background, text `OnBackground`.
    *   *Active:* Highlighted background indicating open menu.
*   **Content:** Full Date and Time (e.g., "Wed, Dec 31 03:59").
    *   **Format:** Auto-detected from system locale (12h/24h).
    *   **Typography:** Inter Semi-bold, adjusted to panel height.

### 2. Main Interface (Popup)
Floating container with rounded corners (Corner Radius 12px) and standard `Surface` background.

#### A. Top Navigation (Tab System)
Located at the absolute top of the container.
*   **Component:** `segmented_button::horizontal` with `SingleSelectModel`.
*   **Style:**
    *   *Active:* Highlighted background (Accent Color), high-contrast text and icon.
    *   *Inactive:* Transparent background, gray elements (`OnSurfaceVariant`).
*   **Tabs:**
    *   📅 **Calendar:** Icon `com.system76.CosmicAppletTime-symbolic`
    *   🌤️ **Weather:** Icon `weather-clear-symbolic`
    *   ⏰ **Timer:** Icon `alarm-symbolic`

#### B. Content Area (Calendar)
*   **Header:** Prominent Month/Year (`text::Title`, size 18) and navigation controls (`button::icon`) on the right.
*   **Day Grid:**
    *   Weekdays ("Mon", "Tue"...) in smaller text (`text::Caption`).
    *   Current Day highlighted with a **Perfect Circle** filled with accent color (Cyan) and high-contrast text.

#### C. Footer
*   **Divider:** Subtle horizontal line separating content.
*   **Settings:** `menu_button` style ("Date, time & calendar settings...") filling width and reacting to hover.

---

## ✨ Features

### 🏗️ Modular Architecture
- **Separate modules** for Calendar, Weather, and Timer
- Clean separation of concerns
- Easy to extend and maintain
- Follows COSMIC applet patterns

### 📅 Calendar
- Full calendar grid with proper localization
- Month navigation with ICU formatters
- Today highlighting with accent color
- Optimized rendering with formatter caching
- Accessible via dedicated "Calendar" tab

### 🌤️ Weather *(Placeholder)*
- Modular `weather.rs` implementation
- Consistent header + content structure
- Standard COSMIC divider
- Ready for API integration
- *Coming Soon:* Current weather, forecasts, location config

### ⏱️ Timer *(Placeholder)*
- Modular `timer.rs` implementation
- Matches calendar visual consistency
- Standard COSMIC patterns
- Ready for countdown logic
- *Coming Soon:* Pomodoro presets, notifications, persistence

---

## 🏗️ Software Architecture

### Neutral Messenger + Orchestrator Pattern

Time Plus follows a clean **Neutral Messenger + Orchestrator + Specialist Modules** architecture introduced in v0.1.1:

```mermaid
graph TD
    %% Estilos de Cores
    classDef neutral fill:#2d333b,stroke:#adbac7,color:#adbac7,stroke-width:2px;
    classDef orchestrator fill:#1e4273,stroke:#58a6ff,color:#fff,stroke-width:4px;
    classDef module fill:#238636,stroke:#2ea043,color:#fff;
    classDef logic fill:#d29922,stroke:#e3b341,color:#000;

    subgraph "Core Infrastructure"
        LIB(lib.rs<br/>Neutral Messenger):::neutral
    end

    subgraph "Event Inputs"
        SUB(subscriptions.rs<br/>System Sensors):::logic
    end

    subgraph "Main Logic"
        WIN(window.rs<br/>Orchestrator):::orchestrator
    end

    subgraph "Specialized Modules"
        CAL(calendar.rs<br/>Calendar):::module
        WEA(weather.rs<br/>Weather):::module
        TIM(timer.rs<br/>Timer):::module
    end

    subgraph "Shared Utilities"
        TIME(time.rs<br/>Panel Formatter):::logic
    end

    %% Fluxo de Dados
    SUB -->|Emits Message| WIN
    WIN -->|Delegates View| CAL
    WIN -->|Delegates View| WEA
    WIN -->|Delegates View| TIM
    WIN -->|Uses| TIME

    %% Relação com o Mensageiro (Tipos)
    CAL -.->|References| LIB
    WEA -.->|References| LIB
    TIM -.->|References| LIB
    WIN -.->|References| LIB
    TIME -.->|References| LIB

```

### Message Envelope Pattern

Each module has its own **isolated message system**:

```rust
// Global message envelope in lib.rs (Neutral Messenger)
pub enum Message {
    Calendar(calendar::CalendarMessage),  // Envelope for calendar
    Weather(weather::WeatherMessage),     // Envelope for weather
    Timer(timer::TimerMessage),           // Envelope for timer
    // ... only orchestration messages
}

// Module-specific messages in calendar.rs
pub enum CalendarMessage {
    SelectDay(u32),
    PreviousMonth,
    NextMonth,
}

// Module handles its own state
impl CalendarState {
    pub fn update(&mut self, msg: CalendarMessage) {
        // All calendar logic here
    }
}
```

**Benefits:**
- ✅ **Encapsulation**: Each module is self-contained
- ✅ **Maintainability**: Changes to one module don't affect others
- ✅ **Testability**: Modules can be tested independently
- ✅ **Scalability**: Easy to add new modules
- ✅ **No Circular Dependencies**: Neutral Messenger breaks dependency cycles

### Design Principles

#### 🎯 Native by Default

**The applet prioritizes COSMIC native solutions over custom implementations:**

- **Widgets**: Use `libcosmic` components (`segmented_button`, `padded_control`, etc.)
- **System APIs**: Integrate with COSMIC daemons (notifications, settings, etc.)
- **Styling**: Follow COSMIC HIG strictly (spacing, colors, typography)
- **Patterns**: Match official applets' architecture and code style

**Example:**
```rust
// ✅ GOOD: Use native COSMIC widget
let tabs = segmented_button::horizontal(&self.tab_model)
    .on_activate(Message::TabActivated);

// ❌ BAD: Custom tab implementation
let tabs = custom_tab_widget();
```

#### 🧩 Separation of Concerns

- **lib.rs**: Neutral message envelope (no dependencies)
- **window.rs**: Window management + tab orchestration ONLY
- **Modules**: Complete ownership of their domain (state + logic + view)
- **subscriptions.rs**: Heavy async logic (time, timezone, wake-from-sleep)
- **time.rs**: Panel time formatting (PanelFormatter)
- **No cross-module dependencies**: Modules never import each other

#### 📦 Single Responsibility

Each file has ONE clear purpose:
- `lib.rs` → Neutral Messenger (Message + Tab enums)
- `window.rs` → Popup window orchestration (369 lines, -48% from v0.1.0)
- `calendar.rs` → Calendar functionality (state + view + logic)
- `time.rs` → Panel time formatting (PanelFormatter)
- `subscriptions.rs` → Subscription management (time tick, timezone, wake-from-sleep)
- `config.rs` → Configuration management
- `localize.rs` → Internationalization + system locale detection

---

## 🤖 Development Philosophy

This project is an experiment in **"Vibe Coding"** (Assisted Development) - a collaboration between human creativity and AI precision.

- **Human**: Thiago Cysneiros ([@defNickTCys](https://github.com/defNickTCys)) - Architecture, Design Decisions, Testing
- **AI**: Google Antigravity IDE & Claude 4.5 Sonnet - Implementation, Refactoring, Documentation

The goal is to demonstrate how advanced AI tools can accelerate modern desktop development while maintaining high standards of code quality and following strict architectural patterns.

---

## 🚀 Installation

### Prerequisites
- COSMIC Desktop Environment
- Rust toolchain (1.70+)
- libcosmic dependencies

### From Source

```bash
# Clone the repository
git clone https://github.com/defNickTCys/cosmic-applet-timeplus
cd cosmic-applet-timeplus

# Build release binary
cargo build --release

# Install to system
sudo install -Dm755 target/release/cosmic-applet-timeplus /usr/bin/cosmic-applet-timeplus
sudo install -Dm644 data/com.system76.CosmicAppletTimeplus.desktop /usr/share/applications/com.system76.CosmicAppletTimeplus.desktop
sudo install -Dm644 data/com.system76.CosmicAppletTimeplus.svg /usr/share/icons/hicolor/scalable/apps/com.system76.CosmicAppletTimeplus.svg

# Restart COSMIC panel
killall cosmic-panel
```

**Note**: For development, use `./dev.sh dev` for fast iteration without system installation.

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

### Development Script (`dev.sh`)

The project includes an optimized development script with smart Git dependency management and multiple commands for different workflows.

#### Quick Commands

```bash
# 🚀 Development (Fast iteration)
./dev.sh dev        # Debug build + install + reload (~15s, no Git updates)
./dev.sh check      # Quick code verification (no compilation)
./dev.sh test       # Run unit tests
./dev.sh clippy     # Run Rust linter

# 📦 Release
./dev.sh run        # Release build + install + reload (smart Git updates)
./dev.sh build      # Build release binary only
./dev.sh install    # Install to ~/.cargo/bin (smart Git updates)
./dev.sh reload     # Restart cosmic-panel only

# 🛠️ Utility
./dev.sh clean        # Remove build artifacts
./dev.sh force-update # Force Git dependency update
```

#### Smart Git Updates

The script automatically manages dependency updates:
- **First run of the day**: Full update with Git dependencies (~3min)
- **Subsequent runs**: Fast mode with `--locked` (~1min)
- **Manual override**: Use `force-update` to refresh dependencies

This optimization reduces development cycle time by **~60%** on subsequent builds.

#### Recommended Workflow

```bash
# Initial setup (once per day)
./dev.sh run

# Fast iteration during development
./dev.sh dev    # Make changes, test immediately

# Before committing
./dev.sh clippy # Check code quality
./dev.sh test   # Run tests
```

### Project Structure

```
cosmic-applet-timeplus/
├── src/
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Neutral Messenger (Message + Tab enums)
│   ├── window.rs        # Orchestrator (369 lines, -48% from v0.1.0)
│   ├── config.rs        # Configuration structs
│   ├── localize.rs      # i18n system + system locale detection
│   ├── calendar.rs      # Calendar module (view + logic + state)
│   ├── time.rs          # Panel time formatting (PanelFormatter)
│   ├── subscriptions.rs # Subscription management (time, timezone, wake)
│   ├── weather.rs       # Weather module (placeholder)
│   └── timer.rs         # Timer module (placeholder)
├── i18n/                # Translations (61 languages)
│   └── */cosmic_applet_timeplus.ftl
├── screenshots/         # UI screenshots
│   ├── calendar.png
│   ├── weather.png
│   └── timer.png
├── data/                # Desktop files
├── dev.sh               # Development helper script
├── create_i18n.sh       # i18n file generator
└── TRANSLATIONS.md      # Translation status
```

**Key Architectural Decisions (v0.1.1):**
- **Neutral Messenger Pattern**: `lib.rs` breaks circular dependencies
- **Modular Design**: Each tab has its own module (`calendar.rs`, `weather.rs`, `timer.rs`)
- **Separation of Concerns**: `window.rs` orchestrates (369 lines), modules implement
- **Subscription Isolation**: Heavy async logic in dedicated `subscriptions.rs` (166 lines)
- **Panel Formatting**: Dedicated `time.rs` with `PanelFormatter` (222 lines)
- **No Code Duplication**: Uses `cosmic::applet::padded_control` and standard patterns
- **Consistent Structure**: All placeholders match calendar's header + content layout

### Performance Optimizations

Recent improvements include:
- **ICU Formatter Caching**: ~94% reduction in calendar rendering time
- **Consolidated Helpers**: Eliminated code duplication
- **Named Constants**: Improved code readability and maintainability



---

## 🌍 Localization

Time Plus supports **61 languages** out of the box!

**Translation Status:**
- ✅ **8 languages** fully translated (de, es-ES, fr, it, ru, ja, zh-CN, ko)
- 📝 **53 languages** using English fallback
- 🤝 **Community contributions welcome!**

📋 **See [TRANSLATIONS.md](TRANSLATIONS.md) for:**
- Complete language list with native speaker counts
- Translation guidelines
- How to contribute
- Priority languages

**Quick Translation:**
```bash
# Edit your language file
nano i18n/{language}/cosmic_applet_timeplus.ftl

# Test locally
./dev.sh dev

# Submit PR with your translation!
```

---

## 📝 Roadmap

### Phase 1: Foundation ✅
- [x] Fork cosmic-applet-time
- [x] Proper project structure
- [x] Build system and dependencies
- [x] Desktop integration
- [x] Panel display with auto-locale

### Phase 2: Tab System ✅
- [x] Implement segmented tabs (Calendar | Weather | Timer)
- [x] Extract calendar to `time.rs` module
- [x] Create `weather.rs` and `timer.rs` modules
- [x] Consistent visual style with standard COSMIC patterns
- [x] Content-driven height (no fixed dimensions)
- [x] Standard dividers with proper spacing

### Phase 2.5: Calendar Modularization ✅
- [x] Create `CalendarState` for state encapsulation
- [x] Create `CalendarMessage` enum for calendar interactions
- [x] Implement message envelope pattern (`Message::Calendar`)
- [x] Move all calendar logic to `time.rs`
- [x] Transform `window.rs` into pure orchestrator
- [x] Follow official cosmic-applet-time patterns
- [x] Zero warnings compilation

### Phase 3: Infrastructure Refactoring ✅ *v0.1.1*
- [x] **Rename** `time.rs` → `calendar.rs` (better semantic clarity)
- [x] **Move** `Message` and `Tab` enums from `window.rs` to `lib.rs` (Neutral Messenger)
- [x] **Move** `get_system_locale()` from `window.rs` to `localize.rs`
- [x] **Create** `subscriptions.rs` for heavy async logic (time, timezone, wake-from-sleep)
- [x] **Create** `time.rs` for panel formatting (`PanelFormatter`)
- [x] **Reduce** `window.rs` from 704 to 369 lines (-48%)
- [x] **Fix** APP_ID to use `com.system76.CosmicAppletTime` for config sync
- [x] **Fix** HourCycle configuration for military_time
- [x] **Fix** Real-time configuration updates (show_seconds, military_time)
- [x] **Optimize** Date format to use `MDT::medium` for better space usage

### Phase 4: Weather Module 🌤️ *NEXT*
- [ ] OpenWeatherMap API integration
- [ ] Location configuration
- [ ] Weather display in popup
- [ ] Mini weather widget on panel

### Phase 5: Timer Module ⏱️
- [ ] Countdown timer logic
- [ ] Preset management
- [ ] Desktop notifications
- [ ] Mini timer widget on panel

### Phase 6: Quick Reminders 📝
- [ ] Date-based reminder storage
- [ ] Visual indicators on calendar
- [ ] Add/edit/delete UI
- [ ] Desktop notifications when due

### Phase 7: Polish 💎
- [ ] Settings UI
- [ ] Keyboard shortcuts
- [ ] Accessibility improvements

---

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📜 License

This project is licensed under the **GNU General Public License v3.0** - see [LICENSE](LICENSE) file for details.

Based on [cosmic-applet-time](https://github.com/pop-os/cosmic-applets) by System76.

---

## 🙏 Acknowledgments

- **Dio Linux** for convincing me of the adventure of coming to the Linux world!
- **Google Antigravity & Claude 3.5 Sonnet** - AI Assistance
- **System76** for COSMIC Desktop and the base time applet
- **Pop!_OS** team for libcosmic framework

---

## 📫 Support & Contact

- **Issues**: [GitHub Issues](https://github.com/defNickTCys/cosmic-applet-timeplus/issues)
- **Discussions**: [GitHub Discussions](https://github.com/defNickTCys/cosmic-applet-timeplus/discussions)

---

<p align="center">
Made with ❤️ and 🤖 for the COSMIC Desktop community
</p>
