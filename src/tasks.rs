use reqwest::blocking::Client;
use anyhow::{Context, Result};

pub fn list_tasks(client: &Client, api_token: &str) -> Result<String> {
    let response = client
        .get("https://rask.nomlab.org/tasks.json")
        .query(&[("api_token", api_token)])
        .send()
        .with_context(|| "could not fetch tasks")?;
    let body = response
        .text()
        .with_context(|| "could not read response body")?;
    println!("{}", body);
    Ok(body)
}
