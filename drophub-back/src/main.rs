mod cli;
mod config;
mod jwt;
mod server;
#[cfg(test)]
mod test_utils;

use clap::Parser;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, Registry};

use crate::{cli::Cli, config::Config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_logging()?;
    run_server().await
}

fn init_logging() -> anyhow::Result<()> {
    let formatting_layer =
        BunyanFormattingLayer::new(env!("CARGO_PKG_NAME").to_owned(), std::io::stdout);
    let subscriber = Registry::default()
        .with(JsonStorageLayer)
        .with(formatting_layer);
    tracing::subscriber::set_global_default(subscriber)?;

    Ok(())
}

async fn run_server() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let cfg = Config::new(cli.config_path.as_deref())?;
    let (_, handle) = server::run(&cfg).await?;
    handle.stopped().await;

    Ok(())
}
