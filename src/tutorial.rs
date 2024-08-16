// https://youtu.be/ogpE4hviXyA?si=kx9q8CZCNY2P9TER

use serde::{Deserialize, Serialize};
use serde_json::{json, Value}; 

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename="userId")]
    user_id: i32,
    id: Option<i32>,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // let todos: Vec<Todo> = reqwest::Client::new()
    //     .get("https://jsonplaceholder.typicode.com/todos?userId=1")
    //     .send()
    //     .await?
    //     .json()
    //     .await?;

    // println!("{:#?}", todos);

    // let new_todo = Todo {
    //     user_id: 1,
    //     id: None, // will be assigned automatically
    //     title: "Learning JSON APIs with Rust".to_owned(),
    //     completed: false,
    // };

    let new_todo: Value = reqwest::Client::new()
        .post("https://jsonplaceholder.typicode.com/todos")
        .json(&json!({
            "userId": 1,
            "title": "Learning JSON APIs with Rust",
            "completed": false
        }))
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", new_todo);

    Ok(())
}