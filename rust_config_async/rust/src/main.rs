use anyhow::Context;
use tokio::signal::unix::{self, SignalKind};
use tokio_util::sync::CancellationToken;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use crate::config::Config;

mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscribe()?;

    let config = Config::from_env()?;
    tracing::debug!(?config);

    let token = CancellationToken::new();
    // TODO: start subtasks

    // can't forget the sigterm hook (or it won't stop in a Docker container)
    let mut sigterm = unix::signal(SignalKind::terminate()).context("setting up sigterm hook")?;
    // can't forget the sigint hook (or it won't listen to ctrl-c)
    let mut sigint = unix::signal(SignalKind::interrupt()).context("setting up sigint hook")?;
    tokio::select! {
        _ = sigterm.recv() => {
            token.cancel();
            tracing::info!("Shutting down application (received SIGTERM)");
        }
        _ = sigint.recv() => {
            token.cancel();
            tracing::info!("Shutting down application (received SIGINT)");
        }
        _ = token.cancelled() => {}
    }

    // TODO: wait for subtask shutdown
    tracing::info!("Application is shutdown");

    Ok(())
}

/// Setup basic tracing
///
/// This is the recommended crate to use, it allows for an environment variable
/// filter `RUST_LOG` for easier debugging. See [here](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives).
fn tracing_subscribe() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy()
        )
        .finish();
    tracing::subscriber::set_global_default(subscriber).context("Subscribing to tracing")
}

