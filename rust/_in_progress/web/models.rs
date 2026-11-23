use serde::{Deserialize, Serialize};


/// Represents a Todo item in the system.
#[derive(Debug, Serialize, Deserialize, Clone)]
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
/// All fields are optional to allow for partial updates.
#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
}