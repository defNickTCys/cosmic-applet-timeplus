// SPDX-License-Identifier: GPL-3.0-only

use cosmic_config::CosmicConfigEntry;
use cosmic_config_derive::CosmicConfigEntry as CosmicConfigEntryDerive;
use serde::{Deserialize, Serialize};

/// Configuration for the time applet (from system)
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, CosmicConfigEntryDerive)]
#[version = 1]
pub struct TimeAppletConfig {
    /// Show the date in the top panel.
    #[serde(default)]
    pub show_date_in_top_panel: bool,
    /// Show the weekday in the top panel.
    #[serde(default)]
    pub show_weekday: bool,
    /// Use 24-hour time format.
    #[serde(default)]
    pub military_time: bool,
    /// Show seconds in the time display.
    #[serde(default)]
    pub show_seconds: bool,
    /// First day of the week (0 = Sunday, 1 = Monday, etc.)
    #[serde(default)]
    pub first_day_of_week: u8,
    /// Custom strftime format string (overrides other settings if set).
    #[serde(default)]
    pub format_strftime: String,
}

impl Default for TimeAppletConfig {
    fn default() -> Self {
        Self {
            show_date_in_top_panel: true, // Show date + time on panel
            show_weekday: false,
            military_time: false, // Let ICU auto-detect from locale
            show_seconds: false,
            first_day_of_week: 0,
            format_strftime: String::new(),
        }
    }
}

impl TimeAppletConfig {
    /// Check if custom strftime format includes seconds (%S)
    pub fn has_seconds_in_format(&self) -> bool {
        self.format_strftime.contains("%S")
    }

    /// Check if seconds should be displayed (either via flag or custom format)
    pub fn should_show_seconds(&self) -> bool {
        self.show_seconds || self.has_seconds_in_format()
    }

    /// Validate strftime format string
    pub fn validate_format(&self) -> bool {
        // Empty format is valid (means use ICU defaults)
        if self.format_strftime.is_empty() {
            return true;
        }

        // Basic validation: check for common strftime patterns
        // Allow alphanumeric, spaces, colons, slashes, and % prefixes
        self.format_strftime
            .chars()
            .all(|c| c.is_alphanumeric() || " :/-%.".contains(c))
    }

    /// Get effective time format preference
    pub fn is_24_hour(&self) -> bool {
        self.military_time
    }
}
