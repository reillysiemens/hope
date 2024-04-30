#![forbid(unsafe_code)]
use std::io::Read;

use clap::Parser;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

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
            let event = Event {
                eventcmd: eventcmd.clone(),
                info,
            };
            tracing::trace!("{event:#?}");

            let data = serde_json::to_string(&event)?;
            let socket = tokio::net::UnixSocket::new_stream()?;
            tracing::trace!("Connecting to Unix socket at {}", args.socket);
            let mut stream = socket.connect(args.socket).await?;
            stream.write_all(data.as_bytes()).await?;
            tracing::trace!("Sent {eventcmd:#?} eventcmd to remote");
        }
        None => {
            tracing::debug!("No eventcmd to handle, running main program");

            if args.socket.exists() {
                tracing::trace!("Removing pre-existing Unix socket at {}", args.socket);
                // TODO: This should also be deleted when the program exits.
                tokio::fs::remove_file(&args.socket).await?;
            }

            tracing::debug!("Listening on {}", args.socket);
            let socket = tokio::net::UnixSocket::new_stream()?;
            socket.bind(args.socket)?;
            // TODO: Should backlog be more than 1 here?
            let listener = socket.listen(1)?;

            loop {
                tracing::debug!("Waiting for new client connection");
                let (mut stream, _) = listener.accept().await?;
                tracing::trace!("Received new client connection");
                let mut data = String::new();
                stream.read_to_string(&mut data).await?;
                let event: Event = serde_json::from_str(&data)?;
                tracing::debug!("Received {:?} eventcmd", event.eventcmd);
                tracing::trace!("{event:#?}");
            }
        }
    }
    Ok(())
}
