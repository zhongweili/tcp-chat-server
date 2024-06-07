use anyhow::Result;
use futures::{SinkExt, StreamExt};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{Framed, LinesCodec};
use tracing::{info, warn};

use crate::server::{message::Message, user::Peer};

use super::state::State;

pub async fn handle_connection(
    state: Arc<State>,
    addr: SocketAddr,
    stream: TcpStream,
) -> Result<()> {
    let mut stream = Framed::new(stream, LinesCodec::new());
    stream.send("Enter your username:").await?;

    let username = match stream.next().await {
        Some(Ok(username)) => username,
        Some(Err(e)) => return Err(e.into()),
        None => return Ok(()),
    };

    let mut peer: Peer = state.add(addr, username, stream).await?;

    let message = Arc::new(Message::new_user_joined(&peer.username));
    info!("{}", message);
    state.broadcast(addr, message).await;

    while let Some(line) = peer.stream.next().await {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                warn!("Failed to read line from {}: {}", addr, e);
                break;
            }
        };

        let message = Arc::new(Message::new_chat(&peer.username, &line));
        state.broadcast(addr, message).await;
    }

    state.peers.remove(&addr);

    let message = Arc::new(Message::new_user_left(&peer.username));
    info!("{}", message);
    state.broadcast(addr, message).await;

    Ok(())
}

pub async fn run_server(addr: &str, state: Arc<State>) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;
    info!("Starting chat server on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Accepted connection from: {}", addr);
        let state_cloned = state.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(state_cloned, addr, stream).await {
                warn!("Failed to handle client {}: {}", addr, e);
            }
        });
    }
}
