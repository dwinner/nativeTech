// 11_tokio_basics.rs
//
// IMPORTANT: To run this code, you must add the `tokio` crate to your Cargo.toml:
//
// [dependencies]
// tokio = { version = "1", features = ["full"] }

use tokio::time::{sleep, Duration};

/// This is an async function. When called, it returns a `Future`
/// that will resolve to a String.
async fn fetch_simulated_data(task_id: u32) -> String {
    println!("Task {}: Starting fetch...", task_id);
    
    // This is an async-aware sleep.
    // Unlike `std::thread::sleep`, this does NOT block the whole thread.
    // It yields control back to the tokio runtime,
    // allowing other async tasks to run.
    sleep(Duration::from_secs(1)).await;
    
    println!("Task {}: Finished fetch.", task_id);
    format!("Data from task {}", task_id)
}

/// This function contains our main async logic.
async fn process_tasks_sequentially() {
    println!("Starting sequential processing...");

    // We call and .await the first task.
    // Our function's execution pauses here (non-blockingly)
    // until `fetch_simulated_data(1)` completes.
    let data1 = fetch_simulated_data(1).await;
    println!("Main: Received first data: '{}'", data1);

    // Only *after* the first task is complete, we call and .await the second.
    let data2 = fetch_simulated_data(2).await;
    println!("Main: Received second data: '{}'", data2);

    println!("Sequential processing finished.");
}

/// The #[tokio::main] macro automatically:
/// 1. Creates a new Tokio runtime instance.
/// 2. Runs the `async fn main` on that runtime.
#[tokio::main]
async fn main() {
    // We .await the future returned by our main logic function.
    process_tasks_sequentially().await;
}