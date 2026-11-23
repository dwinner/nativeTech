// 05_rwlock_cache.rs
use std::sync::{Arc, RwLock};
use std::thread;
use std::collections::HashMap;
use std::time::Duration;

fn main() {
    // A cache that is read often, written to occasionally
    let cache: Arc<RwLock<HashMap<String, String>>> = Arc::new(RwLock::new(HashMap::new()));
    let mut handles = vec![];

    // Writer thread to populate the cache
    let cache_writer_clone = Arc::clone(&cache);
    let writer_handle = thread::spawn(move || {
        let mut cache_guard = cache_writer_clone.write().unwrap(); // Acquire write lock
        println!("Writer: Acquired write lock. Populating cache...");
        
        cache_guard.insert("url1".to_string(), "Data for URL1".to_string());
        cache_guard.insert("url2".to_string(), "Data for URL2".to_string());
        
        thread::sleep(Duration::from_millis(100));
        println!("Writer: Cache populated. Releasing write lock.");
        // Write lock released when cache_guard goes out of scope
    });
    handles.push(writer_handle);

    // Multiple reader threads
    for i in 0..3 {
        let cache_reader_clone = Arc::clone(&cache);
        let reader_handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(20 * i as u64)); // Stagger readers slightly
            
            // Acquire read lock. Multiple readers can hold this lock simultaneously,
            // but it blocks if a writer holds the write lock.
            let cache_guard = cache_reader_clone.read().unwrap(); 
            println!("Reader {}: Acquired read lock. Reading cache...", i);
            
            if let Some(data1) = cache_guard.get("url1") {
                println!("Reader {}: Found data for url1: '{}'", i, data1);
            }
            if let Some(data2) = cache_guard.get("url2") {
                println!("Reader {}: Found data for url2: '{}'", i, data2);
            }
            
            thread::sleep(Duration::from_millis(50));
            println!("Reader {}: Releasing read lock.", i);
            // Read lock released when cache_guard goes out of scope
        });
        handles.push(reader_handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Main: All threads finished.");
}