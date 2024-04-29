//! Pianobar events.

mod eventcmd;
mod info;

pub use eventcmd::EventCmd;
pub use info::Info;

#[derive(Debug)]
pub struct Event {
    eventcmd: EventCmd,
    info: Info,
}

impl Event {
    pub fn new(eventcmd: EventCmd, info: Info) -> Self {
        Self { eventcmd, info }
    }
}
