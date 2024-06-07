use dashmap::DashMap;
use futures::{SinkExt, StreamExt};
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpStream, sync::mpsc};
use tokio_util::codec::{Framed, LinesCodec};
use tracing::warn;

use crate::ChatError;

use super::{message::Message, user::Peer};

const MAX_MESSAGES: usize = 128;

#[derive(Debug, Default)]
pub struct State {
    pub peers: DashMap<SocketAddr, mpsc::Sender<Arc<Message>>>,
    pub usernames: DashMap<SocketAddr, String>,
}
impl State {
    pub async fn add(
        &self,
        addr: SocketAddr,
        username: String,
        stream: Framed<TcpStream, LinesCodec>,
    ) -> Result<Peer, ChatError> {
        if self.is_username_taken(&username) {
            return Err(ChatError::UsernameTaken);
        }

        let (tx, mut rx) = mpsc::channel(MAX_MESSAGES);
        self.peers.insert(addr, tx);
        self.usernames.insert(addr, username.clone());

        let (mut stream_sender, stream_receiver) = stream.split();

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                if let Err(e) = stream_sender.send(message.to_string()).await {
                    warn!("Failed to send message to {}: {}", addr, e);
                    break;
                }
            }
        });

        Peer::new(&username, stream_receiver)
    }

    pub fn is_username_taken(&self, username: &str) -> bool {
        self.usernames.iter().any(|user| user.value() == username)
    }

    pub async fn broadcast(&self, addr: SocketAddr, message: Arc<Message>) {
        for peer in self.peers.iter() {
            if peer.key() == &addr {
                continue;
            }
            if let Err(e) = peer.value().send(message.clone()).await {
                warn!("Failed to send message to {}: {}", peer.key(), e);
                self.peers.remove(peer.key());
            }
        }
    }
}
