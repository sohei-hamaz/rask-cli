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
    }

    Ok(())
}
