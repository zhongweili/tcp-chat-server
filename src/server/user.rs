use futures::stream::SplitStream;
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, LinesCodec};

use crate::ChatError;

#[derive(Debug)]
pub struct Peer {
    pub username: String,
    pub stream: SplitStream<Framed<TcpStream, LinesCodec>>,
}

impl Peer {
    pub fn new(
        username: &str,
        stream: SplitStream<Framed<TcpStream, LinesCodec>>,
    ) -> Result<Self, ChatError> {
        if username.len() < 3 || username.len() > 20 || !username.chars().all(char::is_alphanumeric)
        {
            return Err(ChatError::InvalidUsername);
        }
        Ok(Peer {
            username: username.to_string(),
            stream,
        })
    }
}
