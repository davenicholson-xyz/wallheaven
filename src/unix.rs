use std::{collections::HashMap, env};

use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Command {
    command: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    linux: HashMap<String, Command>,
    macos: Command,
}

fn load_commands() -> Config {
    let commands_toml = include_str!("setcommands.toml");
    toml::from_str(commands_toml).expect("Failed to parse commands")
}

pub fn set_wallpaper(filepath: &str) -> Result<()> {
    let config = load_commands();
    let os = env::consts::OS;

    if os == "linux" {
        let desktop_session = env::var("DESKTOP_SESSION").unwrap_or_default();
        if let Some(config_command) = config.linux.get(&desktop_session) {
            let cmd_string = config_command.command.replace("{IMG}", filepath);
            println!("{}", cmd_string);
            return Ok(());
        } else {
            println!("{}", filepath);
            return Ok(());
        }
    } else if os == "macos" {
        let cmd_string = config.macos.command.replace("{IMG}", filepath);
        println!("{}", cmd_string);
        return Ok(());
    } else {
        println!("{}", filepath);
        return Ok(());
    }
}
