// 17_18_data_races.rs
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // --- The Pitfall: Data Races ---
    // Safe Rust prevents data races at compile time.
    // The code below attempts to mutate `data` from a thread without synchronization.
    // It will NOT compile because `Vec` is not thread-safe for shared mutation.

    /*
    let mut data = vec![1, 2, 3]; 

    thread::spawn(|| {
        // Error: Closure requires unique access to `data`, but it's not safe to send 
        // a mutable reference across threads without synchronization.
        data.push(4); 
    });

    data.push(5); // Concurrent mutation from main thread
    */

    // --- The Fix: Arc<Mutex<T>> ---
    // 1. Arc (Atomic Reference Counted): Allows multiple threads to own the data.
    // 2. Mutex (Mutual Exclusion): Ensures only one thread can modify the data at a time.

    let shared_data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let mut handles = vec![];

    for i in 0..2 { 
        // Clone the Arc pointer. This creates a new handle to the SAME data on the heap.
        let data_clone = Arc::clone(&shared_data); 

        let handle = thread::spawn(move || {
            // Lock the mutex. This blocks until the lock is available.
            // .unwrap() handles the case where a previous thread panicked while holding the lock.
            let mut data_guard = data_clone.lock().unwrap();

            // Safe mutation inside the lock
            data_guard.push(i + 4);
            println!("Thread {} modified data to: {:?}", i, *data_guard);
            
            // The lock is automatically released when `data_guard` goes out of scope here.
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Read final state (requires locking again)
    println!("Final data: {:?}", shared_data.lock().unwrap());
}