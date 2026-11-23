// 17_19_deadlocks.rs
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let lock_a = Arc::new(Mutex::new(0)); // Resource A
    let lock_b = Arc::new(Mutex::new(0)); // Resource B

    println!("--- Starting Consistent Lock Ordering (Deadlock Fix) ---");

    // --- The Fix: Consistent Lock Ordering ---
    // To prevent deadlocks, ensure all threads acquire locks in the 
    // SAME global order (e.g., always A, then B).

    // Thread 1: Order A -> B
    let t1_la = Arc::clone(&lock_a);
    let t1_lb = Arc::clone(&lock_b);
    let thread1 = thread::spawn(move || {
        println!("Thread 1: Acquiring lock_a...");
        let _guard_a = t1_la.lock().unwrap();
        println!("Thread 1: Acquired lock_a. Simulating work...");
        
        thread::sleep(Duration::from_millis(50));
        
        println!("Thread 1: Acquiring lock_b...");
        let _guard_b = t1_lb.lock().unwrap();
        println!("Thread 1: Acquired lock_b.");
    });

    // Thread 2: Consistent Order A -> B (Fix)
    let t2_la = Arc::clone(&lock_a);
    let t2_lb = Arc::clone(&lock_b);
    let thread2 = thread::spawn(move || {
        println!("Thread 2: Acquiring lock_a (consistent order)...");
        // Even though Thread 2 might want to start, it waits here for Thread 1 to release A.
        let _guard_a = t2_la.lock().unwrap(); 
        println!("Thread 2: Acquired lock_a. Simulating work...");
        
        thread::sleep(Duration::from_millis(50));
        
        println!("Thread 2: Acquiring lock_b...");
        let _guard_b = t2_lb.lock().unwrap();
        println!("Thread 2: Acquired lock_b.");
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("Finished successfully (deadlock avoided).");

    // --- The Pitfall: Inconsistent Lock Ordering ---
    // Uncomment the block below to simulate a deadlock.
    // Thread 1 takes A->B, Thread 2 takes B->A.
    // They will both freeze, waiting for each other indefinitely.
    
    /*
    println!("\n--- Starting Inconsistent Lock Ordering (Deadlock Risk) ---");
    let d_lock_a = Arc::new(Mutex::new(0));
    let d_lock_b = Arc::new(Mutex::new(0));
    
    let d_t1_a = Arc::clone(&d_lock_a);
    let d_t1_b = Arc::clone(&d_lock_b);
    let d_thread1 = thread::spawn(move || {
        let _g_a = d_t1_a.lock().unwrap();
        thread::sleep(Duration::from_millis(50));
        let _g_b = d_t1_b.lock().unwrap(); // Waits for B
    });

    let d_t2_a = Arc::clone(&d_lock_a);
    let d_t2_b = Arc::clone(&d_lock_b);
    let d_thread2 = thread::spawn(move || {
        let _g_b = d_t2_b.lock().unwrap(); // Acquired B
        thread::sleep(Duration::from_millis(50));
        let _g_a = d_t2_a.lock().unwrap(); // Waits for A
    });

    d_thread1.join().unwrap();
    d_thread2.join().unwrap();
    */
}