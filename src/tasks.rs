use reqwest::blocking::Client;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Task {
    pub id: i32,
    pub content: String,
    pub description: String,
    pub due_at: String,
    pub created_at: String,
    pub updated_at: String,
    pub creator: User,
    pub assigner: User,
    pub project: Option<Project>,
    pub tags: Vec<Tag>,
    pub url: String,
}

pub fn list_tasks(client: &Client, api_token: &str) -> Result<Vec<Task>> {
    let response = client
        .get("https://rask.nomlab.org/tasks.json")
        .query(&[("api_token", api_token)])
        .send()
        .with_context(|| "タスクを取得できませんでした．")?;
    let tasks: Vec<Task> = response
        .json()
        .with_context(|| "could not read response body")?;
    Ok(tasks)
}
