use anyhow::{Context, Result};
use clap::{Parser, Subcommand, Args};
use reqwest::blocking::Client;
use dotenvy::dotenv;
use std::env;
mod tasks;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Task(TaskCommand),
}

#[derive(Args)]
struct TaskCommand {
    #[command(subcommand)]
    action: TaskAction,
}

#[derive(Subcommand)]
enum TaskAction {
    List,
}

fn main() -> Result<()> {
    let _args = Cli::parse();

    dotenv().with_context(|| ".env ファイルが見つかりません．")?;
    let rask_api_token = env::var("RASK_API_TOKEN")
        .with_context(|| "RASK_API_TOKEN が設定されていません．")?;
    let rask_url = env::var("RASK_URL")
        .with_context(|| "RASK_URL が設定されていません．")?;

    let client = Client::new();

    let tasks = tasks::list_tasks(
        &client, 
        &rask_url,
        &rask_api_token,
    )?;
    println!("{}", serde_json::to_string(&tasks)?);

    Ok(())
}
