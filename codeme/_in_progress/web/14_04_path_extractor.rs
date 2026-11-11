// 14_04_path_extractor.rs
/*
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
*/

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{atomic::{AtomicU32, Ordering}, Arc, Mutex};
use tokio::net::TcpListener;

// --- Data Models ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub text: String,
    pub completed: bool,
}

// --- App State ---
#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Vec<Todo>>>,
    next_id: Arc<AtomicU32>,
}

impl AppState {
    fn new() -> Self {
        Self {
            db: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(AtomicU32::new(1)),
        }
    }
}

// --- Handler: Get Todo by ID ---
async fn get_todo_by_id(
    State(app_state): State<AppState>, // Access shared state
    Path(id): Path<u32>,               // Extract 'id' from the URL path
) -> Result<Json<Todo>, StatusCode> {
    let db = app_state.db.lock().expect("Mutex was poisoned");

    // Search for the todo with the matching ID
    if let Some(todo) = db.iter().find(|t| t.id == id) {
        Ok(Json(todo.clone())) // Return 200 OK
    } else {
        Err(StatusCode::NOT_FOUND) // Return 404 Not Found
    }
}

#[tokio::main]
async fn main() {
    // Initialize state and seed it with one item for testing
    let app_state = AppState::new();
    {
        let mut db = app_state.db.lock().unwrap();
        db.push(Todo {
            id: 1,
            text: "Buy milk".to_string(),
            completed: false,
        });
    }

    // Define the router
    // Note: We use ":id" to define the path parameter
    let app = Router::new()
        .route("/todos/:id", get(get_todo_by_id))
        .with_state(app_state);

    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind");

    println!("🚀 Server listening on http://{}", listener.local_addr().unwrap());
    println!("   Try visiting: http://127.0.0.1:8080/todos/1");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}