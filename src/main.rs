use clap::Parser;

mod cli;
mod pianobar;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    tracing_subscriber::fmt()
        .with_max_level(args.log_level)
        .init();
    match args.eventcmd {
        Some(event) => tracing::debug!("Handling {event:#?} eventcmd"),
        None => tracing::debug!("No eventcmd to handle"),
    }
    Ok(())
}
