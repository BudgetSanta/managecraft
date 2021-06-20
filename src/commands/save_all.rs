use crate::Client;
use clap::Clap;

/// Save the server world state
#[derive(Clap)]
pub struct SaveAll {
    /// Force all the chunks to be saved to disk
    /// immediately, freezing the server for a
    /// short time.
    #[clap(short, long)]
    pub flush: bool,
}

impl Client {
    /// Saves the world state to disk with the option to
    /// temporarily freeze the server and flush save data
    /// saving quicker
    pub async fn save_all(&mut self, s: SaveAll) -> Result<String, rcon::Error> {
        let command = format!("save-all{}", if s.flush { " flush" } else { "" });
        Ok(self.send(&command).await?)
    }
}
