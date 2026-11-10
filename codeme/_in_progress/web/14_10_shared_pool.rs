// 14_10_shared_pool.rs
/*
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
dotenvy = "0.15"
# Features needed: runtime-tokio (for async), tls-rustls, postgres (db), migrate (for migrations)
sqlx = { version = "0.8", features = ["runtime-tokio", "tls-rustls", "postgres", "migrate"] }
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

// --- Modules ---
// (Placeholders for where your actual code would go)
pub mod models {}
pub mod handlers {} 

// --- Application State ---
// We derive Clone so the state (which contains the pool) can be cheapily 
// cloned and shared across threads/handlers. 
// PgPool is internally an Arc, so cloning it is efficient.
#[derive(Clone)]
pub struct AppState {
    db_pool: PgPool,
}

#[tokio::main]
async fn main() {
    // 1. Load environment variables from .env file
    // Ensure .env contains: DATABASE_URL=postgres://user:pass@localhost:5432/mydb
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // 2. Create the Database Connection Pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool");

    println!("🚀 Database connection pool initialized.");

    // 3. Run SQLx migrations (Optional / Requires 'migrations' folder)
    // sqlx::migrate!()
    //     .run(&pool)
    //     .await
    //     .expect("Failed to run database migrations.");
    // println!("🚀 Database migrations ran successfully.");

    // 4. Create the AppState
    let app_state = AppState { db_pool: pool };

    // 5. Build the Router
    let app = Router::new()
        .route("/", get(|| async { "Hello, World! (from database-backed server)" }))
        // .route("/todos", ...) // We will add these handlers later
        .with_state(app_state);

    // 6. Bind and Serve
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("🚀 Server listening on http://{}", addr);

    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    // In Axum 0.8, we pass the app router directly to serve
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}