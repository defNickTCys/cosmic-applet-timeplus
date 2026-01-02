// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    applet::padded_control,
    cosmic_theme::Spacing,
    iced::{Alignment, Length},
    widget::{column, container, divider, icon, row, text, Space},
    Element,
};

use crate::fl;
use crate::Message;

/// Weather tab placeholder view
pub fn view_weather<'a>() -> Element<'a, Message> {
    let Spacing {
        space_xxs, space_s, ..
    } = cosmic::theme::active().cosmic().spacing;

    let weather_icon: cosmic::widget::Icon = icon::from_name("weather-clear-symbolic")
        .size(24) // Smaller to match calendar header height
        .into();

    let features_icon: cosmic::widget::Icon = icon::from_name("starred-symbolic").size(16).into();

    // Match calendar structure: header + content (2 elements)
    column()
        // Header (like calendar: icon + 2 text lines)
        .push(
            row()
                .push(weather_icon)
                .push(Space::with_width(Length::Fixed(12.0)))
                .push(
                    column()
                        .push(text(fl!("weather")).size(18)) // Match calendar date size
                        .push(text::body(fl!("weather-subtitle"))), // Match calendar day_of_week
                )
                .align_y(Alignment::Center)
                .padding([12, 20]), // Match calendar HEADER_PADDING
        )
        // Standard separator
        .push(padded_control(divider::horizontal::default()).padding([space_xxs, space_s]))
        // Content (like calendar grid)
        .push(
            container(
                column()
                    .push(text::body(fl!("weather-placeholder-message")))
                    .push(Space::with_height(Length::Fixed(12.0)))
                    .push(
                        row()
                            .push(features_icon)
                            .push(Space::with_width(Length::Fixed(8.0)))
                            .push(text::heading(fl!("weather-placeholder-features")))
                            .align_y(Alignment::Center),
                    )
                    .push(Space::with_height(Length::Fixed(8.0)))
                    .push(text::body(
                        "• ".to_string() + &fl!("weather-feature-current"),
                    ))
                    .push(text::body(
                        "• ".to_string() + &fl!("weather-feature-temperature"),
                    ))
                    .push(text::body(
                        "• ".to_string() + &fl!("weather-feature-forecast"),
                    ))
                    .spacing(4),
            )
            .padding([0, 20]), // Match header horizontal padding for alignment
        )
        .into()
}
