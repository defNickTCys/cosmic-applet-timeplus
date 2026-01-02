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

[Unreleased]: https://github.com/defNickTCys/cosmic-applet-timeplus/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/defNickTCys/cosmic-applet-timeplus/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/defNickTCys/cosmic-applet-timeplus/releases/tag/v0.1.0
[0.0.1]: https://github.com/defNickTCys/cosmic-applet-timeplus/releases/tag/v0.0.1

