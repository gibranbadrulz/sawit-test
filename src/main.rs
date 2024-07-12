use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    log_file: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LogEntry {
    timestamp: String,
    service_name: String,
    status_code: u16,
    response_time_ms: String,
    user_id: String,
    transaction_id: String,
    additional_info: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.log_file);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parsed_log = parse_log_line(&line);
        match parsed_log {
            Some(entry) => {
                let json_entry = serde_json::to_string(&entry).unwrap();
                println!("{}", json_entry);
            }
            None => println!("Failed to parse line: {}", line),
        }
    }

    Ok(())
}

fn parse_log_line(line: &str) -> Option<LogEntry> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 7 {
        return None;
    }

    Some(LogEntry {
        timestamp: format!("{} {}", parts[0], parts[1]),
        service_name: parts[2].to_string(),
        status_code: parts[3].parse().ok()?,
        response_time_ms: parts[4].to_string(),
        user_id: parts[5].to_string(),
        transaction_id: parts[6].to_string(),
        additional_info: parts[7..].join(" "),
    })
}
