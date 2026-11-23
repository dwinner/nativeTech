// 03_arc_sharing.rs
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct ImportantConfig {
    api_url: String,
    max_retries: u32,
}

fn main() {
    // Data we want to share (read-only) across multiple threads
    let config = Arc::new(ImportantConfig {
        api_url: "https://api.example.com/data".to_string(),
        max_retries: 5,
    });

    let mut handles = vec![];

    println!("Main thread: Initial Arc strong count = {}", Arc::strong_count(&config));

    for i in 0..3 { // Spawn 3 threads
        // Clone the Arc for each thread. The clone is moved into the thread.
        let config_clone = Arc::clone(&config);
        
        println!("Main thread: Count before thread {} spawn: {}", i, Arc::strong_count(&config));

        let handle = thread::spawn(move || {
            // This thread now has its own Arc pointing to the same ImportantConfig data
            println!("Thread {}: Started. Accessing resource API URL '{}' with max_retries = {}. Current Arc count in this thread's scope (approx): {}", 
                i, 
                config_clone.api_url, // Accessing data through Arc
                config_clone.max_retries,
                Arc::strong_count(&config_clone)
            );

            // Simulate some work
            thread::sleep(Duration::from_millis(100));
            println!("Thread {}: Finished.", i);

            // When config_clone goes out of scope here, the count is decremented.
        });
        handles.push(handle);
    }

    println!("Main thread: Count after all threads spawned: {}", Arc::strong_count(&config));
    println!("Main thread: Resource API URL: {}", config.api_url);

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Main thread: All threads finished. Final Arc strong count (before main's Arc drops): {}", Arc::strong_count(&config));
    // When 'config' in main goes out of scope, the count drops to 0, and ImportantConfig is deallocated.
}