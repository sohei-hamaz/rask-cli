use serde::{Serialize, Deserialize};
use reqwest::blocking::Client;
use anyhow::{Context, Result};

#[derive(Serialize, Debug)]
pub struct TaskRequest {
    pub assigner_id:   i32,
    pub content:       String,
    pub due_at:        Option<String>,
    pub description:   Option<String>,
    pub project_id:    Option<i32>,
    pub task_state_id: i32,
    pub tags: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct UserResponse {
    pub id:      i32,
    pub name:    String,
}

#[derive(Deserialize, Debug)]
pub struct ProjectResponse {
    pub id:      i32,
    pub name:    String,
}

#[derive(Deserialize, Debug)]
pub struct TagsResponse {
    pub id:      String,
    pub name:    String,
}

#[derive(Deserialize, Debug)]
pub struct TaskResponse {
    pub id:          i32,
    pub content:     String,
    pub description: Option<String>,
    pub due_at:      Option<String>,
    pub created_at:  String,
    pub updated_at:  String,
    pub creator:     UserResponse,
    pub assigner:    UserResponse,
    pub project:     Option<ProjectResponse>,
    pub tags:        Option<Vec<TagsResponse>>,
    pub url:         String,
}

pub fn list_tasks(client: &Client, api_token: &str) -> Result<String> {
    let response = client
        .get("http://localhost:3000/tasks.json")
        .query(&[("api_token", api_token)])
        .send()
        .with_context(|| "could not fetch tasks")?;
    let body = response
        .text()
        .with_context(|| "could not read response body")?;
    println!("{}", body);
    Ok(body)
}