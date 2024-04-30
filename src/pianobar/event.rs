//! Pianobar events.

mod eventcmd;
mod info;

pub use eventcmd::EventCmd;
pub use info::Info;

/// A pianobar event.
///
/// This contains both the [EventCmd] that triggered it and the [Info] about it.
#[derive(Debug)]
pub struct Event {
    pub eventcmd: EventCmd,
    pub info: Info,
}
