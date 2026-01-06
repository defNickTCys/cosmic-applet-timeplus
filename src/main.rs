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

    // Configure logging level
    if args.debug {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
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
