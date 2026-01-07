# Changelog

All notable changes to cosmic-applet-timeplus will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Weather module with OpenWeatherMap API integration
- Timer module with Pomodoro presets
- Date-based reminders system
- Desktop notifications

## [0.1.3] - 2026-01-07

### Added - Phase 3.6: Infrastructure Refactoring & Observability

**🔧 CLI Foundation**
- Command-line interface with `clap` derive
  - `--config <path>` - Custom configuration file (placeholder)
  - `--debug` - Enable verbose logging with filtered output
  - `--help` / `--version` - Standard help and version info
- Smart logging configuration with `RUST_LOG` environment variable
  - Debug mode: `cosmic_applet_timeplus=debug,i18n_embed=warn,wgpu=warn`
  - Reduces noise from dependencies (~60 lines of i18n errors eliminated)

**📊 Comprehensive Observability**
- Initialization logging with full context (panel position, locale, timezone, config)
- User interaction tracing categories: `[UI]`, `[Navigation]`, `[Calendar]`, `[System]`
- Calendar logs with full YYYY-MM-DD context
- Proper log levels (INFO/DEBUG/WARN)

**🏗️ Architecture Improvements**
- Neutral Messenger pattern fully implemented
- Dependency injection via `Flags` parameter  
- Immutable panel position captured at initialization
- Centralized validation (`config.rs`) and parsing logic (`time.rs`)

### Changed - Code Organization

**Module Updates**
- `src/config.rs`: Added `should_show_seconds()`, `validate_format()`, `is_24_hour()`
- `src/time.rs`: Added `parse_timezone()`, `system_timezone()`
- `src/lib.rs`: Public `config` module, notification message placeholders
- `src/main.rs`: CLI parsing, conditional logging
- `src/window.rs`: Dependency injection, comprehensive tracing, enriched calendar logs

### Fixed

**i18n Cleanup** - Removed duplicate keys from 61 locale files (122 lines)
**Error Handling** - Wayland errors now WARN instead of ERROR

**Commits**: 14 atomic commits with semantic versioning

## [0.1.2] - 2026-01-04

### Added - Phase 3.8: UI Architecture & Separation

**🎨 Core UI Layer** - New architectural layer for UI rendering
- `src/panel.rs` (195 lines) - Panel UI module
  - `vertical_layout()` - Renders panel button for vertical panel orientation
  - `horizontal_layout()` - Renders panel button for horizontal panel orientation
  - `view()` - Main entry point that delegates to appropriate layout
- `src/popup.rs` (83 lines) - Popup UI module
  - `view()` - Renders complete popup structure (tabs, content, footer)
  - Handles tab navigation with `segmented_button`
  - Delegates tab content to feature modules

### Changed - Architecture Refactoring

**🧠 Pure Orchestrator Pattern** (`window.rs`)
- Reduced from 369 to 334 lines (-9% further reduction)
- `view()` method now delegates to `panel::view()` (removed inline UI construction)
- `view_window()` method now delegates to `popup::view()` (removed inline UI construction)
- Maintains only state management, message handling, and application lifecycle
- Zero inline widget construction - pure orchestration

**⚙️ Pure Utilities Pattern** (`time.rs`)
- Reduced from 222 to 84 lines (-62% reduction)
- Removed all UI dependencies (`cosmic::iced::widget`, `cosmic::widget`, `Element`)
- Extracted `vertical_layout()` and `horizontal_layout()` to `panel.rs`
- Now contains only data formatting logic
- Added `locale()` getter for panel module access
- Pure utility module with zero UI coupling

**📋 Module Registration** (`lib.rs`)
- Added `mod panel` - Panel UI module
- Added `mod popup` - Popup UI module

### Technical Improvements

**Separation of Concerns**
- ✅ **UI Layer** (`panel.rs`, `popup.rs`) - Widget construction and visual logic
- ✅ **Orchestration** (`window.rs`) - State, messages, lifecycle
- ✅ **Utilities** (`time.rs`) - Data formatting, no UI dependencies
- ✅ **Features** (`calendar.rs`, `weather.rs`, `timer.rs`) - Domain-specific content

