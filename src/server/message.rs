use std::fmt;

#[derive(Debug)]
pub enum Message {
    UserJoined(String),
    UserLeft(String),
    Chat { sender: String, content: String },
}

impl Message {
    pub fn new_user_joined(username: &str) -> Self {
        let content = format!("{} has joined the chat", username);
        Self::UserJoined(content)
    }

    pub fn new_user_left(username: &str) -> Self {
        let content = format!("{} has left the chat", username);
        Self::UserLeft(content)
    }

    pub fn new_chat(sender: &str, content: &str) -> Self {
        Self::Chat {
            sender: sender.to_string(),
            content: content.to_string(),
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UserJoined(content) => write!(f, "[{}]", content),
            Self::UserLeft(content) => write!(f, "[{} :(]", content),
            Self::Chat { sender, content } => write!(f, "{}: {}", sender, content),
        }
    }
}
