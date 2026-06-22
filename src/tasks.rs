use reqwest::blocking::Client;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
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
    pub creator: Person,
    pub assigner: Person,
    pub project: Option<Project>,
    pub tags: Vec<Tag>,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub screen_name: String,
}

pub fn list_tasks(client: &Client, rask_url: &str, api_token: &str) -> Result<Vec<Task>> {
    let url = format!(
        "{}/tasks.json",
        rask_url,
    );
    let response = client
        .get(url)
        .query(&[("api_token", api_token)])
        .send()
        .with_context(|| "タスクを取得できません．")?;
    let tasks: Vec<Task> = response
        .json()
        .with_context(|| "レスポンスを読み取れません．")?;
    Ok(tasks)
}

pub fn list_users(client: &Client, rask_url: &str, api_token: &str) -> Result<Vec<User>> {
    let url = format!(
        "{}/users.json",
        rask_url,
    );
    let response = client
        .get(url)
        .query(&[("api_token", api_token)])
        .send()
        .with_context(|| "ユーザを取得できません．")?;
    let users: Vec<User> = response
        .json()
        .with_context(|| "レスポンスを読み取れません．")?;
    Ok(users)
}
