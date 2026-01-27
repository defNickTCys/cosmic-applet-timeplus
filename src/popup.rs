// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

//! Popup UI module
//!
//! This module is responsible for rendering the applet popup window.
//! It handles the popup structure: tab navigation, tab content, and footer.
//!
//! The visual logic (spacing, layout, structure) was extracted from `window.rs`
//! and is preserved as-is to maintain the approved UI design.

use chrono::{DateTime, FixedOffset};
use cosmic::{
    Element,
    applet::{Context as AppletContext, menu_button, padded_control},
    cosmic_theme::Spacing,
    iced::widget::column,
    theme,
    widget::{divider, segmented_button, text},
};
use icu::locale::Locale;

use crate::{Message, Tab, calendar::CalendarState, config::TimeAppletConfig, fl};

// ============================================================================
// Popup View Function
// ============================================================================

/// Main view function for the popup window
///
/// This is the primary entry point for rendering the applet popup.
/// It constructs the complete popup structure:
/// - Tab navigation at top
/// - Tab content in middle (delegated to feature modules)
/// - Footer with settings button at bottom
///
/// This function was extracted from `window.rs::view_window()` and preserves
/// the exact visual logic (spacing, layout, structure).
pub fn view<'a>(
    locale: &'a Locale,
    calendar_state: &'a CalendarState,
    now: &'a DateTime<FixedOffset>,
    config: &'a TimeAppletConfig,
    selected_tab: Tab,
    tab_model: &'a segmented_button::SingleSelectModel,
    applet: &'a AppletContext,
) -> Element<'a, Message> {
    let Spacing {
        space_xxs, space_s, ..
    } = theme::active().cosmic().spacing;

    // Tab navigation - matches system separator width with padding
    let tabs = cosmic::widget::container(
        segmented_button::horizontal(tab_model)
            .button_spacing(space_xxs) // Spacing between icon and text
            .button_padding([space_xxs, space_s, space_xxs, space_s]) // Symmetric padding (top, right, bottom, left)
            .on_activate(Message::TabActivated),
    )
    .padding([0, space_s]); // Horizontal padding to match separator

    // Select view based on active tab
    let tab_content = match selected_tab {
        Tab::Calendar => {
            crate::calendar::view_calendar(locale, calendar_state, now, config.first_day_of_week)
                .map(Message::Calendar)
        }
        Tab::Weather => crate::weather::view_weather(),
        Tab::Timer => crate::timer::view_timer(),
    };

    // Footer with settings button
    let footer = column![
        padded_control(divider::horizontal::default()).padding([space_xxs, space_s]),
        menu_button(text::body(fl!("datetime-settings"))).on_press(Message::OpenDateTimeSettings),
    ];

    let content_list = column![tabs, tab_content, footer,].padding([8, 0]);

    applet
        .popup_container(cosmic::widget::container(content_list))
        .into()
}
