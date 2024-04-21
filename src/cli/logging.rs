//! Logging type conversions.

use clap::ValueEnum;
use tracing::level_filters::LevelFilter;

// Allow users to set log level as an enum, restricting them to valid choices.
#[derive(Copy, Clone, Debug, Default, ValueEnum)]
pub enum LogLevel {
    Off,
    Error,
    #[default]
    Warn,
    Info,
    Debug,
    Trace,
}

// Allow for easy conversion from our custom LogLevel to a tracing LevelFilter.
impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Off => LevelFilter::OFF,
            LogLevel::Error => LevelFilter::ERROR,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Trace => LevelFilter::TRACE,
        }
    }
}
