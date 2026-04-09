use anyhow::{Context, Result}; // Error handling library for better error messages
use clap::Parser; // CLI argument parser
use reqwest::blocking::Client; // HTTP client for making requests
use serde::{Serialize, Deserialize}; // For serializing data structures to JSON
use dotenvy::dotenv;
use std::env; // For accessing environment variables

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: Vec<String>,
}

#[derive(Serialize, Debug)]
struct TaskRequest {
    assigner_id:   i32,
    content:       String,
    due_at:        Option<String>,
    description:   Option<String>,
    project_id:    Option<i32>,
    task_state_id: i32,
    tags: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct UserResponse {
    id:      i32,
    name:    String,
}

#[derive(Deserialize, Debug)]
struct ProjectResponse {
    id:      i32,
    name:    String,
}

#[derive(Deserialize, Debug)]
struct TagsResponse {
    id:      String,
    name:    String,
}

#[derive(Deserialize, Debug)]
struct TaskResponse {
    id:          i32,
    content:     String,
    description: Option<String>,
    due_at:      Option<String>,
    created_at:  String,
    updated_at:  String,
    creator:     UserResponse,
    assigner:    UserResponse,
    project:     Option<ProjectResponse>,
    tags:        Option<Vec<TagsResponse>>,
    url:         String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    dotenv().with_context(|| ".env file not found")?;
    let rask_api_token = env::var("RASK_API_TOKEN")
        .with_context(|| "RASK_API_TOKEN environment variable not set")?;

    let client = Client::new();

    let pattern = args.pattern.join(" ");
    
    if pattern == "task list" {
        let response = client
            .get("http://localhost:3000/tasks.json") // 全員のタスクを表示するにはクエリを追加する必要あり
            .query(&[("api_token", rask_api_token)]) 
            .send()
            .with_context(|| format!("could not fetch URL `{}`", "http://localhost:3000/tasks.json"))?;

        // println!("Status code: {}", response.status());

        let body = response
            .text()
            .with_context(|| "could not read response body")?;
        println!("{}", body);

    // } else if pattern == "user list" {
    //     let response = client
    //         .get("http://localhost:3000/users.json")
    //         .query(&[("api_token", api_token)])
    //         .send()
    //         .with_context(|| format!("could not fetch URL `{}`", "http://localhost:3000/users.json"))?;

    //     println!("Status code: {}", response.status());

    //     let body = response
    //         .text()
    //         .with_context(|| "could not read response body")?;
    //     println!("user list: \n{}", body);
    // } else if pattern == "projects list" {
    //     let response = client
    //         .get("http://localhost:3000/projects.json")
    //         .query(&[("api_token", api_token)])
    //         .send()
    //         .with_context(|| format!("could not fetch URL `{}`", "http://localhost:3000/projects.json"))?;

    //     println!("Status code: {}", response.status());

    //     let body = response
    //         .text()
    //         .with_context(|| "could not read response body")?;
    //     println!("projects list: \n{}", body);
    // } else if pattern == "task create" {
    //     let task_request = TaskRequest {
    //             assigner_id: 1,
    //             content: "New task".into(),
    //             due_at: Some("2026-04-30".into()),
    //             description: Some("test".into()),
    //             project_id: None,
    //             task_state_id: 1,
    //             tags: None,
    //     };

    //     let response = client
    //         .post("http://localhost:3000/tasks.json")
    //         .query(&[("api_token", api_token)])
    //         .json(&task_request)
    //         .send()
    //         .with_context(|| format!("could not fetch URL `{}`", "http://localhost:3000/tasks.json"))?;

    //     println!("Status code: {}", response.status());

    //     let body = response
    //         .json::<TaskResponse>()
    //         .with_context(|| "could not read response body")?;
    //     println!("Task created: \n{:?}", body);
    // } else {
    //     println!("Searching for pattern: {}", pattern);
    }

    Ok(())
}
