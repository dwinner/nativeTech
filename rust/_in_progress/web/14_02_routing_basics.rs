// 14_02_routing_basics.rs
/*
[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
*/

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn root_handler() -> &'static str {
    "Welcome to our Axum server!"
}

async fn hello_handler() -> &'static str {
    "Hello, Web!"
}

#[tokio::main]
async fn main() {
    // Define routes for root and /hello
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/hello", get(hello_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 Server listening on http://{}", addr);

    axum::serve(listener, app)
        .await
        .unwrap();
}