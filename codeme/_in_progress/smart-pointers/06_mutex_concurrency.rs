// 06_mutex_concurrency.rs
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// A wrapper struct to hold our shared counter.
// We use Arc to share it across threads, and Mutex to allow safe mutation.
struct GlobalCounter {
    count: Arc<Mutex<u32>>, 
}

impl GlobalCounter {
    fn new() -> Self {
        GlobalCounter { 
            count: Arc::new(Mutex::new(0)) 
        }
    }

    fn increment(&self) {
        // Acquire the lock. This blocks the current thread if another thread has the lock.
        // unwrap() is used here to handle "poisoned" mutexes (if another thread panicked while holding the lock).
        let mut num_guard = self.count.lock().unwrap();
        
        // We modify the data through the MutexGuard.
        // The guard behaves like a mutable reference to the data inside.
        *num_guard += 1; 
        
        // The lock is automatically released when `num_guard` goes out of scope here.
    }

    fn get_value(&self) -> u32 {
        // We must lock even just to read, to ensure no one else is writing at the same time.
        *self.count.lock().unwrap() 
    }
}

fn main() {
    let global_counter = GlobalCounter::new();
    let mut handles = vec![];

    println!("Initial count: {}", global_counter.get_value()); // Output: 0

    // Spawn 5 threads that all increment the same counter.
    for i in 0..5 {
        // We need to clone the Arc to give a new pointer to the new thread.
        // Note: We are cloning the internal Arc specifically for this example logic.
        let counter_clone = Arc::clone(&global_counter.count);

        let handle = thread::spawn(move || {
            // Inside the thread:
            for _ in 0..100 {
                // Lock the mutex to get access to the u32
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
                
                // Important: We drop the lock implicitly here because `num` goes out of scope 
                // at the end of the iteration, allowing other threads to jump in.
                
                // Slight delay to make interleaving more likely (simulating real work)
                thread::sleep(Duration::from_micros(1));
            }
            println!("Thread {} finished incrementing.", i);
        });
        handles.push(handle);
    }

    // While threads are running, we can also modify it from the main thread.
    // This will wait for the lock just like the spawned threads.
    global_counter.increment(); 

    // Wait for all spawned threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Expected calculation:
    // 5 threads * 100 increments = 500
    // + 1 increment from main thread = 501
    println!("Final count: {}", global_counter.get_value()); 
}