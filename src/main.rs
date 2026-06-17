use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use dotenvy::dotenv;
use std::env;
mod tasks;

#[derive(Parser)]
struct Cli {
    pattern: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    dotenv().with_context(|| ".env ファイルが見つかりません．")?;
    let rask_api_token = env::var("RASK_API_TOKEN")
        .with_context(|| "RASK_API_TOKEN 環境変数が設定されていません．")?;

    let client = Client::new();
    
    let tasks = tasks::list_tasks(&client, &rask_api_token)?;

    for task in tasks {
        println!("{:?}", task);
    }

    Ok(())
}
