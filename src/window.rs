// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::widget::segmented_button;
use cosmic::widget::Id;
use cosmic::{
    app,
    applet::{cosmic_panel_config::PanelAnchor, menu_button, padded_control},
    cctk::sctk::reexports::calloop,
    cosmic_theme::Spacing,
    iced::{
        platform_specific::shell::wayland::commands::popup::{destroy_popup, get_popup},
        widget::column,
        window, Rectangle, Subscription,
    },
    theme,
    widget::{autosize, button, container, divider, icon, rectangle_tracker::*, text},
    Element, Task,
};
use std::sync::LazyLock;
use tokio::sync::watch;

use crate::{config::TimeAppletConfig, fl};
use cosmic::applet::token::subscription::{
    activation_token_subscription, TokenRequest, TokenUpdate,
};
use icu::locale::Locale;

// Import global types from lib.rs (Neutral Messenger)
use crate::{Message, Tab};

// Import localization function
use crate::localize::get_system_locale;

static AUTOSIZE_MAIN_ID: LazyLock<Id> = LazyLock::new(|| Id::new("autosize-main"));

pub struct Window {
    core: cosmic::app::Core,
    popup: Option<window::Id>,
    now: chrono::DateTime<chrono::FixedOffset>,
    timezone: Option<chrono_tz::Tz>,
    calendar_state: crate::calendar::CalendarState,
    panel_formatter: crate::time::PanelFormatter, // Panel time formatter
    rectangle_tracker: Option<RectangleTracker<u32>>,
    rectangle: Rectangle,
    token_tx: Option<calloop::channel::Sender<TokenRequest>>,
    config: TimeAppletConfig,
    show_seconds_tx: watch::Sender<bool>,
    locale: Locale,
    // Tab system
    selected_tab: Tab,
    tab_model: segmented_button::SingleSelectModel,
}

impl cosmic::Application for Window {
    type Message = Message;
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    const APP_ID: &'static str = "com.system76.CosmicAppletTime";

    fn init(core: app::Core, _flags: Self::Flags) -> (Self, app::Task<Self::Message>) {
        let locale = get_system_locale();

        // Chrono evaluates the local timezone once whereby it's stored in a thread local
        // variable but never updated
        // Instead of using the local timezone, we will store an offset that is updated if the
        // timezone is ever externally changed
        let now = chrono::Local::now().fixed_offset();

        // Synch `show_seconds` from the config within the time subscription
        let (show_seconds_tx, _) = watch::channel(true);

        // Initialize tab model (using segmented_button for tab navigation)
        let mut tab_model = segmented_button::Model::builder()
            .insert(|b| {
                b.text(fl!("calendar"))
                    .icon(icon::from_name("com.system76.CosmicAppletTime-symbolic"))
                    .data(Tab::Calendar)
            })
            .insert(|b| {
                b.text(fl!("weather"))
                    .icon(icon::from_name("weather-clear-symbolic"))
                    .data(Tab::Weather)
            })
            .insert(|b| {
                b.text(fl!("timer"))
                    .icon(icon::from_name("alarm-symbolic"))
                    .data(Tab::Timer)
            })
            .build();

        // Activate first tab
        tab_model.activate_position(0);

        (
            Self {
                core,
                popup: None,
                now,
                timezone: None,
                calendar_state: crate::calendar::CalendarState::new(now),
                panel_formatter: crate::time::PanelFormatter::new(locale.clone()),
                rectangle_tracker: None,
                rectangle: Rectangle::default(),
                token_tx: None,
                config: TimeAppletConfig::default(),
                show_seconds_tx,
                locale,
                selected_tab: Tab::Calendar,
                tab_model,
            },
            Task::none(),
        )
    }

    fn core(&self) -> &cosmic::app::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::app::Core {
        &mut self.core
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }

    fn subscription(&self) -> Subscription<Message> {
        let show_seconds_rx = self.show_seconds_tx.subscribe();
        Subscription::batch([
            rectangle_tracker_subscription(0).map(|e| Message::Rectangle(e.1)),
            crate::subscriptions::time_subscription(show_seconds_rx),
            activation_token_subscription(0).map(Message::Token),
            crate::subscriptions::timezone_subscription(),
            crate::subscriptions::wake_from_sleep_subscription(),
            self.core.watch_config(Self::APP_ID).map(|u| {
                for err in u.errors {
                    tracing::error!(?err, "Error watching config");
                }
                Message::ConfigChanged(u.config)
            }),
        ])
    }

