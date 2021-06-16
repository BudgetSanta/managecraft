//! # managecraft
//!
//! `managecraft` is a command line tool to send commands to a running Minecraft
//! server over the RCON protocol

/// A collection of commands for interacting
/// with managecraft and the server
pub mod commands;

/// Application Settings
pub mod settings;

use rcon::Connection;

use commands::{Execute, SaveAll, Say};
use settings::Settings;

/// Client used to communicate with the Minecraft server
pub struct Client {
    /// Url That `connection` is pointing to
    pub url: String,
    /// Port that `connection` is pointing to
    pub port: u16,
    /// RCON Connection client uses to communicate with server
    pub connection: rcon::Connection,
}

impl Client {
    /// Instantiates a new client for communicating with
    /// the minecraft server. This will also attempt
    /// to connect to the server
    pub async fn new(props: Settings) -> Result<Client, rcon::Error> {
        let conn = Connection::builder()
            .enable_minecraft_quirks(true)
            .connect(format!("{}:{}", &props.url, props.port), &props.password)
            .await?;

        Ok(Client {
            url: props.url,
            port: props.port,
            connection: conn,
        })
    }

    /// An abstraction of Client.Connection.Cmd(c) to Client.Send(c)
    async fn send(&mut self, command: &str) -> std::result::Result<String, rcon::Error> {
        Ok(self.connection.cmd(command).await?)
    }

    /// Executes an arbitrary command on the server
    pub async fn execute(&mut self, ex: Execute) -> Result<String, rcon::Error> {
        Ok(self.send(&ex.command).await?)
    }

    /// Broadcasts the message provided to the server
    pub async fn say(&mut self, s: Say) -> Result<String, rcon::Error> {
        let command = format!("say {}", s.message);
        Ok(self.send(&command).await?)
    }

    /// Saves the world state to disk with the option to
    /// temporarily freeze the server and flush save data
    /// saving quicker
    pub async fn save_all(&mut self, s: SaveAll) -> Result<String, rcon::Error> {
        let command = format!("save-all{}", if s.flush { " flush" } else { "" });
        Ok(self.send(&command).await?)
    }
}
