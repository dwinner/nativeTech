// 14_08_database_pool.rs
/*
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
# specific features needed for Postgres + Tokio
sqlx = { version = "0.8", features = ["runtime-tokio", "tls-rustls", "postgres"] }
dotenvy = "0.15"
*/

use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;

// We wrap the pool in our AppState
// PgPool is internally an Arc, so it's cheap to clone
#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

async fn health_check() -> &'static str {
    "Database connected!"
}

#[tokio::main]
async fn main() {
    // 1. Load environment variables from .env file
    // Ensure you have a file named `.env` with: DATABASE_URL=postgres://user:pass@localhost:5432/mydb
    dotenvy::dotenv().ok();

    // 2. Read the connection string
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    // 3. Create the connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database connection pool");

    println!("🚀 Database connection pool initialized.");

    // 4. Initialize State
    let state = AppState { pool };

    // 5. Build Router
    let app = Router::new()
        .route("/", get(health_check))
        .with_state(state);

    // 6. Run Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    println!("🚀 Server listening on http://{}", addr);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}