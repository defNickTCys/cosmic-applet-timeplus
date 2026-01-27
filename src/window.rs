// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use chrono::Datelike;
use cosmic::widget::Id;
use cosmic::widget::segmented_button;
use cosmic::{
    Element, Task, app,
    applet::cosmic_panel_config::PanelAnchor,
    cctk::sctk::reexports::calloop,
    iced::{
        Rectangle, Subscription,
        platform_specific::shell::wayland::commands::popup::{destroy_popup, get_popup},
        window,
    },
    widget::{autosize, button, icon, rectangle_tracker::*},
};
use std::sync::LazyLock;
use tokio::sync::watch;

use crate::config::TimeAppletConfig;
use cosmic::applet::token::subscription::{
    TokenRequest, TokenUpdate, activation_token_subscription,
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
    /// Panel position captured at initialization (immutable during process lifecycle)
    panel_anchor: PanelAnchor,
}

impl cosmic::Application for Window {
    type Message = Message;
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = TimeAppletConfig;
    const APP_ID: &'static str = "com.system76.CosmicAppletTime";

    fn init(core: app::Core, config: Self::Flags) -> (Self, app::Task<Self::Message>) {
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
                b.text(Tab::Calendar.label())
                    .icon(icon::from_name(Tab::Calendar.icon_name()))
                    .data(Tab::Calendar)
            })
            .insert(|b| {
                b.text(Tab::Weather.label())
                    .icon(icon::from_name(Tab::Weather.icon_name()))
                    .data(Tab::Weather)
            })
            .insert(|b| {
                b.text(Tab::Timer.label())
                    .icon(icon::from_name(Tab::Timer.icon_name()))
                    .data(Tab::Timer)
            })
            .build();

        // Activate first tab
        tab_model.activate_position(0);

        // Capture panel position at initialization (immutable for process lifecycle)
        let panel_anchor = core.applet.anchor;

        tracing::info!("[Init] Initializing applet");
        tracing::info!("[Init] Panel position: {:?}", panel_anchor);
        tracing::info!("[Init] Locale: {}", locale);
        tracing::info!("[Init] Timezone: {}", now.timezone());
        tracing::debug!(
            "[Init] Config: show_seconds={}, format={}",
            config.show_seconds,
            config.format_strftime
        );

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
                config,
                show_seconds_tx,
                locale,
                selected_tab: Tab::Calendar,
                tab_model,
                panel_anchor,
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
                    tracing::info!("[UI] Closing popup");
                    destroy_popup(p)
                } else {
                    tracing::info!("[UI] Opening popup");
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
                    tracing::info!("[UI] Closing popup (close requested)");
                    self.popup = None;
                }
                Task::none()
            }
            Message::Calendar(msg) => {
                // Log with full date context
                match &msg {
                    crate::calendar::CalendarMessage::SelectDay(day) => {
                        let selected = self.calendar_state.date_selected;
                        tracing::debug!(
                            "[Calendar] SelectDay({}) -> {}-{:02}-{:02}",
                            day,
                            selected.year(),
                            selected.month(),
                            day
                        );
                    }
                    crate::calendar::CalendarMessage::PreviousMonth => {
                        tracing::debug!("[Calendar] PreviousMonth");
                    }
                    crate::calendar::CalendarMessage::NextMonth => {
                        tracing::debug!("[Calendar] NextMonth");
                    }
                }
                self.calendar_state.update(msg);
                Task::none()
            }
            Message::OpenDateTimeSettings => {
                let exec = "cosmic-settings time".to_string();
                if let Some(tx) = self.token_tx.as_ref() {
                    tracing::info!("[System] Opening settings: {}", exec);
                    let _ = tx.send(TokenRequest {
                        app_id: Self::APP_ID.to_string(),
                        exec,
                    });
                } else {
                    tracing::warn!(
                        "[System] Settings requested but Wayland tx unavailable (not running in panel?)"
                    );
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
                    let new_value = c.should_show_seconds();
                    if *show_seconds != new_value {
                        *show_seconds = new_value;
                        true
                    } else {
                        false
                    }
                });
                self.config = c;
                Task::none()
            }
            Message::TimezoneUpdate(timezone) => {
                let tz = crate::time::parse_timezone(&timezone);
                self.now = chrono::Local::now().with_timezone(&tz).fixed_offset();
                self.calendar_state.reset_to_today(self.now);
                self.timezone = Some(tz);

                self.update(Message::Tick)
            }
            // Notification placeholders (Phase 3.9 - not yet implemented)
            Message::TriggerNotification { .. } => {
                // TODO: Implement in Phase 3.9
                Task::none()
            }
            Message::NotificationDismissed => {
                // TODO: Implement in Phase 3.9
                Task::none()
            }
            Message::NotificationAction(_) => {
                // TODO: Implement in Phase 3.9
                Task::none()
            }
            Message::TabActivated(entity) => {
                self.tab_model.activate(entity);
                if let Some(tab) = self.tab_model.data::<Tab>(entity) {
                    tracing::info!("[Navigation] Switched to tab: {:?}", tab);
                    self.selected_tab = *tab;
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // Use pre-captured panel position (immutable during process lifecycle)
        let horizontal = matches!(self.panel_anchor, PanelAnchor::Top | PanelAnchor::Bottom);

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
        crate::popup::view(
            &self.locale,
            &self.calendar_state,
            &self.now,
            &self.config,
            self.selected_tab,
            &self.tab_model,
            &self.core.applet,
        )
    }

    fn on_close_requested(&self, id: window::Id) -> Option<Message> {
        Some(Message::CloseRequested(id))
    }
}
