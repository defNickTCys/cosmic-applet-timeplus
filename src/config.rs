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
