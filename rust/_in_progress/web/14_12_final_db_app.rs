// 14_12_final_db_app.rs
/*
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1"
dotenvy = "0.15"
# sqlx needs "runtime-tokio", "tls-rustls", "postgres", and "macros"
sqlx = { version = "0.8", features = ["runtime-tokio", "tls-rustls", "postgres", "macros"] }
*/

use axum::{
    extract::State,
    routing::get,
    Router,
};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;

// --- Application State ---
#[derive(Clone)]
pub struct AppState {
    db_pool: PgPool,
}

// --- Module: Models ---
// (Reflects src/models.rs)
pub mod models {
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;

    #[derive(Debug, Serialize, Clone, FromRow)]
    pub struct Todo {
        pub id: i32,
        pub title: String,
        pub completed: bool,
    }

    #[derive(Debug, Deserialize)]
    pub struct NewTodo {
        pub title: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct UpdateTodo {
        pub title: Option<String>,
        pub completed: Option<bool>,
    }
}

// --- Module: Handlers ---
// (Reflects src/handlers.rs)
pub mod handlers {
    use super::AppState;
    use crate::models::{NewTodo, Todo, UpdateTodo};
    use axum::{
        extract::{Path, State, Json as AxJson},
        http::StatusCode,
        response::{IntoResponse, Json},
    };

    fn internal_db_error(e: sqlx::Error) -> (StatusCode, String) {
        eprintln!("Database error: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
    }

    // CREATE
    pub async fn create_todo_db(
        State(state): State<AppState>,
        AxJson(payload): AxJson<NewTodo>,
    ) -> impl IntoResponse {
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

    // READ ALL
    pub async fn get_all_todos_db(State(state): State<AppState>) -> impl IntoResponse {
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

    // READ ONE
    pub async fn get_todo_db(
        State(state): State<AppState>,
        Path(todo_id): Path<i32>,
    ) -> impl IntoResponse {
        let result = sqlx::query_as!(
            Todo,
            "SELECT id, title, completed FROM todos WHERE id = $1",
            todo_id
        )
        .fetch_optional(&state.db_pool)
        .await;

        match result {
            Ok(Some(todo)) => (StatusCode::OK, Json(todo)).into_response(),
            Ok(None) => (StatusCode::NOT_FOUND, "Todo not found".to_string()).into_response(),
            Err(e) => internal_db_error(e).into_response(),
        }
    }

    // UPDATE
    pub async fn update_todo_db(
        State(state): State<AppState>,
        Path(todo_id): Path<i32>,
        AxJson(payload): AxJson<UpdateTodo>,
    ) -> impl IntoResponse {
        // 1. Fetch existing
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

        // 2. Prepare updates
        let new_title = payload.title.unwrap_or(existing_todo.title);
        let new_completed = payload.completed.unwrap_or(existing_todo.completed);

        // 3. Execute update
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

    // DELETE
    pub async fn delete_todo_db(
        State(state): State<AppState>,
        Path(todo_id): Path<i32>,
    ) -> impl IntoResponse {
        let result = sqlx::query!("DELETE FROM todos WHERE id = $1", todo_id)
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
}

// --- Main Function ---
#[tokio::main]
async fn main() {
    // 1. Setup Environment
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // 2. Create Connection Pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool");
    println!("🚀 Database connection pool initialized.");

    // 3. Create Table (Migration)
    // Running this on startup ensures the DB structure exists
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id SERIAL PRIMARY KEY,
            title VARCHAR NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT FALSE
        );
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to create 'todos' table");
    println!("🚀 'todos' table is ready.");

    // 4. Initialize State
    let app_state = AppState { db_pool: pool };

    // 5. Build Router
    use handlers::{
        create_todo_db, delete_todo_db, get_all_todos_db, get_todo_db, update_todo_db,
    };

    let app = Router::new()
        .route("/todos", 
            get(get_all_todos_db)
            .post(create_todo_db)
        )
        // Use :id for Axum dynamic path syntax
        .route("/todos/:id", 
            get(get_todo_db)
            .patch(update_todo_db)
            .delete(delete_todo_db)
        )
        .with_state(app_state);

    // 6. Bind and Serve
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Server listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}