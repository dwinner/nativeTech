// 14_13_final_backend_with_cors.rs
/*
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1"
dotenvy = "0.15"
# sqlx features
sqlx = { version = "0.8", features = ["runtime-tokio", "tls-rustls", "postgres", "macros"] }
# Tower-Http for middleware, including CORS
tower-http = { version = "0.5", features = ["cors"] }
*/

use axum::{
    extract::State,
    routing::{get, post, patch, delete},
    Router,
};
use axum::http::Method;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use dotenvy::dotenv;

// --- Application State ---
#[derive(Clone)]
pub struct AppState {
    db_pool: PgPool,
}

// --- Module: Models (Data Structures) ---
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
use models::{Todo, NewTodo, UpdateTodo}; // Import structs for handlers

// --- Module: Handlers (DB Logic) ---
pub mod handlers {
    use super::AppState;
    use crate::models::{NewTodo, Todo, UpdateTodo};
    use axum::{
        extract::{Path, State, Json as AxJson},
        http::StatusCode,
        response::{IntoResponse, Json},
    };
    use sqlx;

    fn internal_db_error(e: sqlx::Error) -> (StatusCode, String) {
        eprintln!("Database error: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
    }

    // CREATE (POST)
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

    // READ ALL (GET)
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

    // READ ONE (GET /:id)
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

    // UPDATE (PATCH /:id)
    pub async fn update_todo_db(
        State(state): State<AppState>,
        Path(todo_id): Path<i32>,
        AxJson(payload): AxJson<UpdateTodo>,
    ) -> impl IntoResponse {
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

        let new_title = payload.title.unwrap_or(existing_todo.title);
        let new_completed = payload.completed.unwrap_or(existing_todo.completed);

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

    // DELETE (DELETE /:id)
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

// --- Main Server Setup ---
#[tokio::main]
async fn main() {
    // Import handlers for routing
    use handlers::{
        create_todo_db, delete_todo_db, get_all_todos_db, get_todo_db, update_todo_db,
    };
    
    // Load environment variables
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // 1. Create the Database Connection Pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool");
    println!("🚀 Database connection pool initialized.");

    // 2. Create the Schema (our 'todos' table)
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
    
    // 3. Create the AppState
    let app_state = AppState { db_pool: pool };

    // 4. Define CORS Policy
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers([axum::http::header::CONTENT_TYPE])
        .allow_origin(Any);
    
    // 5. Build our application router
    let app = Router::new()
        .route("/todos",
            get(get_all_todos_db)
            .post(create_todo_db)
        )
        // Note: Using :id for Axum 0.8 compatibility, 
        // despite the text using {id} which is often for non-Axum contexts.
        .route("/todos/:id", 
            get(get_todo_db)
            .patch(update_todo_db)
            .delete(delete_todo_db)
        )
        .with_state(app_state)
        .layer(cors); // Apply the CORS layer

    // 6. Bind and Serve
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Server (with CORS) listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    // Removed .into_make_service() for Axum 0.8 compatibility
    axum::serve(listener, app).await.unwrap();
}