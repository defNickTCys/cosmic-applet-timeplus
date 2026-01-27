// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use chrono::{Datelike, Days, NaiveDate, Timelike, Weekday};
use cosmic::{
    iced::{
        widget::{column, row},
        Alignment, Length,
    },
    widget::{self, button, container, grid, text, Button, Grid},
    Apply, Element,
};
use icu::{
    datetime::{
        fieldsets,
        input::{Date, DateTime, Time},
        DateTimeFormatter, DateTimeFormatterPreferences,
    },
    locale::Locale,
};

use crate::icons;

// Calendar layout constants
const CALENDAR_DAYS: usize = 42; // 6 weeks × 7 days
const DAY_BUTTON_SIZE: f32 = 44.0; // COSMIC HIG standard button size
const HEADER_PADDING: [u16; 2] = [12, 20]; // Vertical, Horizontal padding

// ============================================================================
// Calendar State Management
// ============================================================================

/// State management for the calendar functionality
#[derive(Debug, Clone)]
pub struct CalendarState {
    pub date_today: NaiveDate,
    pub date_selected: NaiveDate,
}

impl CalendarState {
    /// Create a new calendar state from the current time
    pub fn new(now: chrono::DateTime<chrono::FixedOffset>) -> Self {
        let today = NaiveDate::from(now.naive_local());
        Self {
            date_today: today,
            date_selected: today,
        }
    }

    /// Reset calendar to today's date
    pub fn reset_to_today(&mut self, now: chrono::DateTime<chrono::FixedOffset>) {
        self.date_today = NaiveDate::from(now.naive_local());
        self.date_selected = self.date_today;
    }

    /// Update calendar state based on message
    pub fn update(&mut self, message: CalendarMessage) {
        match message {
            CalendarMessage::SelectDay(day) => {
                if let Some(date) = self.date_selected.with_day(day) {
                    self.date_selected = date;
                } else {
                    tracing::error!("invalid naivedate");
                }
            }
            CalendarMessage::PreviousMonth => {
                if let Some(date) = self
                    .date_selected
                    .checked_sub_months(chrono::Months::new(1))
                {
                    self.date_selected = date;
                } else {
                    tracing::error!("invalid naivedate");
                }
            }
            CalendarMessage::NextMonth => {
                if let Some(date) = self
                    .date_selected
                    .checked_add_months(chrono::Months::new(1))
                {
                    self.date_selected = date;
                } else {
                    tracing::error!("invalid naivedate");
                }
            }
        }
    }
}

// ============================================================================
// Calendar Messages
// ============================================================================

/// Messages for calendar interactions
#[derive(Debug, Clone)]
pub enum CalendarMessage {
    SelectDay(u32),
    PreviousMonth,
    NextMonth,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Gets the first date that will be visible on the calendar
fn get_calendar_first(year: i32, month: u32, from_weekday: Weekday) -> NaiveDate {
    let date = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let num_days = (date.weekday() as u32 + 7 - from_weekday as u32) % 7;
    date.checked_sub_days(Days::new(num_days as u64)).unwrap()
}

/// Creates an ICU DateTime from a chrono date and time components
pub fn create_datetime<D: Datelike, T: Timelike>(
    date: &D,
    now: &T,
) -> DateTime<icu::calendar::Gregorian> {
    DateTime {
        date: Date::try_new_gregorian(date.year(), date.month() as u8, date.day() as u8).unwrap(),
        time: Time::try_new(now.hour() as u8, now.minute() as u8, now.second() as u8, 0).unwrap(),
    }
}

/// Creates a styled button for a calendar day
fn date_button(
    day: u32,
    is_month: bool,
    is_day: bool,
    is_today: bool,
) -> Button<'static, CalendarMessage> {
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
        button.on_press(CalendarMessage::SelectDay(day))
    } else {
        button
    }
}

/// Builds the calendar grid with weekday headers and day buttons
fn calendar_grid<'a, T: Timelike>(
    locale: &'a Locale,
    calendar_state: &CalendarState,
    now: &T,
    first_day_of_week: u8,
) -> Grid<'a, CalendarMessage> {
    let mut calendar: Grid<'a, CalendarMessage> = grid().width(Length::Fill);
    let mut first_day_of_week =
        chrono::Weekday::try_from(first_day_of_week).unwrap_or(chrono::Weekday::Sun);

    let first_day = get_calendar_first(
        calendar_state.date_selected.year(),
        calendar_state.date_selected.month(),
        first_day_of_week,
    );

    let day_iter = first_day.iter_days();

    // Create formatter once for this render (following cosmic-applet-time pattern)
    let prefs = DateTimeFormatterPreferences::from(locale.clone());
    let weekday_formatter = DateTimeFormatter::try_new(prefs, fieldsets::E::short()).unwrap();

    for date in day_iter.take(7) {
        let datetime = create_datetime(&date, now);
        calendar = calendar.push(
            text::caption(weekday_formatter.format(&datetime).to_string())
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
        let is_month = date.month() == calendar_state.date_selected.month()
            && date.year_ce() == calendar_state.date_selected.year_ce();
        let is_day = date.day() == calendar_state.date_selected.day() && is_month;
        let is_today = date == calendar_state.date_today;

        calendar = calendar.push(date_button(date.day(), is_month, is_day, is_today));
    }

    calendar
}

// ============================================================================
// Public View Function
// ============================================================================

/// Renders the complete calendar view with header, navigation, and day grid
/// Follows cosmic-applet-time pattern: formatters created once per render
pub fn view_calendar<'a, T: Timelike>(
    locale: &'a Locale,
    calendar_state: &CalendarState,
    now: &T,
    first_day_of_week: u8,
) -> Element<'a, CalendarMessage> {
    let datetime = create_datetime(&calendar_state.date_selected, now);

    // Create formatters once for this render (following cosmic-applet-time pattern)
    let prefs = DateTimeFormatterPreferences::from(locale.clone());
    let date_formatter = DateTimeFormatter::try_new(prefs, fieldsets::YMD::long()).unwrap();
    let weekday_formatter = DateTimeFormatter::try_new(prefs, fieldsets::E::long()).unwrap();

    let date = text(date_formatter.format(&datetime).to_string()).size(18);

    let day_of_week = text::body(weekday_formatter.format(&datetime).to_string());

    let month_controls = row![
        button::icon(widget::icon::from_name(icons::navigation::PREVIOUS))
            .padding(8)
            .on_press(CalendarMessage::PreviousMonth),
        button::icon(widget::icon::from_name(icons::navigation::NEXT))
            .padding(8)
            .on_press(CalendarMessage::NextMonth)
    ]
    .spacing(8);

    let calendar = calendar_grid(locale, calendar_state, now, first_day_of_week);

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
