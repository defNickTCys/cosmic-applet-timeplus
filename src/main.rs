// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use clap::Parser;
use cosmic_applet_timeplus::config::TimeAppletConfig;
use std::path::PathBuf;

/// COSMIC Time+ Applet
#[derive(Parser, Debug)]
#[command(name = "cosmic-applet-timeplus")]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// Path to custom configuration file
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Enable debug mode (verbose logging)
    #[arg(short, long)]
    debug: bool,
}

fn main() -> cosmic::iced::Result {
    let args = CliArgs::parse();

    // Configure logging level with noise filtering
    if args.debug {
        // Debug mode: show cosmic-applet-timeplus logs, suppress noisy dependencies
        // User can override with RUST_LOG environment variable
        if std::env::var("RUST_LOG").is_err() {
            // SAFETY: Setting RUST_LOG at startup before any threads are spawned
            unsafe {
                std::env::set_var(
                    "RUST_LOG",
                    "cosmic_applet_timeplus=debug,i18n_embed=warn,wgpu=warn",
                );
            }
        }
        tracing_subscriber::fmt::init();
    } else {
        tracing_subscriber::fmt::init();
        let _ = tracing_log::LogTracer::init();
    }

    // Note: Custom config path is not yet supported
    // COSMIC handles config loading/updates via watch_config subscription
    if args.config.is_some() {
        eprintln!("⚠️  Custom config path not yet supported. Using system config.");
    }

    // Start with default config - COSMIC will update via ConfigChanged message
    let config = TimeAppletConfig::default();

    // Initialize application via lib.rs (neutral messenger pattern)
    cosmic_applet_timeplus::run(config)
}
