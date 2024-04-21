//! CLI parsing utilities.

mod env;
mod logging;
mod styles;

use clap::Parser;

use crate::pianobar::eventcmd::EventCmd;

/// A prettier CLI for pianobar, a console-based Pandora client.
#[derive(Debug, Parser)]
#[clap(version, styles=styles::STYLES)]
pub struct Args {
    /// A pianobar eventcmd
    pub eventcmd: Option<EventCmd>,
    /// Control logging verbosity
    #[arg(
        long,
        value_enum,
        env = env::LOG_LEVEL,
        default_value_t = logging::LogLevel::default()
    )]
    pub log_level: logging::LogLevel,
}
