use clap::{Arg, Command};
use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::fs::{read, File, OpenOptions};
use std::io::{self, Write, BufRead};
use std::path::PathBuf;
use std::error::Error;
use chrono::Local;
#[derive(Debug, Deserialize, Serialize)]
struct Message {
    role:String,
    content: String
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message
}

#[derive(Debug, Deserialize)]
struct OAIRes {
    choices: Vec<Choice>
}

#[derive(Debug, Deserialize, Serialize)]
struct OAIReq {
    model: String,
    messages:Vec<Message>
}

fn write_file(filepath: &str, content: &str) -> io::Result<()> {
    let home_dir: String = env::var("HOME").expect("Could not find home directory. Make sure it's in your PATH");
    let mut path: PathBuf = PathBuf::from(home_dir);
    path.push(".config");
    path.push("rust-utils");
    path.push(filepath);
    let mut file: File = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)?;

    writeln!(file, "{}", content)?;

    Ok(())
}

fn read_file(filepath: &str) -> Result<String, Box<dyn Error>> {
    let home_dir: String = env::var("HOME")?;
    let mut path: PathBuf = PathBuf::from(home_dir);
    path.push(".config");
    path.push("rust-utils");
    path.push(filepath);
    let file: File = File::open(path)?;
    let mut reader: io::BufReader<File> = io::BufReader::new(file);
    let mut first_line: String = String::new();
    reader.read_line(&mut first_line)?;
    Ok(first_line.trim().to_string())
}

async fn prompt(input: String) -> Result<String, Box<dyn Error>> {
    let token: String = read_file("key")?;   
    let model: String = read_file("model")?; 

    print!("{}", input_summary(&[model.clone(), input.clone()]));

    let client: Client = Client::new();
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());

    let prompt: Message = Message {
        role: String::from("system"),
        content: String::from("You are a helpful assistant. Your responses will be displayed in a POSIX-compliant terminal. Please respond to the following: \n"),
    };

    let req: OAIReq = OAIReq {
        model: model.clone(), 
        messages: vec![
            prompt,
            Message {
                role: String::from("user"),
                content: input.clone(),
            },
        ],
    };

    let res: Result<reqwest::Response, reqwest::Error> = client
        .post("https://api.openai.com/v1/chat/completions")
        .headers(headers)
        .json(&req)
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let body: OAIRes = response.json().await?;
                if let Some(choice) = body.choices.last() {
                    Ok(choice.message.content.clone())
                } else {
                    Err("Invalid response body (missing choices key).".into())
                }
            } else {
                Err(format!("Non-200 status code received: {}", response.status()).into())
            }
        }
        Err(e) => Err(format!("Request error: {}", e).into()),
    }
}
fn input_summary(lines: &[String]) -> String {
    let now: chrono::DateTime<Local> = Local::now(); 
    let date_fmt = now.format("%d %b %Y - %I:%M %p").to_string();

    let model_fmt = format!("Model: \x1b[1m\x1b[37m{}\x1b[0m", &lines[0]);
    let prompt_fmt = format!("Input: \x1b[1m\x1b[37m{}\x1b[0m", &lines[1]);

    let mut output = String::new();
    output.push_str(&format!("┏━━═{}═━━┓\n", date_fmt));
    output.push_str(&format!("┣━ {}\n", model_fmt));
    output.push_str(&format!("┣━ {}\n", prompt_fmt));
    output.push_str("┗━━━━┛\n"); 

    output
}



// fn input_summary(lines: &[String]) -> String {
//     let max_width: usize = lines.iter()
//     .map(|s: &String| UnicodeWidthStr::width(strip_ansi(s).as_str()))
//     .max()
//     .unwrap_or(0);

// let mut output: String = String::new();
// output.push_str(&format!("┏{}┓\n", "━".repeat(max_width)));

// for line in lines {
//     let visible_width: usize = UnicodeWidthStr::width(strip_ansi(line).as_str());
//     let padding: usize = max_width - visible_width;
//     output.push_str(&format!("┃{}{}┃\n", line, " ".repeat(padding)));
// }

// output.push_str(&format!("┗{}┛", "━".repeat(max_width)));
// output
// }
#[tokio::main]
async fn main() {
    let matches: clap::ArgMatches = Command::new("chatgpt")
        .version("1.0")
        .author("beanfrog")
        .about("use chatgpt from the command line")
        .subcommand(
            Command::new("apikey")
                .about("add your OAI api key")
                .arg(Arg::new("key").required(true).help("your api key")),
        )
        .subcommand(
            Command::new("model")
                .about("set model name")
                .arg(Arg::new("modelname").required(true).help("name of the model")),
        )
        .subcommand(
            Command::new("verbose")
                .about("run a prompt, and show input/output summary")
                .arg(Arg::new("input").required(true).help("name of the model")),
        )
        .arg(Arg::new("input").index(1).help("Prompt input to send to ChatGPT")) // Default input
        .get_matches();

    if let Some(("apikey", args)) = matches.subcommand() {
        let key: &String = args.get_one::<String>("key").unwrap();
        let _ = write_file("key", key);
    } else if let Some(("model", args)) = matches.subcommand() {
        let model: &String = args.get_one::<String>("modelname").unwrap();
        let _ = write_file("model", model);
    } else if let Some(input) = matches.get_one::<String>("input") {
        match prompt(input.clone()).await {
            Ok(response_message) => print!("\r{}", response_message),
            Err(e) => eprintln!("Error: {}", e),
        }
    } else {
        println!("Invalid command. Use --help for usage information.");
    }
}