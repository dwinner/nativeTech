// 17_36_debugging_concurrency.rs
use std::sync::{Arc, Mutex};
use std::thread;
use log::{info, debug, error}; // Requires 'log' and 'env_logger' crates

fn main() {
    env_logger::init(); // Initialize logger
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..3 {
        let counter_clone = Arc::clone(&counter);
        let thread_name = format!("worker-{}", i);

        // Name the thread for easier debugging
        let handle = thread::Builder::new()
            .name(thread_name.clone())
            .spawn(move || {
                info!("[{}] Started", thread_name);
                for j in 0..5 {
                    {
                        let mut num_guard = counter_clone.lock().unwrap();
                        *num_guard += 1;
                        debug!("[{}] Count: {}, Iter: {}", thread_name, *num_guard, j);
                    } // Guard dropped here, lock released

                    // Simulate varied work
                    thread::sleep(std::time::Duration::from_millis(10 + i * 5));
                }
                info!("[{}] Finished", thread_name);
            }).unwrap();
            
        handles.push(handle);
    }

    for handle in handles {
        let name = handle.thread().name().unwrap_or("unnamed").to_string();
        if let Err(e) = handle.join() {
            error!("Thread {} panicked: {:?}", name, e);
        } else {
            info!("Thread {} joined.", name);
        }
    }
}