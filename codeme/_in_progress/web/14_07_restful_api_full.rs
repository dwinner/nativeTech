// 14_07_restful_api_full.rs
/*
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1"
*/

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post}, // handlers are imported below
    Router,
};
use std::net::SocketAddr;
use std::sync::{atomic::AtomicU32, Arc, Mutex};
use tokio::net::TcpListener;
use tokio::signal;

// --- Application State ---
#[derive(Clone)]
pub struct AppState {
    db: Arc<Mutex<Vec<models::Todo>>>,
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

// --- Module: Models ---
// (Corresponds to src/models.rs)
pub mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
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
// (Corresponds to src/handlers.rs)
pub mod handlers {
    use super::AppState;
    use crate::models::{NewTodo, Todo, UpdateTodo};
    use axum::{
        extract::{Path, State, Json as AxJson},
        http::StatusCode,
        response::Json,
    };
    use std::sync::atomic::Ordering;

    // --- Create ---
    pub async fn create_todo(
        State(state): State<AppState>,
        AxJson(payload): AxJson<NewTodo>,
    ) -> (StatusCode, Json<Todo>) {
        let mut db = state.db.lock().expect("Mutex was poisoned");
        // Generate ID and cast to i32
        let id_u32 = state.next_id.fetch_add(1, Ordering::SeqCst);
        
        let new_todo = Todo {
            id: id_u32 as i32,
            title: payload.title,
            completed: false,
        };
        
        db.push(new_todo.clone());
        (StatusCode::CREATED, Json(new_todo))
    }

    // --- Read All ---
    pub async fn get_all_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
        let todos = state.db.lock().expect("Mutex was poisoned").clone();
        Json(todos)
    }

    // --- Read Single ---
    pub async fn get_todo(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<Todo>, StatusCode> {
        let db = state.db.lock().expect("Mutex was poisoned");

        if let Some(todo) = db.iter().find(|t| t.id == id) {
            Ok(Json(todo.clone()))
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    }

    // --- Update ---
    pub async fn update_todo(
        State(state): State<AppState>,
        Path(id): Path<i32>,
        AxJson(payload): AxJson<UpdateTodo>,
    ) -> Result<Json<Todo>, StatusCode> {
        let mut db = state.db.lock().expect("Mutex was poisoned");

        if let Some(todo) = db.iter_mut().find(|t| t.id == id) {
            if let Some(title) = payload.title {
                todo.title = title;
            }
            if let Some(completed) = payload.completed {
                todo.completed = completed;
            }
            Ok(Json(todo.clone()))
        } else {
            Err(StatusCode::NOT_FOUND)
        }
    }

    // --- Delete ---
    pub async fn delete_todo(
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> StatusCode {
        let mut db = state.db.lock().expect("Mutex was poisoned");
        let len_before = db.len();

        db.retain(|todo| todo.id != id);

        if db.len() < len_before {
            StatusCode::NO_CONTENT
        } else {
            StatusCode::NOT_FOUND
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
    let app_state = AppState::new();

    // Imports from our handlers module
    use handlers::{create_todo, delete_todo, get_all_todos, get_todo, update_todo};

    let app = Router::new()
        .route("/health", get(health_check))
        // Root resource CRUD
        .route("/todos", 
            get(get_all_todos)
            .post(create_todo)
        )
        // Individual resource CRUD
        // Note: Using :id for the dynamic segment
        .route("/todos/:id", 
            get(get_todo)
            .patch(update_todo)
            .delete(delete_todo)
        )
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Server listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.expect("Failed to bind to address");

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

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("Graceful shutdown initiated.");
}