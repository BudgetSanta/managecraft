use crate::Client;
use clap::Clap;

/// Broadcast a message to the server
#[derive(Clap)]
pub struct Say {
    /// Message content
    pub message: String,
}

impl Client {
    /// Broadcasts the message provided to the server
    pub async fn say(&mut self, s: Say) -> Result<String, rcon::Error> {
        let command = format!("say {}", s.message);
        Ok(self.send(&command).await?)
    }
}