**Code Quality**
- All new modules include GPL-3.0 headers and System76 copyright
- Comprehensive doc comments on all public functions and modules
- Zero compilation warnings or clippy errors
- Visual logic preserved exactly (no UI/UX changes)

### Performance
- Memory usage: Stable at 30-33MB (within baseline range)
- Build time (release): ~1m 32s
- All functional tests passing
- Configuration updates work in real-time

## [0.1.1] - 2026-01-02

### Added
- Neutral Messenger pattern for better modularity (`lib.rs`)
- `subscriptions.rs` module (166 lines) for subscription management
- `time.rs` module (222 lines) for panel time formatting with `PanelFormatter`
- Pre-commit hooks for automatic code quality checks (`cargo fmt`, `cargo clippy`)
- Explicit HourCycle configuration for proper 24h/12h format handling

### Changed
- Reduced `window.rs` from 704 to 369 lines (-48% reduction)
- Improved date format to use `MDT::medium` for better space usage ("2 de jan., 02:39")
- Refactored architecture to Orchestrator + Specialist Modules pattern

### Fixed
- APP_ID now uses `com.system76.CosmicAppletTime` for proper config synchronization
- Real-time configuration updates for `show_seconds` setting
- Real-time configuration updates for `military_time` (24h/12h) setting
- HourCycle properly applied based on `military_time` configuration

### Performance
- Zero compilation warnings
- Excellent runtime performance (30MB RAM, <1% CPU idle)
- All functional tests passing


## [0.1.0] - 2026-01-01

### Added
- **Three-tab interface** using `segmented_button::horizontal`
  - 📅 Calendar tab with full month grid
  - 🌤️ Weather tab (placeholder ready for API)
  - ⏱️ Timer tab (placeholder ready for logic)
- **Modular architecture** with separate modules
  - `src/time.rs` - Calendar view and logic
  - `src/weather.rs` - Weather placeholder
  - `src/timer.rs` - Timer placeholder
- **Internationalization** support for 61 languages
  - 8 fully translated (de, es-ES, fr, it, ru, ja, zh-CN, ko)
  - 53 with English fallback
  - Community contribution guidelines in `TRANSLATIONS.md`
- **COSMIC HIG compliance**
  - Standard `padded_control` for dividers
  - Consistent spacing using `cosmic_theme::Spacing`
  - Proper header + content structure
- **Calendar features**
  - ICU formatters with caching for performance
  - Month navigation
  - Today highlighting with accent color
  - Auto-detected locale (12h/24h format)
- **Development tools**
  - Optimized `dev.sh` script with smart Git updates
  - `create_i18n.sh` for language file generation
  - Comprehensive documentation

### Changed
- Migrated from `cosmic-applet-time` base
- Refactored calendar logic from `window.rs` to `time.rs`
- Consolidated helper functions (eliminated duplication)
- Added named constants for better maintainability
  - `CALENDAR_DAYS = 42`
  - `DAY_BUTTON_SIZE = 44.0`
  - `HEADER_PADDING = [12, 20]`

### Fixed
- All compilation warnings resolved
- Dead code removed (`WeatherConfig`, `Message::Surface`, unused imports)
- Popup expansion issue (removed `.center_y()`)
- Builder pattern syntax for cosmic widgets
- Import errors for spacing constants

### Performance
- ~94% reduction in calendar rendering time (ICU formatter caching)
- ~60% faster development cycle (smart Git dependency updates)

## [0.0.1] - 2025-12-29

### Added
- Initial fork from cosmic-applet-time
- Basic project structure
- Build system configuration

---

[Unreleased]: https://github.com/defNickTCys/cosmic-applet-timeplus/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/defNickTCys/cosmic-applet-timeplus/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/defNickTCys/cosmic-applet-timeplus/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/defNickTCys/cosmic-applet-timeplus/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/defNickTCys/cosmic-applet-timeplus/releases/tag/v0.1.0
[0.0.1]: https://github.com/defNickTCys/cosmic-applet-timeplus/releases/tag/v0.0.1

