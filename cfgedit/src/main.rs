use std::fs;
use std::io::{self};
use std::path::PathBuf;
use std::process::Command;
use serde::{Deserialize, Serialize};
use ansi_term::Colour;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    aliases: std::collections::HashMap<String, String>,
}

impl Config {
    fn load(config_path: &PathBuf) -> Self {
        if config_path.exists() {
            let data = fs::read_to_string(config_path).unwrap_or_else(|_| "{}".to_string());
            serde_json::from_str(&data).unwrap_or(Self {
                aliases: std::collections::HashMap::new(),
            })
        } else {
            Self {
                aliases: std::collections::HashMap::new(),
            }
        }
    }

    fn save(&self, config_path: &PathBuf) {
        let data = serde_json::to_string_pretty(self).expect("Failed to serialize configuration");
        fs::write(config_path, data).expect("Failed to write configuration file");
    }
}

fn main() {
    let home_dir = dirs::home_dir().expect("Unable to find home directory");
    let config_path = home_dir.join(".config/rust-utils/configs.json");

    let mut config = Config::load(&config_path);
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cfgedit <command> [options]");
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() != 4 {
                eprintln!("Usage: cfgedit add <alias> <path>");
                return;
            }
            let alias = &args[2];
            let path = &args[3];
            config.aliases.insert(alias.clone(), path.clone());
            config.save(&config_path);
            println!("Alias '{}' added for path '{}'.", alias, path);
        }
        "view" => {
            for (index, (alias, path)) in config.aliases.iter().enumerate() {
                println!(
                    "{}: {}",
                    Colour::White.bold().paint(format!("{}.", index + 1)),
                    Colour::White.paint(format!("{} -> {}", alias, path))
                );
            }
        }
        "delete" => {
            if args.len() != 3 {
                eprintln!("Usage: cfgedit delete <n>");
                return;
            }
            let index: usize = match args[2].parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Invalid index: {}", args[2]);
                    return;
                }
            };

            if let Some((alias, path)) = config.aliases.iter().nth(index - 1).map(|(k, v)| (k.clone(), v.clone())) {
                println!("Are you sure you want to delete '{}' ({})? (y/n)", alias, path);
                let mut confirmation = String::new();
                io::stdin().read_line(&mut confirmation).expect("Failed to read input");
                if confirmation.trim().eq_ignore_ascii_case("y") {
                    config.aliases.remove(&alias);
                    config.save(&config_path);
                    println!("Alias '{}' deleted.", alias);
                } else {
                    println!("Deletion cancelled.");
                }
            } else {
                eprintln!("No alias found at index {}.", index);
            }
        }
        _ => {
            let alias = &args[1];
            if let Some(path) = config.aliases.get(alias) {
                Command::new("micro")
                    .arg(path)
                    .status()
                    .expect("Failed to open file with micro");
            } else {
                eprintln!("Unknown command or alias: {}", alias);
            }
        }
    }
}
