// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

mod calendar; // Calendar module (renamed from time)
pub mod config;
pub mod icons; // Centralized icon names (SSoT)
mod localize;
mod panel; // Panel UI module
pub mod paths; // Portable asset location (XDG-compliant)
mod popup; // Popup UI module
mod subscriptions; // Subscription management (time, timezone, wake-from-sleep)
mod time; // Panel time formatting module (new)
mod timer;
mod weather;
mod window;

use window::Window;

// ============================================================================
// Global Message Envelope (Neutral Messenger)
// ============================================================================
// This enum serves as the central message dispatcher for the entire applet.
// Each module has its own message type that gets wrapped in this envelope,
// preventing circular dependencies and allowing modules to be self-contained.

use crate::config::TimeAppletConfig;
use cosmic::{applet::token::subscription::TokenUpdate, widget::segmented_button};

/// Global message envelope - accessible by all modules
#[derive(Debug, Clone)]
pub enum Message {
    // Window orchestration
    TogglePopup,
    CloseRequested(cosmic::iced::window::Id),
    Tick,
    Rectangle(cosmic::widget::rectangle_tracker::RectangleUpdate<u32>),
    TabActivated(segmented_button::Entity),

    // Module envelopes
    Calendar(calendar::CalendarMessage),

    // Notifications (Placeholders for Phase 3.7)
    /// Trigger a notification alert with sound
    TriggerNotification {
        message: String,
        duration_secs: u64,
    },
    /// Notification was dismissed (body click or X button)
    NotificationDismissed,
    /// User clicked notification action button
    NotificationAction(String),

    // System
    OpenDateTimeSettings,
    Token(TokenUpdate),
    ConfigChanged(TimeAppletConfig),
    TimezoneUpdate(String),
}

// ============================================================================
// Tab Selection
// ============================================================================
// Shared tab enum used across all modules for consistent tab navigation

/// Tab selection for the applet popup
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Calendar,
    Weather,
    Timer,
}

impl Tab {
    /// Retorna o nome do ícone simbólico para este tab.
    ///
    /// Estes nomes seguem o padrão FreeDesktop Icon Naming Specification
    /// e são automaticamente resolvidos pelo sistema de temas do COSMIC.
    ///
    /// # Retorno
    /// String estática com o nome do ícone (ex: "office-calendar-symbolic")
    pub fn icon_name(&self) -> &'static str {
        match self {
            Tab::Calendar => "office-calendar-symbolic",
            Tab::Weather => "weather-clear-symbolic",
            Tab::Timer => "alarm-symbolic",
        }
    }

    /// Retorna o label traduzido para este tab.
    ///
    /// # Nota
    /// Esta função será útil quando implementarmos i18n completo nas tabs.
    /// Por enquanto, retorna strings em inglês.
    pub fn label(&self) -> &'static str {
        match self {
            Tab::Calendar => "Calendar",
            Tab::Weather => "Weather",
            Tab::Timer => "Timer",
        }
    }
}

// ============================================================================
// Application Entry Point (Neutral Messenger)
// ============================================================================
// lib.rs serves as the neutral messenger: receives config from main.rs
// and passes it to the COSMIC runtime. This maintains architectural separation.

pub fn run(config: TimeAppletConfig) -> cosmic::iced::Result {
    localize::localize();

    cosmic::applet::run::<Window>(config)
}
