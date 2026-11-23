// 14_11_db_handlers.rs
// Replace the contents of src/handlers.rs with this code.

use axum::{
    extract::{Path, State, Json as AxJson},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use crate::models::{Todo, NewTodo, UpdateTodo};
use crate::AppState; // This now contains the PgPool
use sqlx; 

// Helper function to convert a generic sqlx error into a 500 Internal Server Error
fn internal_db_error(e: sqlx::Error) -> (StatusCode, String) {
    eprintln!("Database error: {:?}", e);
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
}

// --- Create ---
// Handler for POST /todos
pub async fn create_todo_db(
    State(state): State<AppState>,
    AxJson(payload): AxJson<NewTodo>,
) -> impl IntoResponse {
    // `sqlx::query_as!` provides compile-time checking of your SQL
    let result = sqlx::query_as!(
        Todo,
        "INSERT INTO todos (title) VALUES ($1) RETURNING *",
        payload.title
    )
    .fetch_one(&state.db_pool)
    .await;

    match result {
        Ok(new_todo) => (StatusCode::CREATED, Json(new_todo)).into_response(),
        Err(e) => internal_db_error(e).into_response(),
    }
}

// --- Read All ---
// Handler for GET /todos
pub async fn get_all_todos_db(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Todo,
        "SELECT id, title, completed FROM todos ORDER BY id"
    )
    .fetch_all(&state.db_pool)
    .await;

    match result {
        Ok(all_todos) => (StatusCode::OK, Json(all_todos)).into_response(),
        Err(e) => internal_db_error(e).into_response(),
    }
}

// --- Read Single ---
// Handler for GET /todos/{id}
pub async fn get_todo_db(
    State(state): State<AppState>,
    Path(todo_id): Path<i32>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Todo,
        "SELECT id, title, completed FROM todos WHERE id = $1",
        todo_id
    )
    .fetch_optional(&state.db_pool) // Returns Option<Todo>
    .await;

    match result {
        Ok(Some(todo)) => (StatusCode::OK, Json(todo)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Todo not found".to_string()).into_response(),
        Err(e) => internal_db_error(e).into_response(),
    }
}

// --- Update (Partial) ---
// Handler for PATCH /todos/{id}
pub async fn update_todo_db(
    State(state): State<AppState>,
    Path(todo_id): Path<i32>,
    AxJson(payload): AxJson<UpdateTodo>,
) -> impl IntoResponse {
    // 1. Fetch the existing todo to ensure it exists and get current values
    let existing_todo: Todo = match sqlx::query_as!(
        Todo,
        "SELECT id, title, completed FROM todos WHERE id = $1",
        todo_id
    )
    .fetch_optional(&state.db_pool)
    .await
    {
        Ok(Some(todo)) => todo,
        Ok(None) => return (StatusCode::NOT_FOUND, "Todo not found".to_string()).into_response(),
        Err(e) => return internal_db_error(e).into_response(),
    };

    // 2. Overwrite only the fields provided in the payload
    let new_title = payload.title.unwrap_or(existing_todo.title);
    let new_completed = payload.completed.unwrap_or(existing_todo.completed);

    // 3. Update the database
    let result = sqlx::query_as!(
        Todo,
        "UPDATE todos SET title = $1, completed = $2 WHERE id = $3 RETURNING *",
        new_title,
        new_completed,
        todo_id
    )
    .fetch_one(&state.db_pool)
    .await;

    match result {
        Ok(updated_todo) => (StatusCode::OK, Json(updated_todo)).into_response(),
        Err(e) => internal_db_error(e).into_response(),
    }
}

// --- Delete ---
// Handler for DELETE /todos/{id}
pub async fn delete_todo_db(
    State(state): State<AppState>,
    Path(todo_id): Path<i32>,
) -> impl IntoResponse {
    let result = sqlx::query!(
        "DELETE FROM todos WHERE id = $1",
        todo_id
    )
    .execute(&state.db_pool)
    .await;

    match result {
        Ok(db_result) => {
            if db_result.rows_affected() > 0 {
                (StatusCode::NO_CONTENT, "").into_response()
            } else {
                (StatusCode::NOT_FOUND, "Todo not found".to_string()).into_response()
            }
        }
        Err(e) => internal_db_error(e).into_response(),
    }
}