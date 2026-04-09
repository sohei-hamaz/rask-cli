use anyhow::{Context, Result}; // Error handling library for better error messages
use clap::Parser; // CLI argument parser
use reqwest::blocking::Client; // HTTP client for making requests
use dotenvy::dotenv;
use std::env; // For accessing environment variables
mod tasks;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    dotenv().with_context(|| ".env file not found")?;
    let rask_api_token = env::var("RASK_API_TOKEN")
        .with_context(|| "RASK_API_TOKEN environment variable not set")?;

    let client = Client::new();

    let pattern = args.pattern.join(" ");
    
    if pattern == "task list" {
        tasks::list_tasks(&client, &rask_api_token)?;
        // let response = client
        //     .get("http://localhost:3000/tasks.json") // 全員のタスクを表示するにはクエリを追加する必要あり
        //     .query(&[("api_token", rask_api_token)]) 
        //     .send()
        //     .with_context(|| format!("could not fetch URL `{}`", "http://localhost:3000/tasks.json"))?;

        // // println!("Status code: {}", response.status());

        // let body = response
        //     .text()
        //     .with_context(|| "could not read response body")?;
        // println!("{}", body);
    }

    Ok(())
}
