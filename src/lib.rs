// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

mod calendar; // Calendar module (renamed from time)
mod config;
mod localize;
mod panel; // Panel UI module
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

// ============================================================================
// Application Entry Point
// ============================================================================

pub fn run() -> cosmic::iced::Result {
    localize::localize();

    cosmic::applet::run::<Window>(())
}
