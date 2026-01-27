// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

//! Centralized icon name constants
//!
//! Single Source of Truth for all icon names used across the applet.
//! Prevents hardcoded strings and ensures consistency.

/// Navigation icons
pub mod navigation {
    /// Previous/back navigation icon (left arrow)
    pub const PREVIOUS: &str = "go-previous-symbolic";

    /// Next/forward navigation icon (right arrow)
    pub const NEXT: &str = "go-next-symbolic";
}

/// Generic UI icons
pub mod ui {
    /// Starred/featured item indicator icon
    pub const STARRED: &str = "starred-symbolic";
}
