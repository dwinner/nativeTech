use axum::routing::{get, patch, post};
use axum::Router;
use std::sync::Arc;
use task_futures_outro::{create_ticket, get_ticket, patch_ticket, AppState};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    /* TODO */
}
