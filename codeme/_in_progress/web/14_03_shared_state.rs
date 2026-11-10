// 14_03_shared_state.rs
/*
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
*/

use axum::{
    extract::State,
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{atomic::{AtomicU32, Ordering}, Arc, Mutex};
use tokio::net::TcpListener;

// --- Data Models (Inlined for this example) ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub text: String,
    pub completed: bool,
}

// --- 1. Define the AppState ---
// This struct will hold all shared state for our application.
// We use `Arc` to allow shared ownership across threads.
// We use `Mutex` for interior mutability for our in-memory `Vec`.
#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Vec<Todo>>>,
    next_id: Arc<AtomicU32>,
}

impl AppState {
    fn new() -> Self {
        Self {
            db: Arc::new(Mutex::new(Vec::new())), // Start with an empty Vec
            next_id: Arc::new(AtomicU32::new(1)), // Start IDs from 1
        }
    }
}

// A simple handler to show the state is working
async fn get_todos_placeholder(
    State(app_state): State<AppState>,
) -> Json<Vec<Todo>> {
    let todos = app_state.db.lock().unwrap();
    Json(todos.clone())
}

#[tokio::main]
async fn main() {
    // --- 2. Create and initialize our AppState ---
    let app_state = AppState::new();

    // --- 3. Define the Router and add the state ---
    let app = Router::new()
        .route("/todos", get(get_todos_placeholder))
        // `.with_state()` makes the `app_state` available to all routes
        .with_state(app_state);

    // --- 4. Bind and Serve ---
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind to address");

    println!("🚀 Server listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}