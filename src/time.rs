// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use chrono::{Datelike, Days, NaiveDate, Timelike, Weekday};
use cosmic::{
    Apply, Element,
    iced::{
        Alignment, Length,
        widget::{column, row},
    },
    widget::{self, Button, Grid, button, container, grid, text},
};
use icu::{
    datetime::{
        DateTimeFormatter, DateTimeFormatterPreferences, fieldsets,
        input::{Date, DateTime, Time},
    },
    locale::Locale,
};

use crate::window::Message;

// Calendar layout constants
const CALENDAR_DAYS: usize = 42; // 6 weeks × 7 days
const DAY_BUTTON_SIZE: f32 = 44.0; // COSMIC HIG standard button size
const HEADER_PADDING: [u16; 2] = [12, 20]; // Vertical, Horizontal padding

/// Gets the first date that will be visible on the calendar
pub fn get_calendar_first(year: i32, month: u32, from_weekday: Weekday) -> NaiveDate {
    let date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let num_days = (date.weekday() as u32 + 7 - from_weekday as u32) % 7; // chrono::Weekday.num_days_from
    date.checked_sub_days(Days::new(num_days as u64)).unwrap()
}

/// Creates an ICU DateTime from a chrono date and time components
pub fn create_datetime<D: Datelike, T: Timelike>(date: &D, now: &T) -> DateTime<icu::calendar::Gregorian> {
    DateTime {
        date: Date::try_new_gregorian(date.year(), date.month() as u8, date.day() as u8)
            .unwrap(),
        time: Time::try_new(
            now.hour() as u8,
            now.minute() as u8,
            now.second() as u8,
            0,
        )
        .unwrap(),
    }
}

/// Creates a styled button for a calendar day
fn date_button(day: u32, is_month: bool, is_day: bool, is_today: bool) -> Button<'static, Message> {
    let style = if is_day {
        button::ButtonClass::Suggested
    } else if is_today {
        button::ButtonClass::Standard
    } else {
        button::ButtonClass::Text
    };

    let button = button::custom(
        text::body(format!("{day}"))
            .apply(container)
            .center(Length::Fill),
    )
    .class(style)
    .height(Length::Fixed(DAY_BUTTON_SIZE))
    .width(Length::Fixed(DAY_BUTTON_SIZE));

    if is_month {
        button.on_press(Message::SelectDay(day))
    } else {
        button
    }
}

/// Builds the calendar grid with weekday headers and day buttons
fn calendar_grid<'a, T: Timelike>(
    locale: &'a Locale,
    date_selected: NaiveDate,
    date_today: NaiveDate,
    now: &T,
    first_day_of_week: u8,
) -> Grid<'a, Message> {
    let mut calendar: Grid<'a, Message> = grid().width(Length::Fill);
    let mut first_day_of_week = chrono::Weekday::try_from(first_day_of_week)
        .unwrap_or(chrono::Weekday::Sun);

    let first_day = get_calendar_first(
        date_selected.year(),
        date_selected.month(),
        first_day_of_week,
    );

    let day_iter = first_day.iter_days();
    let prefs = DateTimeFormatterPreferences::from(locale.clone());
    let weekday = DateTimeFormatter::try_new(prefs, fieldsets::E::short()).unwrap();

    for date in day_iter.take(7) {
        let datetime = create_datetime(&date, now);
        calendar = calendar.push(
            text::caption(weekday.format(&datetime).to_string())
                .apply(container)
                .center_x(Length::Fixed(DAY_BUTTON_SIZE)),
        );
        first_day_of_week = first_day_of_week.succ();
    }
    calendar = calendar.insert_row();

    let mut day_iter = first_day.iter_days();
    for i in 0..CALENDAR_DAYS {
        if i > 0 && i % 7 == 0 {
            calendar = calendar.insert_row();
        }

        let date = day_iter.next().unwrap();
        let is_month = date.month() == date_selected.month()
            && date.year_ce() == date_selected.year_ce();
        let is_day = date.day() == date_selected.day() && is_month;
        let is_today = date == date_today;

        calendar = calendar.push(date_button(date.day(), is_month, is_day, is_today));
    }

    calendar
}

/// Renders the complete calendar view with header, navigation, and day grid
pub fn view_calendar<'a, T: Timelike>(
    locale: &'a Locale,
    date_selected: NaiveDate,
    date_today: NaiveDate,
    now: &T,
    first_day_of_week: u8,
) -> Element<'a, Message> {
    let datetime = create_datetime(&date_selected, now);
    let prefs = DateTimeFormatterPreferences::from(locale.clone());

    let date = text(
        DateTimeFormatter::try_new(prefs, fieldsets::YMD::long())
            .unwrap()
            .format(&datetime)
            .to_string(),
    )
    .size(18);
    
    let day_of_week = text::body(
        DateTimeFormatter::try_new(prefs, fieldsets::E::long())
            .unwrap()
            .format(&datetime)
            .to_string(),
    );

    let month_controls = row![
        button::icon(widget::icon::from_name("go-previous-symbolic"))
            .padding(8)
            .on_press(Message::PreviousMonth),
        button::icon(widget::icon::from_name("go-next-symbolic"))
            .padding(8)
            .on_press(Message::NextMonth)
    ]
    .spacing(8);

    let calendar = calendar_grid(locale, date_selected, date_today, now, first_day_of_week);

    column![
        row![
            column![date, day_of_week],
            widget::Space::with_width(Length::Fill),
            month_controls,
        ]
        .align_y(Alignment::Center)
        .padding(HEADER_PADDING),
        calendar.padding([0, 12].into()),
    ]
    .into()
}

