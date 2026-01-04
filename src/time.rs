// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

//! Panel time formatting module
//!
//! This module provides time formatting utilities for the panel display.
//! It focuses on data formatting (strftime and ICU) without UI construction.
//! UI layout logic has been moved to the `panel` module.

use chrono::{DateTime, FixedOffset};
use icu::locale::Locale;

// ============================================================================
// Constants
// ============================================================================

/// Specifiers for strftime that indicate seconds
///
/// Subsecond precision isn't supported by the applet so those specifiers
/// aren't listed here. This list is non-exhaustive.
pub const STRFTIME_SECONDS: &[char] = &['S', 'T', '+', 's'];

// ============================================================================
// Panel Formatter
// ============================================================================

/// Panel time formatter
///
/// Handles all time formatting for the panel display, supporting both
/// strftime custom formats and ICU locale-aware formatting.
pub struct PanelFormatter {
    locale: Locale,
}

impl PanelFormatter {
    /// Create a new panel formatter with the given locale
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }

    /// Get the locale
    pub fn locale(&self) -> &Locale {
        &self.locale
    }

    /// Format with strftime if non-empty and ignore errors
    ///
    /// Do not use to_string(). The formatter panics on invalid specifiers.
    pub fn maybe_strftime(&self, now: &DateTime<FixedOffset>, format: &str) -> Option<String> {
        // strftime may override locale specific elements so it stands alone
        // rather than using ICU.
        (!format.is_empty())
            .then(|| {
                let mut s = String::new();
                now.format(format).write_to(&mut s).map(|_| s).ok()
            })
            .flatten()
    }

    /// Check if strftime format contains seconds
    #[allow(dead_code)]
    pub fn format_has_seconds(format: &str) -> bool {
        format
            .split('%')
            .any(|s| STRFTIME_SECONDS.contains(&s.chars().next().unwrap_or_default()))
    }
}
