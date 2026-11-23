/*
Add these dependencies in the Cargo.toml file
[dependencies]
axum = "0.8"   # Check crates.io for the latest
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
*/

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json}, // Json is now used for the response body
    routing::{get, post, patch, delete},
    Router,
};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU32, Ordering};
use tokio::net::TcpListener;
use tokio::signal; // Required for graceful shutdown

// --- Modules ---
pub mod models;
pub mod handlers;

// --- Imports from our modules ---
use models::Todo;
use handlers::{
    create_todo,
    get_all_todos,
    get_todo,
    update_todo,
    delete_todo
};

// --- Application State ---
#[derive(Clone)]
pub struct AppState {
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

// Handler for the health check endpoint
async fn health_check() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Service is healthy")
}

// --- Main Server Setup ---
#[tokio::main]
async fn main() {
    // Initialize our shared state
    let app_state = AppState::new();

    // Build our application router, registering all CRUD handlers
    let app = Router::new()
        .route("/health", get(health_check)) // Health check endpoint
        
        // Root resource CRUD methods
        .route("/todos",
            get(get_all_todos) // GET /todos
            .post(create_todo) // POST /todos
        )
        
        // Individual resource CRUD methods using Axum 0.8 syntax: {id}
        .route("/todos/{id}",
            get(get_todo)        // GET /todos/{id}
            .patch(update_todo)  // PATCH /todos/{id}
            .delete(delete_todo) // DELETE /todos/{id}
        )
        
        // Share the AppState with all handlers
        .with_state(app_state);

    // --- Bind and Serve ---
    // Use expect() for safer startup error handling
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Server listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.expect("Failed to bind to address");
    
    // Run the server and enable graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Server failed to run");
}

// --- Graceful Shutdown Handler ---
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    // Use a pending future for non-Unix systems (like Windows)
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>(); 

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Graceful shutdown initiated.");
}