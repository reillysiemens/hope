use tokio::io::AsyncReadExt;

use crate::{cli::Args, pianobar::event::Event};

pub async fn serve(args: &Args) -> anyhow::Result<()> {
    if args.socket.exists() {
        tracing::trace!("Removing pre-existing Unix socket at {}", args.socket);
        // TODO: This should also be deleted when the program exits.
        tokio::fs::remove_file(&args.socket).await?;
    }

    tracing::debug!("Listening on {}", args.socket);
    let socket = tokio::net::UnixSocket::new_stream()?;
    socket.bind(&args.socket)?;
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
