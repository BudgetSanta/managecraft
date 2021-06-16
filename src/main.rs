use managecraft::commands::{parse_opts, SubCommand};
use managecraft::settings::Settings;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opts = parse_opts();
    let settings = Settings::new()?;

    let mut client = instantiate_client(settings).await;

    let res = match opts.subcmd {
        SubCommand::Execute(ex) => {
            println!("{}", ex);
            client.execute(ex).await?
        }
        SubCommand::Say(s) => {
            println!("{}", s);
            client.say(s).await?
        }
        SubCommand::SaveAll(s) => {
            println!("{}", s);
            client.save_all(s).await?
        }
    };

    if !res.trim().is_empty() {
        println!("Response: {}", res)
    }

    Ok(())
}

async fn instantiate_client(settings: Settings) -> managecraft::Client {
    println!(
        "Attempting to connect to {}:{}",
        settings.url, settings.port
    );

    match managecraft::Client::new(settings).await {
        Ok(c) => {
            println!("Successfully connected to server!");
            return c;
        }
        Err(rcon::Error::Auth) => {
            eprintln!("Authentication failed");
        }
        Err(rcon::Error::Io(e)) => match e.kind() {
            std::io::ErrorKind::ConnectionRefused => {
                eprintln!("Connection Refused: Is the server running?")
            }
            _ => eprintln!("IO Error: {}", e),
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    };

    std::process::exit(1);
}
