//! # managecraft
//!
//! `managecraft` is a command line tool to send commands to a running Minecraft
//! server over the RCON protocol

/// A collection of commands for interacting
/// with managecraft and the server
pub mod commands;

/// Application Settings
pub mod settings;
use settings::Settings;

use rcon::Connection;

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
}
