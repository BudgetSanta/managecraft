use clap::Clap;
use std::fmt::{Display, Formatter, Result};
use std::option::Option;

/// Parse in command line arguments
pub fn parse_opts() -> Opts {
    Opts::parse()
}

#[derive(Clap)]
pub struct Opts {
    /// Supresses normal output
    #[clap(short, long)]
    pub quiet: bool,

    /// Prints out debug messages
    #[clap(short, long)]
    pub debug: bool,

    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    Execute(Execute),
    Say(Say),
    SaveAll(SaveAll),
}

/// Execute an arbitrary command
#[derive(Clap)]
pub struct Execute {
    /// Arbitrary command
    pub command: String,
}

/// Broadcast a message to the server
#[derive(Clap)]
pub struct Say {
    /// Message content
    pub message: String,
}

/// Save the server world state
#[derive(Clap)]
pub struct SaveAll {
    /// Force all the chunks to be saved to disk
    /// immediately, freezing the server for a
    /// short time.
    #[clap(short, long)]
    pub flush: bool,
}

impl Display for Execute {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Executing: '{}'", self.command)
    }
}

impl Display for Say {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Saying: '{}'", self.message)
    }
}

impl Display for SaveAll {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Saving: (flush: {})", self.flush)
    }
}
