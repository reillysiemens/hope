//! Pianobar events.

mod eventcmd;
mod info;

pub use eventcmd::EventCmd;
pub use info::Info;

#[derive(Debug)]
pub struct Event {
    pub eventcmd: EventCmd,
    pub info: Info,
}
