// 04_mutex_counter.rs
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Create a counter, protected by a Mutex, and wrapped in an Arc for sharing
    let counter = Arc::new(Mutex::new(0u32)); // Start with 0
    let mut handles = vec![];

    println!("Main: Initial counter value = {}", *counter.lock().unwrap());

    for i in 0..5 { // Spawn 5 threads
        let counter_clone_for_thread = Arc::clone(&counter); // Clone Arc for the thread
        
        let handle = thread::spawn(move || {
            // Each thread will try to increment the counter 10 times
            for _ in 0..10 {
                // Acquire the lock. This blocks if another thread has it.
                let mut num_guard = counter_clone_for_thread.lock().unwrap();
                
                // We now have exclusive mutable access to the u32 inside the Mutex.
                *num_guard += 1;
                // The lock is released automatically when num_guard goes out of scope here.
            }
            println!("Thread {}: Finished incrementing.", i);
        });
        handles.push(handle);
    }

    // Wait for all threads to complete their work
    for handle in handles {
        handle.join().unwrap();
    }

    // Lock the mutex in the main thread to read the final value
    let final_value = *counter.lock().unwrap();
    println!("Main: All threads finished. Final counter value = {}", final_value); // Expected: 50
}