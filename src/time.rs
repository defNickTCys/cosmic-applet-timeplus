// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

//! Panel time formatting module
//!
//! This module handles all time formatting logic for the panel display,
//! including both vertical and horizontal layouts, strftime formatting,
//! and ICU-based locale-aware time formatting.

use chrono::{DateTime, FixedOffset};
use cosmic::{
    applet,
    iced::{
        widget::{column, row},
        Alignment, Length,
    },
    iced_widget::{horizontal_rule, Column},
    widget::{container, horizontal_space, vertical_space},
    Element,
};
use icu::{
    datetime::{
        fieldsets, options::TimePrecision, DateTimeFormatter, DateTimeFormatterPreferences,
    },
    locale::{preferences::extensions::unicode::keywords::HourCycle, Locale},
};

use crate::config::TimeAppletConfig;

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

    /// Create vertical panel layout
    pub fn vertical_layout<'a, M: 'a>(
        &self,
        now: &DateTime<FixedOffset>,
        config: &TimeAppletConfig,
        applet: &applet::Context,
    ) -> Element<'a, M> {
        let elements: Vec<Element<'a, M>> =
            if let Some(strftime) = self.maybe_strftime(now, &config.format_strftime) {
                strftime
                    .split_whitespace()
                    .map(|piece| applet.text(piece.to_owned()).into())
                    .collect()
            } else {
                let mut elements = Vec::new();
                let date = now.naive_local();
                let datetime = crate::calendar::create_datetime(&date, now);
                let mut prefs = DateTimeFormatterPreferences::from(self.locale.clone());
                prefs.hour_cycle = Some(if config.military_time {
                    HourCycle::H23
                } else {
                    HourCycle::H12
                });

                if config.show_date_in_top_panel {
                    let formatted_date = DateTimeFormatter::try_new(prefs, fieldsets::MD::medium())
                        .unwrap()
                        .format(&datetime)
                        .to_string();

                    for p in formatted_date.split_whitespace() {
                        elements.push(applet.text(p.to_owned()).into());
                    }
                    elements.push(
                        horizontal_rule(2)
                            .width(applet.suggested_size(true).0)
                            .into(),
                    );
                }
                let mut fs = fieldsets::T::medium();
                if !config.show_seconds {
                    fs = fs.with_time_precision(TimePrecision::Minute);
                }
                let formatted_time = DateTimeFormatter::try_new(prefs, fs)
                    .unwrap()
                    .format(&datetime)
                    .to_string();

                // todo: split using formatToParts when it is implemented
                // https://github.com/unicode-org/icu4x/issues/4936#issuecomment-2128812667
                for p in formatted_time.split_whitespace().flat_map(|s| s.split(':')) {
                    elements.push(applet.text(p.to_owned()).into());
                }

                elements
            };

        let date_time_col = Column::with_children(elements)
            .align_x(Alignment::Center)
            .spacing(4);

        Element::from(
            column!(
                date_time_col,
                horizontal_space().width(Length::Fixed(
                    (applet.suggested_size(true).0 + 2 * applet.suggested_padding(true).1) as f32
                ))
            )
            .align_x(Alignment::Center),
        )
    }

    /// Create horizontal panel layout
    pub fn horizontal_layout<'a, M: 'a>(
        &self,
        now: &DateTime<FixedOffset>,
        config: &TimeAppletConfig,
        applet: &applet::Context,
    ) -> Element<'a, M> {
        let formatted_date =
            if let Some(strftime) = self.maybe_strftime(now, &config.format_strftime) {
                strftime
            } else {
                let datetime = crate::calendar::create_datetime(now, now);
                let mut prefs = DateTimeFormatterPreferences::from(self.locale.clone());
                prefs.hour_cycle = Some(if config.military_time {
                    HourCycle::H23
                } else {
                    HourCycle::H12
                });

                if config.show_date_in_top_panel {
                    if config.show_weekday {
                        let mut fs = fieldsets::MDET::medium();
                        if !config.show_seconds {
                            fs = fs.with_time_precision(TimePrecision::Minute);
                        }
                        DateTimeFormatter::try_new(prefs, fs)
                            .unwrap()
                            .format(&datetime)
                            .to_string()
                    } else {
                        let mut fs = fieldsets::MDT::medium(); // Medium format: "2 de jan., 02:31"
                        if !config.show_seconds {
                            fs = fs.with_time_precision(TimePrecision::Minute);
                        }
                        DateTimeFormatter::try_new(prefs, fs)
                            .unwrap()
                            .format(&datetime)
                            .to_string()
                    }
                } else {
                    let mut fs = fieldsets::T::medium();
                    if !config.show_seconds {
                        fs = fs.with_time_precision(TimePrecision::Minute);
                    }
                    DateTimeFormatter::try_new(prefs, fs)
                        .unwrap()
                        .format(&datetime)
                        .to_string()
                }
            };

        Element::from(
            row!(
                applet.text(formatted_date),
                container(vertical_space().height(Length::Fixed(
                    (applet.suggested_size(true).1 + 2 * applet.suggested_padding(true).1) as f32
                )))
            )
            .align_y(Alignment::Center),
        )
    }
}
