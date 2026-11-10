// 04_atomic_reference_counting.rs
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct SharedResource {
    id: u32,
    data: String,
}

fn main() {
    // Create shared data wrapped in an Arc (Atomic Reference Counted).
    // 'Arc' is just like 'Rc', but thread-safe. It uses atomic operations for
    // incrementing the counter, which has a small performance cost but allows sharing across threads.
    let shared_resource_main = Arc::new(SharedResource {
        id: 1001,
        data: "This is some important data shared across threads.".to_string(),
    });

    println!("Main thread: Initial strong count = {}", Arc::strong_count(&shared_resource_main)); // Output: 1

    let mut thread_handles = vec![];

    for i in 0..3 { // Spawn 3 threads
        // Clone the Arc for each thread. This is crucial.
        // We create a new pointer to the SAME data on the heap.
        // We cannot simply move 'shared_resource_main' because we need it in multiple threads.
        let shared_resource_for_thread = Arc::clone(&shared_resource_main);

        println!("Main thread: Count before spawning thread {} = {}", i, Arc::strong_count(&shared_resource_main));

        // Spawn the thread.
        // 'move' is required to move ownership of 'shared_resource_for_thread' into the closure.
        let handle = thread::spawn(move || {
            // This thread now has its own Arc pointing to the shared data
            println!(
                "Thread {}: Started. Accessing resource ID: {}, Data: '{}'. Strong count here: {}",
                i,
                shared_resource_for_thread.id,
                shared_resource_for_thread.data,
                // Count might be higher/lower depending on exactly when this runs relative to other threads
                Arc::strong_count(&shared_resource_for_thread) 
            );

            thread::sleep(Duration::from_millis(50)); // Simulate some work
            
            println!("Thread {}: Finished.", i);
            // When shared_resource_for_thread goes out of scope here, the count is decremented automatically.
        });

        thread_handles.push(handle);
    }

    // The main thread still has its reference, so it can access the data too.
    println!("Main thread: After spawning threads, strong count = {}", Arc::strong_count(&shared_resource_main));
    println!("Main thread: Resource data: {}", shared_resource_main.data);

    // Wait for all threads to complete
    for handle in thread_handles {
        handle.join().unwrap();
    }

    // When threads finish, they drop their Arcs. The count should return to 1 (just the main thread's reference).
    println!(
        "Main thread: All threads finished. Final strong count (before drop): {}", 
        Arc::strong_count(&shared_resource_main)
    ); 

    // When shared_resource_main drops at the end of main, the count goes to 0, 
    // and SharedResource is strictly deallocated then.
}