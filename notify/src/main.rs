use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::path::PathBuf;
use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;

fn get_reminders_file_path() -> PathBuf {
    let home = env::var("HOME").expect("Failed to get HOME directory");
    let config_dir = Path::new(&home).join(".config").join("rust-utils");
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
    config_dir.join("reminders.json")
}

#[derive(Serialize, Deserialize, Debug)]
struct Reminder {
    title: String,
    urgency: String,
}

fn load_reminders() -> Vec<Reminder> {
    let path = get_reminders_file_path();
    if path.exists() {
        let data = fs::read_to_string(&path)
            .unwrap_or_else(|_| "[]".to_string());
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn save_reminders(reminders: &[Reminder]) {
    let path = get_reminders_file_path();
    let data = serde_json::to_string_pretty(reminders).expect("Failed to serialize reminders");
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("Failed to open reminders file");
    file.write_all(data.as_bytes()).expect("Failed to write reminders");
}

fn notify_reminders(reminders: &[Reminder]) {
    for reminder in reminders {
        let urgency = match reminder.urgency.as_str() {
            "low" => "low",
            "med" => "normal",
            "high" => "critical",
            _ => "normal",
        };
        let _ = std::process::Command::new("dunstify")
            .arg(&reminder.title)
            .arg(format!("--urgency={}", urgency))
            .spawn();
    }
}

fn create_reminder(title: &str, urgency: &str) {
    let mut reminders = load_reminders();
    reminders.push(Reminder {
        title: title.to_string(),
        urgency: urgency.to_string(),
    });
    save_reminders(&reminders);
    println!("Reminder created successfully!");
}

fn view_reminders() {
    let reminders = load_reminders();
    if reminders.is_empty() {
        println!("No reminders found.");
    } else {
        for (index, reminder) in reminders.iter().enumerate() {
            let urgency_color = match reminder.urgency.as_str() {
                "low" => "\x1b[32m", // Green
                "med" => "\x1b[33m", // Yellow
                "high" => "\x1b[31m", // Red
                _ => "\x1b[0m",      // Default
            };
            println!(
                "\x1b[1m{}. {}\x1b[0m [{}{}{}\x1b[0m]",
                index + 1,
                reminder.title,
                urgency_color,
                reminder.urgency,
                "\x1b[0m"
            );
        }
    }
}

fn delete_reminder(index: usize) {
    let mut reminders = load_reminders();
    if index == 0 || index > reminders.len() {
        println!("Invalid reminder index.");
        return;
    }

    let reminder = &reminders[index - 1];
    println!(
        "Are you sure you want to delete \x1b[1m{}\x1b[0m? (y/n)",
        reminder.title
    );
    let mut confirmation = String::new();
    io::stdin()
        .read_line(&mut confirmation)
        .expect("Failed to read input");

    if confirmation.trim().eq_ignore_ascii_case("y") {
        reminders.remove(index - 1);
        save_reminders(&reminders);
        println!("Reminder deleted successfully.");
    } else {
        println!("Delete action cancelled.");
    }
}

fn main() {
    let matches = Command::new("Reminder CLI")
        .version("1.0")
        .author("beanfrog")
        .about("Manages reminders")
        .subcommand(Command::new("notify").about("Send all reminders as notifications"))
        .subcommand(
            Command::new("add")
                .about("Create a new reminder")
                .arg(Arg::new("title").required(true).help("Title of the reminder"))
                .arg(Arg::new("urgency").required(true).help("Urgency of the reminder (low/med/high)")),
        )
        .subcommand(Command::new("view").about("View all reminders"))
        .subcommand(
            Command::new("delete")
                .about("Delete a reminder")
                .arg(Arg::new("index").required(true).help("Index of the reminder to delete")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("notify", _)) => notify_reminders(&load_reminders()),
        Some(("add", sub_m)) => {
            let title = sub_m.get_one::<String>("title").unwrap();
            let urgency = sub_m.get_one::<String>("urgency").unwrap();
            create_reminder(title, urgency);
        }
        Some(("view", _)) => view_reminders(),
        Some(("delete", sub_m)) => {
            let index = sub_m.get_one::<String>("index").unwrap().parse::<usize>().unwrap_or(0);
            delete_reminder(index);
        }
        _ => println!("Invalid command. Use --help for usage."),
    }
}
