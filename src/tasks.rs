use reqwest::blocking::Client;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
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

pub fn list_tasks(client: &Client, rask_url: &str, api_token: &str) -> Result<Vec<Task>> {
    let response = client
        .get(rask_url)
        .query(&[("api_token", api_token)])
        .send()
        .with_context(|| "タスクを取得できません．")?;
    let tasks: Vec<Task> = response
        .json()
        .with_context(|| "レスポンスを読み取れません．")?;
    Ok(tasks)
}
