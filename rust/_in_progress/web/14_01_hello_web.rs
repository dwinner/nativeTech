// 14_01_hello_web.rs
/*
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
*/

use axum::{routing::get, Router};
use tokio::net::TcpListener;

async fn hello_world_handler() -> &'static str {
    "Hello, Web!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world_handler));

    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .expect("Failed to bind to address");

    println!("🚀 Server listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}