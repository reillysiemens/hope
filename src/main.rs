#![forbid(unsafe_code)]
use clap::Parser;

use crate::pianobar::event::Event;

mod cli;
mod client;
mod pianobar;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    tracing_subscriber::fmt()
        .with_max_level(args.log_level)
        .init();
    match args.eventcmd {
        Some(eventcmd) => {
            tracing::debug!("Handling {eventcmd:#?} eventcmd");
            let info = client::read_info().await?;
            let event = Event { eventcmd, info };
            client::send_event(&args, &event).await?;
        }
        None => {
            tracing::debug!("No eventcmd to handle, running main program");
            server::serve(&args).await?;
        }
    }
    Ok(())
}
