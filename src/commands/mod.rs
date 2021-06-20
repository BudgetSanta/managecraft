use clap::Clap;
use std::option::Option;

mod time;
pub use self::time::Time;

mod execute;
pub use self::execute::Execute;

mod say;
pub use self::say::Say;

mod save_all;
pub use self::save_all::SaveAll;

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
    Time(Time),
}
