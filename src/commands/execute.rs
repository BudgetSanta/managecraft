use crate::Client;
use clap::Clap;

/// Execute an arbitrary command
#[derive(Clap)]
pub struct Execute {
    /// Arbitrary command
    pub command: String,
}

impl Client {
    /// Executes an arbitrary command on the server
    pub async fn execute(&mut self, ex: Execute) -> Result<String, rcon::Error> {
        Ok(self.send(&ex.command).await?)
    }
}
