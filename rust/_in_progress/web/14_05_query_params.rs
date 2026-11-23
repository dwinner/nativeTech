// 14_05_query_params.rs
/*
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
*/

use axum::{
    extract::{Query, State},
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{atomic::AtomicU32, Arc, Mutex};
use tokio::net::TcpListener;

// --- Data Models ---
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub text: String,
    pub completed: bool,
}

// --- Query Parameter Struct ---
// This struct defines what query parameters we accept.
// Deriving `Deserialize` is required for Axum to parse the URL string.
#[derive(Deserialize)]
pub struct SearchParams {
    pub completed: Option<bool>,
}

// --- App State ---
#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Vec<Todo>>>,
}

// --- Handler: Search Todos ---
async fn search_todos(
    State(app_state): State<AppState>,
    Query(params): Query<SearchParams>, // Extract query params into our struct
) -> Json<Vec<Todo>> {
    let db = app_state.db.lock().expect("Mutex was poisoned");

    // Clone the full list so we can filter it
    let mut results = db.clone();

    // If the `completed` query param was provided, filter the results
    if let Some(completed_status) = params.completed {
        results.retain(|todo| todo.completed == completed_status);
    }

    Json(results)
}

#[tokio::main]
async fn main() {
    // 1. Setup state and seed data
    let db = Arc::new(Mutex::new(vec![
        Todo { id: 1, text: "Buy milk".to_string(), completed: false },
        Todo { id: 2, text: "Walk the dog".to_string(), completed: true },
        Todo { id: 3, text: "Read Rust book".to_string(), completed: false },
    ]));
    let app_state = AppState { db };

    // 2. Define Router
    let app = Router::new()
        .route("/todos/search", get(search_todos))
        .with_state(app_state);

    // 3. Start Server
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind");

    println!("🚀 Server listening on http://{}", listener.local_addr().unwrap());
    println!("   Try: http://127.0.0.1:8080/todos/search?completed=true");
    println!("   Try: http://127.0.0.1:8080/todos/search?completed=false");
    println!("   Try: http://127.0.0.1:8080/todos/search"); // Returns all

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}