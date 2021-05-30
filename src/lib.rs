mod app_commands;

use app_commands::{parse_opts, SubCommand};
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let opts = parse_opts();

    match opts.subcmd {
        SubCommand::Say(s) => {
            println!("/say {}", s.message)
        }
        SubCommand::SaveAll(s) => {
            if s.flush {
                println!("/save-all flush")
            } else {
                println!("/save-all")
            }
        }
        SubCommand::List => println!("/list"),
    }

    Ok(())
}
