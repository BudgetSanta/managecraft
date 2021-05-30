use clap::Clap;

#[derive(Clap)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

pub fn parse_opts() -> Opts {
    Opts::parse()
}

#[derive(Clap)]
pub enum SubCommand {
    /// Broadcast message to the server
    Say(Say),
    /// Save the server world state
    SaveAll(SaveAll),
    /// List all players online
    List,
}

#[derive(Clap)]
pub struct Say {
    /// Message content
    pub message: String,
}

#[derive(Clap)]
pub struct SaveAll {
    /// All the chunks are saved to the data storage device immediately, freezing the server for a short time.
    #[clap(short, long)]
    pub flush: bool,
}
