// 14_06_json_extractor.rs
/*
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
*/

use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{atomic::{AtomicU32, Ordering}, Arc, Mutex};
use tokio::net::TcpListener;

// --- Data Models ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String, // Matching 'title' from your snippet
    pub completed: bool,
}

// The struct for the incoming JSON payload
#[derive(Deserialize)]
pub struct NewTodo {
    pub title: String,
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

// --- Handler: Create Todo ---
// This handler expects a JSON body matching `NewTodo`
async fn create_todo(
    State(app_state): State<AppState>,
    Json(payload): Json<NewTodo>, // Automatically parses the request body
) -> (StatusCode, Json<Todo>) {
    let mut db = app_state.db.lock().expect("Mutex was poisoned");
    
    // Generate a new ID
    let id = app_state.next_id.fetch_add(1, Ordering::SeqCst);

    let new_todo = Todo {
        id,
        title: payload.title,
        completed: false,
    };

    db.push(new_todo.clone());

    // Return 201 Created and the newly created item as JSON
    (StatusCode::CREATED, Json(new_todo))
}

#[tokio::main]
async fn main() {
    let app_state = AppState::new();

    let app = Router::new()
        .route("/todos", post(create_todo)) // Map POST requests to the handler
        .with_state(app_state);

    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind");

    println!("🚀 Server listening on http://{}", listener.local_addr().unwrap());
    println!("   Test with cURL:");
    println!(r#"   curl -X POST -H "Content-Type: application/json" -d '{{"title": "Buy eggs"}}' http://127.0.0.1:8080/todos"#);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}