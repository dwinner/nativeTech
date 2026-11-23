// 02_thread_result.rs
use std::thread;
use std::time::Duration;

fn main() {
    println!("Main: Spawning a worker thread that will succeed...");
    
    let worker_handle = thread::spawn(|| {
        println!("Worker (Success): Starting computation...");
        thread::sleep(Duration::from_secs(1)); // Simulate work
        println!("Worker (Success): Computation finished.");
        42 // This is the return value
    });

    println!("Main: Spawning a worker thread that will panic...");
    
    let panicking_handle = thread::spawn(|| {
        println!("Worker (Panic): I'm about to panic!");
        panic!("The worker thread has panicked!");
    });

    // --- Wait for the successful worker ---
    println!("Main: Waiting for the successful worker...");
    match worker_handle.join() {
        Ok(result_from_worker) => {
            println!("Main: Successful worker joined and returned: {}", result_from_worker);
        }
        Err(e) => {
            // This case won't be hit for worker_handle
            eprintln!("Main: Successful worker panicked (unexpected!): {:?}", e);
        }
    }

    // --- Wait for the panicking worker ---
    println!("\nMain: Waiting for the panicking worker...");
    match panicking_handle.join() {
        Ok(_) => {
            // This case won't be hit for panicking_handle
            println!("Main: Panicking worker... returned Ok? (unexpected!)");
        }
        Err(e) => {
            // This is the expected outcome.
            // The 'e' here is an `Any + Send + 'static` object representing the panic.
            eprintln!("Main: Caught panic from worker thread as expected!");
            // We can't print the panic message directly in a simple way,
            // but we've confirmed it was an `Err`.
            // Note: In real apps, you might downcast 'e' to a String or &str to log the message.
        }
    }

    println!("\nMain: Program finished gracefully, even after a worker panic.");
}