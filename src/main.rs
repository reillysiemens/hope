#![forbid(unsafe_code)]
use std::io::Read;

use clap::Parser;

use crate::pianobar::event::Event;

mod cli;
mod pianobar;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    tracing_subscriber::fmt()
        .with_max_level(args.log_level)
        .init();
    match args.eventcmd {
        Some(eventcmd) => {
            tracing::debug!("Handling {eventcmd:#?} eventcmd");
            let mut info = String::new();
            std::io::stdin().read_to_string(&mut info)?;
            let info = info.parse()?;
            let event = Event { eventcmd, info };
            tracing::debug!("{event:#?}");
        }
        None => tracing::debug!("No eventcmd to handle"),
    }
    Ok(())
}