    fn update(&mut self, message: Self::Message) -> app::Task<Self::Message> {
        match message {
            Message::TogglePopup => {
                if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    self.calendar_state.reset_to_today(self.now);

                    let new_id = window::Id::unique();
                    self.popup = Some(new_id);

                    let mut popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );
                    let Rectangle {
                        x,
                        y,
                        width,
                        height,
                    } = self.rectangle;
                    popup_settings.positioner.anchor_rect = Rectangle::<i32> {
                        x: x.max(1.) as i32,
                        y: y.max(1.) as i32,
                        width: width.max(1.) as i32,
                        height: height.max(1.) as i32,
                    };

                    popup_settings.positioner.size = None;

                    get_popup(popup_settings)
                }
            }
            Message::Tick => {
                self.now = self.timezone.map_or_else(
                    || chrono::Local::now().into(),
                    |tz| chrono::Local::now().with_timezone(&tz).fixed_offset(),
                );
                Task::none()
            }
            Message::Rectangle(u) => {
                match u {
                    RectangleUpdate::Rectangle(r) => {
                        self.rectangle = r.1;
                    }
                    RectangleUpdate::Init(tracker) => {
                        self.rectangle_tracker = Some(tracker);
                    }
                }
                Task::none()
            }
            Message::CloseRequested(id) => {
                if Some(id) == self.popup {
                    self.popup = None;
                }
                Task::none()
            }
            Message::Calendar(msg) => {
                self.calendar_state.update(msg);
                Task::none()
            }
            Message::OpenDateTimeSettings => {
                let exec = "cosmic-settings time".to_string();
                if let Some(tx) = self.token_tx.as_ref() {
                    let _ = tx.send(TokenRequest {
                        app_id: Self::APP_ID.to_string(),
                        exec,
                    });
                } else {
                    tracing::error!("Wayland tx is None");
                }
                Task::none()
            }
            Message::Token(u) => {
                match u {
                    TokenUpdate::Init(tx) => {
                        self.token_tx = Some(tx);
                    }
                    TokenUpdate::Finished => {
                        self.token_tx = None;
                    }
                    TokenUpdate::ActivationToken { token, .. } => {
                        let mut cmd = std::process::Command::new("cosmic-settings");
                        cmd.arg("time");
                        if let Some(token) = token {
                            cmd.env("XDG_ACTIVATION_TOKEN", &token);
                            cmd.env("DESKTOP_STARTUP_ID", &token);
                        }
                        tokio::spawn(cosmic::process::spawn(cmd));
                    }
                }
                Task::none()
            }
            Message::ConfigChanged(c) => {
                // Don't interrupt the tick subscription unless necessary
                self.show_seconds_tx.send_if_modified(|show_seconds| {
                    if !c.format_strftime.is_empty() {
                        if c.format_strftime.split('%').any(|s| {
                            crate::time::STRFTIME_SECONDS
                                .contains(&s.chars().next().unwrap_or_default())
                        }) && !*show_seconds
                        {
                            // The strftime formatter contains a seconds specifier. Force enable
                            // ticking per seconds internally regardless of the user setting.
                            // This does not change the user's setting. It's invisible to the user.
                            *show_seconds = true;
                            true
                        } else {
                            false
                        }
                    } else if *show_seconds == c.show_seconds {
                        false
                    } else {
                        *show_seconds = c.show_seconds;
                        true
                    }
                });
                self.config = c;
                Task::none()
            }
            Message::TimezoneUpdate(timezone) => {
                if let Ok(timezone) = timezone.parse::<chrono_tz::Tz>() {
                    self.now = chrono::Local::now().with_timezone(&timezone).fixed_offset();
                    self.calendar_state.reset_to_today(self.now);
                    self.timezone = Some(timezone);
                }

                self.update(Message::Tick)
            }
            Message::TabActivated(entity) => {
                self.tab_model.activate(entity);
                if let Some(tab) = self.tab_model.data::<Tab>(entity) {
                    self.selected_tab = *tab;
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let horizontal = matches!(
            self.core.applet.anchor,
            PanelAnchor::Top | PanelAnchor::Bottom
        );

        let panel_view = crate::panel::view(
            &self.panel_formatter,
            &self.now,
            &self.config,
            &self.core.applet,
            horizontal,
        );

        let button = button::custom(panel_view)
            .padding(if horizontal {
                [0, self.core.applet.suggested_padding(true).0]
            } else {
                [self.core.applet.suggested_padding(true).0, 0]
            })
            .on_press_down(Message::TogglePopup)
            .class(cosmic::theme::Button::AppletIcon);

        autosize::autosize(
            if let Some(tracker) = self.rectangle_tracker.as_ref() {
                Element::from(tracker.container(0, button).ignore_bounds(true))
            } else {
                button.into()
            },
            AUTOSIZE_MAIN_ID.clone(),
        )
        .into()
    }

    fn view_window(&self, _id: window::Id) -> Element<'_, Message> {
        let Spacing {
            space_xxs, space_s, ..
        } = theme::active().cosmic().spacing;

        // Tab navigation - matches system separator width with padding
        let tabs = container(
            segmented_button::horizontal(&self.tab_model)
                .button_spacing(space_xxs) // Spacing between icon and text
                .button_padding([space_xxs, space_s, space_xxs, space_s]) // Symmetric padding (top, right, bottom, left)
                .on_activate(Message::TabActivated),
        )
        .padding([0, space_s]); // Horizontal padding to match separator

        // Select view based on active tab
        let tab_content = match self.selected_tab {
            Tab::Calendar => crate::calendar::view_calendar(
                &self.locale,
                &self.calendar_state,
                &self.now,
                self.config.first_day_of_week,
            )
            .map(Message::Calendar),
            Tab::Weather => crate::weather::view_weather(),
            Tab::Timer => crate::timer::view_timer(),
        };

        // Footer with settings button
        let footer = column![
            padded_control(divider::horizontal::default()).padding([space_xxs, space_s]),
            menu_button(text::body(fl!("datetime-settings")))
                .on_press(Message::OpenDateTimeSettings),
        ];

        let content_list = column![tabs, tab_content, footer,].padding([8, 0]);

        self.core
            .applet
            .popup_container(container(content_list))
            .into()
    }

    fn on_close_requested(&self, id: window::Id) -> Option<Message> {
        Some(Message::CloseRequested(id))
    }
}
