use managecraft::commands::{parse_opts, Opts, SubCommand};
use managecraft::settings::Settings;
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opts = parse_opts();

    let log_level = get_log_level(&opts);

    TermLogger::init(
        log_level,
        Config::default(),
        TerminalMode::Mixed,
        simplelog::ColorChoice::Always,
    )?;

    let settings = Settings::new()?;
    log::debug!("App Settings: {:#?}", settings);

    let mut client = instantiate_client(settings).await;

    let res = match opts.subcmd {
        SubCommand::Execute(ex) => {
            log::info!("Executing: {}", ex.command);
            client.execute(ex).await?
        }
        SubCommand::Say(s) => {
            log::info!("Saying: {}", s.message);
            client.say(s).await?
        }
        SubCommand::SaveAll(s) => {
            log::info!("Saving...");
            client.save_all(s).await?
        }
        SubCommand::Time(t) => todo!(),
    };

    if !res.trim().is_empty() {
        log::info!("Response: {}", res)
    }

    Ok(())
}

async fn instantiate_client(settings: Settings) -> managecraft::Client {
    match managecraft::Client::new(settings).await {
        Ok(c) => {
            log::info!("Successfully conncted to server");
            return c;
        }
        Err(rcon::Error::Auth) => {
            log::error!("Authentication failed");
        }
        Err(rcon::Error::Io(e)) => match e.kind() {
            std::io::ErrorKind::ConnectionRefused => {
                log::error!("Connection Refused. Is the server running?");
            }
            _ => log::error!("IO Error: {}", e),
        },
        Err(e) => {
            log::error!("Error: {}", e);
        }
    };

    std::process::exit(1);
}

fn get_log_level(opts: &Opts) -> LevelFilter {
    if opts.debug {
        return LevelFilter::Debug;
    } else if opts.quiet {
        return LevelFilter::Error;
    } else {
        return LevelFilter::Info;
    }
}
