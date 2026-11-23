// 14_09_models_with_sqlx.rs
/*
[dependencies]
serde = { version = "1.0", features = ["derive"] }
# We need the "macros" feature to use #[derive(FromRow)]
sqlx = { version = "0.8", features = ["postgres", "macros"] }
*/

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a Todo item in the system.
/// We derive `FromRow` to allow sqlx to map database rows to this struct.
#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Todo {
    pub id: i32, // PostgreSQL SERIAL maps to i32
    pub title: String,
    pub completed: bool,
}

/// Represents the payload for creating a new Todo.
#[derive(Debug, Deserialize)]
pub struct NewTodo {
    pub title: String,
}

/// Represents the payload for updating an existing Todo.
#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
}