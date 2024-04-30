//! CLI parsing utilities.

mod env;
mod logging;
mod styles;

use camino::Utf8PathBuf;
use clap::Parser;

use crate::pianobar::event::EventCmd;

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
    /// The path to the Unix socket used for IPC
    #[arg(long, env = env::SOCKET, default_value_t = Utf8PathBuf::from("/tmp/hope.sock"))]
    pub socket: Utf8PathBuf,
}
