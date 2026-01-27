// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

//! Subscription management module
//!
//! This module handles all heavy subscription logic for the applet,
//! including time ticking, timezone updates, and wake-from-sleep detection.

use chrono::Timelike;
use cosmic::{
    iced::Subscription,
    iced::futures::{SinkExt, StreamExt, channel::mpsc},
    iced_futures::stream,
};
use logind_zbus::manager::ManagerProxy;
use timedate_zbus::TimeDateProxy;
use tokio::{sync::watch, time};

use crate::Message;

// ============================================================================
// Time Subscription
// ============================================================================

/// Time ticking subscription
///
/// Manages the applet's time updates, supporting both per-second and per-minute
/// ticking based on the show_seconds configuration.
pub fn time_subscription(mut show_seconds: watch::Receiver<bool>) -> Subscription<Message> {
    Subscription::run_with_id(
        "time-sub",
        stream::channel(1, |mut output| async move {
            // Mark this receiver's state as changed so that it always receives an initial
            // update during the loop below
            // This allows us to avoid duplicating code from the loop
            show_seconds.mark_changed();
            let mut period = 1;
            let mut timer = time::interval(time::Duration::from_secs(period));
            timer.set_missed_tick_behavior(time::MissedTickBehavior::Skip);

            loop {
                tokio::select! {
                    _ = timer.tick() => {
                        #[cfg(debug_assertions)]
                        if let Err(err) = output.send(Message::Tick).await {
                            tracing::error!(?err, "Failed sending tick request to applet");
                        }
                        #[cfg(not(debug_assertions))]
                        let _ = output.send(Message::Tick).await;

                        // Calculate a delta if we're ticking per minute to keep ticks stable
                        // Based on i3status-rust
                        let current = chrono::Local::now().second() as u64 % period;
                        if current != 0 {
                            timer.reset_after(time::Duration::from_secs(period - current));
                        }
                    },
                    // Update timer if the user toggles show_seconds
                    Ok(()) = show_seconds.changed() => {
                        let seconds = *show_seconds.borrow_and_update();
                        if seconds {
                            period = 1;
                            // Subsecond precision isn't needed; skip calculating offset
                            let period = time::Duration::from_secs(period);
                            let start = time::Instant::now() + period;
                            timer = time::interval_at(start, period);
                        } else {
                            period = 60;
                            let delta = time::Duration::from_secs(period - chrono::Utc::now().second() as u64 % period);
                            let now = time::Instant::now();
                            // Start ticking from the next minute to update the time properly
                            let start = now + delta;
                            let period = time::Duration::from_secs(period);
                            timer = time::interval_at(start, period);
                        }

                        timer.set_missed_tick_behavior(time::MissedTickBehavior::Skip);
                    }
                }
            }
        }),
    )
}

// ============================================================================
// Timezone Subscription
// ============================================================================

/// Update applet's timezone if the system's timezone changes
async fn timezone_update(output: &mut mpsc::Sender<Message>) -> zbus::Result<()> {
    let conn = zbus::Connection::system().await?;
    let proxy = TimeDateProxy::new(&conn).await?;

    // The stream always returns the current timezone as its first item even if it wasn't
    // updated. If the proxy is recreated in a loop somehow, the resulting stream will
    // always yield an update immediately which could lead to spammed false updates.
    let mut stream_tz = proxy.receive_timezone_changed().await;
    while let Some(property) = stream_tz.next().await {
        let tz = property.get().await?;
        output
            .send(Message::TimezoneUpdate(tz))
            .await
            .map_err(|e| zbus::Error::InputOutput(std::sync::Arc::new(std::io::Error::other(e))))?;
    }
    Ok(())
}

/// Timezone change subscription
///
/// Monitors system timezone changes and updates the applet accordingly.
pub fn timezone_subscription() -> Subscription<Message> {
    Subscription::run_with_id(
        "timezone-sub",
        stream::channel(1, |mut output| async move {
            'retry: loop {
                match timezone_update(&mut output).await {
                    Ok(()) => break 'retry,
                    Err(err) => {
                        tracing::error!(
                            ?err,
                            "Automatic timezone updater failed; retrying in one minute"
                        );
                        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                    }
                }
            }

            std::future::pending().await
        }),
    )
}

// ============================================================================
// Wake from Sleep Subscription
// ============================================================================

/// Update the time when waking from sleep
async fn wake_from_sleep(output: &mut mpsc::Sender<Message>) -> zbus::Result<()> {
    let connection = zbus::Connection::system().await?;
    let proxy = ManagerProxy::new(&connection).await?;

    while let Some(property) = proxy.receive_prepare_for_sleep().await?.next().await {
        let waking = !property.args()?.start();
        if waking {
            let _ = output.send(Message::Tick).await;
        }
    }
    Ok(())
}

/// Wake from sleep subscription
///
/// Detects when the system wakes from sleep and triggers a time update.
pub fn wake_from_sleep_subscription() -> Subscription<Message> {
    Subscription::run_with_id(
        "wake-from-suspend-sub",
        stream::channel(1, |mut output| async move {
            if let Err(err) = wake_from_sleep(&mut output).await {
                tracing::error!(?err, "Failed to subscribe to wake-from-sleep signal");
            }
        }),
    )
}
