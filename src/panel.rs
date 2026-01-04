// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

//! Panel UI module
//!
//! This module is responsible for rendering the applet button in the COSMIC panel.
//! It handles both horizontal and vertical panel layouts.
//!
//! The visual logic (sizes, spacing, alignment) was extracted from `time.rs`
//! and is preserved as-is to maintain the approved UI design.

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
    locale::preferences::extensions::unicode::keywords::HourCycle,
};

use crate::config::TimeAppletConfig;
use crate::time::PanelFormatter;

// ============================================================================
// Panel View Functions
// ============================================================================

/// Create vertical panel layout
///
/// This function was extracted from `time.rs::PanelFormatter::vertical_layout()`
/// and preserves the exact visual logic (sizes, spacing, alignment).
pub fn vertical_layout<'a, M: 'a>(
    formatter: &PanelFormatter,
    now: &DateTime<FixedOffset>,
    config: &TimeAppletConfig,
    applet: &applet::Context,
) -> Element<'a, M> {
    let elements: Vec<Element<'a, M>> =
        if let Some(strftime) = formatter.maybe_strftime(now, &config.format_strftime) {
            strftime
                .split_whitespace()
                .map(|piece| applet.text(piece.to_owned()).into())
                .collect()
        } else {
            let mut elements = Vec::new();
            let date = now.naive_local();
            let datetime = crate::calendar::create_datetime(&date, now);
            let mut prefs = DateTimeFormatterPreferences::from(formatter.locale().clone());
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
///
/// This function was extracted from `time.rs::PanelFormatter::horizontal_layout()`
/// and preserves the exact visual logic (sizes, spacing, alignment).
pub fn horizontal_layout<'a, M: 'a>(
    formatter: &PanelFormatter,
    now: &DateTime<FixedOffset>,
    config: &TimeAppletConfig,
    applet: &applet::Context,
) -> Element<'a, M> {
    let formatted_date =
        if let Some(strftime) = formatter.maybe_strftime(now, &config.format_strftime) {
            strftime
        } else {
            let datetime = crate::calendar::create_datetime(now, now);
            let mut prefs = DateTimeFormatterPreferences::from(formatter.locale().clone());
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

/// Main view function for the panel
///
/// This is the primary entry point for rendering the applet button in the panel.
/// It delegates to either `horizontal_layout()` or `vertical_layout()` based on
/// the panel orientation.
pub fn view<'a, M: 'a>(
    formatter: &PanelFormatter,
    now: &DateTime<FixedOffset>,
    config: &TimeAppletConfig,
    applet: &applet::Context,
    horizontal: bool,
) -> Element<'a, M> {
    if horizontal {
        horizontal_layout(formatter, now, config, applet)
    } else {
        vertical_layout(formatter, now, config, applet)
    }
}
