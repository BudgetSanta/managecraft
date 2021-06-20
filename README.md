# `managecraft` :building_construction:
A CLI Utility to communicate with your Minecraft server over the RCON protocol

[![Crates Badge]][Crates] [![Docs Badge]][Docs] [![Build Badge]][Build] [![License Badge]][License]

## Command Line Utility Quick Start
Use `managecraft --help` or `managecraft <subcommand> --help` for more information about available commands

```shell
managecraft say "Hello Server!"
```
_You'll see output of `[Rcon] Hello Server!` in the server log as well as in game_

## Library Quick Start
```rust
// Create Server Settings and Client
let settings = settings::Settings::new()?;
let mut client = Client::new(settings).await?;

// Send a command
let c = String::from("time set day");
client.execute(commands::Execute { command: c }).await?;;
```

## Configuration
Setting up configuration for connecting to the server can be done in three ways. Each subsequent step will update any matching values from previous setps.

1. Managecraft will set default values for all properties
```
Url: 0.0.0.0
Port: 25575
Password: "test"
```

2. A configuration file can be created in your home diretory at `$HOME/.config/managecraft.toml` to overwrite some or all default values

_Creating a config file with only a pasword field will leave all default values but overwrite the password value_
```toml
# $HOME/.config/managecraft.toml

password="hunter2"
```

3. Environment variables can also be created to set configuration values. Any environment variable prefixed with `MANAGECRAFT_` will be considered and if matches will overwrite a property.

_This will set the port value to `12345` overwriting default config as well as a port value specified in the config file from step 2._
```shell
MANAGECRAFT_PORT=12345 managecraft save-all
```

## Command Usage

### Execute
_Executes an arbitrary command on the server_

`execute <command>`

Check out https://minecraft.fandom.com/wiki/Commands for all available commands

### Say
_Broadcast a message to the server_

`say <message>` 

### Save All
_Save the game world_

`save_all [-f|--flush]` 

[Crates]: https://crates.io/crates/managecraft
[Crates Badge]: https://img.shields.io/crates/v/managecraft

[Docs]: https://docs.rs/managecraft
[Docs Badge]: https://docs.rs/managecraft/badge.svg

[Build]: https://github.com/jayman888/managecraft/actions/workflows/release.yml
[Build Badge]: https://github.com/jayman888/managecraft/actions/workflows/release.yml/badge.svg?branch=master

[License]: https://spdx.org/licenses/MIT.html
[License Badge]: https://img.shields.io/badge/License-MIT-blue.svg
