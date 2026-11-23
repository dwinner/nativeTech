// 12_mutex_lock_duration.rs
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct SharedData {
    value: i32,
    // some other complex data
}

fn main() {
    let shared_data = Arc::new(Mutex::new(SharedData { value: 0 }));
    let mut handles = vec![];

    for i in 0..2 {
        let data_clone = Arc::clone(&shared_data);
        
        let handle = thread::spawn(move || {
            // --- BAD: Long operation inside lock ---
            // If we uncomment this block, threads will serialize their 1s sleeps,
            // taking total 2s instead of running concurrently.
            /*
            let mut data_guard = data_clone.lock().unwrap();
            data_guard.value += i + 1;
            println!("Thread {}: Updated value to {}", i, data_guard.value);
            thread::sleep(Duration::from_secs(1)); // Simulate long work WHILE HOLDING LOCK
            println!("Thread {}: Releasing lock after long work.", i);
            // Drop(data_guard) happens here
            */

            // --- GOOD: Prepare data, then short lock ---
            let value_to_add = i + 1; // Prepare computation outside lock
            
            // 1. Acquire lock
            let mut data_guard = data_clone.lock().unwrap(); 
            
            // 2. Quick update
            data_guard.value += value_to_add; 
            println!("Thread {}: Updated value to {}. Releasing lock.", i, data_guard.value);
            
            // 3. Lock released as data_guard goes out of scope immediately?
            // Or we can be explicit if we have more work to do:
            let current_value_snapshot = data_guard.value; // Copy value out if needed
            drop(data_guard); // Explicitly drop guard to release lock early
            
            // Now do other work without holding the lock.
            // Since the lock is dropped, other threads can access SharedData now.
            thread::sleep(Duration::from_secs(1));
            println!("Thread {}: Finished other work with snapshot value {}", i, current_value_snapshot);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final value: {}", shared_data.lock().unwrap().value);
}