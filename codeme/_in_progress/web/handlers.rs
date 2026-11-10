use axum::{
    extract::{Path, State, Json as AxJson},
    http::StatusCode,
    response::IntoResponse,
    Json, // The response type for all successful bodies
};
use std::sync::atomic::Ordering; 
use crate::models::{Todo, NewTodo, UpdateTodo}; 
use crate::AppState; // We assume AppState is defined in main.rs

// --- Create ---
/// Handler for POST /todos
/// Creates a new todo item.
pub async fn create_todo(
    State(state): State<AppState>,
    AxJson(payload): AxJson<NewTodo>, // The Json extractor (aliased for clarity)
) -> (StatusCode, Json<Todo>) {
    
    // Lock the mutex safely
    let mut db = state.db.lock().expect("Mutex was poisoned");
    
    // Generate u32 ID from counter and cast it to i32 for the Todo struct.
    let id_u32 = state.next_id.fetch_add(1, Ordering::SeqCst);

    let new_todo = Todo {
        id: id_u32 as i32, // FIX: Cast u32 to i32 to match Todo struct/PostgreSQL
        title: payload.title,
        completed: false,
    };

    db.push(new_todo.clone());
    
    // Return 201 Created and the new todo as JSON
    (StatusCode::CREATED, Json(new_todo))
}

// --- Read All ---
/// Handler for GET /todos
/// Returns a list of all todo items.
pub async fn get_all_todos(
    State(state): State<AppState>
) -> Json<Vec<Todo>> {
    
    // Lock the mutex for reading and clone the vector safely
    let todos = state.db.lock().expect("Mutex was poisoned").clone();
    
    Json(todos) // Return 200 OK with JSON body
}

// --- Read Single ---
/// Handler for GET /todos/{id} (Axum 0.8 Syntax)
/// Returns a single todo by its ID.
pub async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>, // FIX: Change extractor type to i32
) -> Result<Json<Todo>, StatusCode> {
    
    let db = state.db.lock().expect("Mutex was poisoned");

    // Find the todo by its ID (i32 == i32 comparison is now valid)
    if let Some(todo) = db.iter().find(|t| t.id == id) {
        Ok(Json(todo.clone())) // Return 200 OK with the todo
    } else {
        Err(StatusCode::NOT_FOUND) // Return 404 Not Found
    }
}

// --- Update (Partial) ---
/// Handler for PATCH /todos/{id} (Axum 0.8 Syntax)
/// Updates a todo item (partial updates).
pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>, // FIX: Change extractor type to i32
    AxJson(payload): AxJson<UpdateTodo>, // Use the UpdateTodo struct
) -> Result<Json<Todo>, StatusCode> {
    
    let mut db = state.db.lock().expect("Mutex was poisoned");

    // Find a mutable reference to the todo
    if let Some(todo) = db.iter_mut().find(|t| t.id == id) {
        // Update fields if they are provided in the JSON payload
        if let Some(title) = payload.title {
            todo.title = title;
        }
        if let Some(completed) = payload.completed {
            todo.completed = completed;
        }
        Ok(Json(todo.clone())) // Return 200 OK with the updated todo
    } else {
        Err(StatusCode::NOT_FOUND) // 404 Not Found
    }
}

// --- Delete ---
/// Handler for DELETE /todos/{id} (Axum 0.8 Syntax)
/// Deletes a todo item by its ID.
pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>, // FIX: Change extractor type to i32
) -> StatusCode {
    
    let mut db = state.db.lock().expect("Mutex was poisoned");
    
    let len_before = db.len();
    // Keep all todos *except* the one with the matching ID
    db.retain(|todo| todo.id != id);
    let len_after = db.len();

    if len_before > len_after {
        // We removed an item
        StatusCode::NO_CONTENT // 204 No Content (success, no body)
    } else {
        // No item was removed, so it wasn't found
        StatusCode::NOT_FOUND // 404 Not Found
    }
}