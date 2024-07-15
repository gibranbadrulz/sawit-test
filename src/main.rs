mod args;
mod elastic;
mod log_entry;

use crate::args::Args;
use crate::elastic::send_to_elasticsearch;
use crate::log_entry::parse_log_line;
use chrono::Local;
use clap::Parser;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use reqwest::Client;
use std::sync::Arc;
use std::{env, error::Error};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

static CLIENT: Lazy<Client> = Lazy::new(Client::new);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // parse command-line arguments
    let args = Args::parse();
    let path = &args.log_file;

    // open the log file asynchronously
    let file = File::open(path).await?;
    let reader = BufReader::new(file);

    let elastic_url = env::var("ELASTIC_URL")?;
    let elastic_user = env::var("ELASTIC_USER").ok();
    let elastic_password = env::var("ELASTIC_PASSWORD").ok();

    let elastic_url_arc = Arc::new(elastic_url);
    let elastic_user_arc = Arc::new(elastic_user);
    let elastic_password_arc = Arc::new(elastic_password);

    let timestamp = Local::now().to_rfc3339();

    // read each line from the log file asynchronously
    let mut lines = reader.lines();
    let mut futures = vec![];
    while let Some(line_result) = lines.next_line().await? {
        let parsed_log = parse_log_line(&line_result, &timestamp);
        match parsed_log {
            Some(entry) => {
                let json_entry = serde_json::to_string(&entry)?;

                let elastic_url = elastic_url_arc.clone();
                let elastic_user = elastic_user_arc.clone();
                let elastic_password = elastic_password_arc.clone();

                // create a future to send each log entry to Elasticsearch
                let fut = send_to_elasticsearch(
                    CLIENT.clone(),
                    elastic_url,
                    elastic_user,
                    elastic_password,
                    json_entry.clone(), // clone json_entry since it's moved into the async block
                );
                futures.push(tokio::spawn(async move {
                    match fut.await {
                        Ok(_) => println!("Successfully sent to Elasticsearch: {}", json_entry),
                        Err(e) => eprintln!("Failed to send to Elasticsearch: {}", e),
                    }
                }));
            }
            None => println!("[{}] Failed to parse line: {}", timestamp, line_result),
        }
    }

    for fut in futures {
        fut.await?;
    }

    Ok(())
}
