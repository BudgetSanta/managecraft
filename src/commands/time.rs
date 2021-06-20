use crate::Client;
use clap::Clap;

/// Set and Query the game time
#[derive(Clap)]
pub struct Time {
    #[clap(short, long)]
    pub add: String,

    #[clap(short, long)]
    pub set: String,

    #[clap(short, long)]
    pub query: String,
}

impl Client {
    /// Broadcasts the message provided to the server
    pub async fn time(&mut self, t: Time) -> Result<String, rcon::Error> {
        todo!();
    }
}
