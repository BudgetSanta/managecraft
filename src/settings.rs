use config::{Config, Environment, File};
use serde::Deserialize;

/// Configuration values for connecting to the minecraft server
#[derive(Debug, Deserialize)]
pub struct Settings {
    /// Minectaft Server URL
    pub url: String,
    /// RCON Protocol Port
    pub port: u16,
    /// RCON Authentication Password
    pub password: String,
}

impl Settings {
    /// Instantiate new server settings
    ///
    /// # Configuration
    /// 1. Values will first default to static values
    ///  - Url: `0.0.0.0`
    ///  - Port: `25575`
    ///  - Password: `test`
    ///
    /// 2. If it exists, the config file at `$HOME/.config/managecraft.toml`
    /// will be used to override values
    ///
    /// 3. Any setting property can be prefixed with `MANAGECRAFT_` as
    /// an environment variable to override it
    ///  - eg. `MANAGECRAFT_PASSWORD="hunter2"`
    pub fn new() -> Result<Self, config::ConfigError> {
        let mut s = Config::default();

        // Lazy default config
        s.set_default("url", String::from("0.0.0.0"))?;
        s.set_default("port", 25575)?;
        s.set_default("password", String::from("test"))?;

        // Read in settings from config file
        if let Some(user_dir) = directories::UserDirs::new() {
            s.merge(
                File::from(user_dir.home_dir().join(".config/managecraft.toml")).required(false),
            )?;
        }

        // Override settings from the environment
        s.merge(Environment::with_prefix("managecraft"))?;

        s.try_into()
    }
}
