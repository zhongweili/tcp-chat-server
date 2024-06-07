use anyhow::Result;
use server::{network::run_server, state::State};
use std::sync::Arc;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};
mod server;

const LISTEN_ADDR: &str = "0.0.0.0:8877";

#[derive(Debug, thiserror::Error)]
pub enum ChatError {
    #[error("network error")]
    NetworkError(#[from] std::io::Error),
    #[error("decode error")]
    DecodeError(#[from] tokio_util::codec::LinesCodecError),
    #[error("username already taken")]
    UsernameTaken,
    #[error("invalid username")]
    InvalidUsername,
}

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let state = Arc::new(State::default());
    run_server(LISTEN_ADDR, state).await
}
