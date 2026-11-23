// 07_rwlock_concurrency.rs
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    // A shared configuration that many threads will read, but one will update.
    // RwLock (Read-Write Lock) is optimized for scenarios with many readers and few writers.
    let config = Arc::new(RwLock::new("Initial Config".to_string()));
    
    let mut handles = vec![];

    // --- Spawn multiple READER threads ---
    for i in 0..5 {
        let config_clone = Arc::clone(&config);
        
        handles.push(thread::spawn(move || {
            // Simulate variable start times so readers overlap
            thread::sleep(Duration::from_millis(i * 5));

            // Acquire a READ lock. 
            // Multiple threads can hold this lock simultaneously.
            // This blocks ONLY if a Writer currently holds the write lock.
            let config_guard = config_clone.read().unwrap();
            
            println!("Reader {}: sees config: '{}'", i, *config_guard);
            
            // The read lock is released here when config_guard goes out of scope.
        }));
    }

    // --- Spawn one WRITER thread ---
    let config_clone = Arc::clone(&config);
    handles.push(thread::spawn(move || {
        // Simulate waiting for some condition before writing
        thread::sleep(Duration::from_millis(10));
        
        // Acquire a WRITE lock.
        // This will wait until ALL current readers are finished.
        // Once held, NO other readers or writers can access the data until this is released.
        let mut config_guard = config_clone.write().unwrap();
        
        println!("\n--- Writer: attempting to update... ---");
        *config_guard = "Updated Config".to_string();
        println!("--- Writer: Updated config! ---\n");
        
        // Write lock released here.
    }));

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Final check from the main thread
    let final_read = config.read().unwrap();
    println!("\nFinal config value: '{}'", *final_read);
}