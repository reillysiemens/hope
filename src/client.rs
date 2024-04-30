use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{
    cli::Args,
    pianobar::event::{Event, Info},
};

pub async fn read_info() -> anyhow::Result<Info> {
    let mut info = String::new();
    tokio::io::stdin().read_to_string(&mut info).await?;
    let info = info.parse()?;
    Ok(info)
}

pub async fn send_event(args: &Args, event: &Event) -> anyhow::Result<()> {
    let data = serde_json::to_string(event)?;
    let socket = tokio::net::UnixSocket::new_stream()?;
    let mut stream = socket.connect(&args.socket).await?;
    stream.write_all(data.as_bytes()).await?;
    Ok(())
}
