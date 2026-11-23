// 17_20_atomic_memory_ordering.rs
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(AtomicUsize::new(0));
    let ready_flag = Arc::new(AtomicBool::new(false));

    let data_producer = Arc::clone(&data);
    let ready_producer = Arc::clone(&ready_flag);

    // --- Producer Thread ---
    let producer_handle = thread::spawn(move || {
        // 1. Write the data.
        // We can use Relaxed here strictly because the subsequent operation 
        // (store to ready_flag) will act as a memory fence (Release).
        data_producer.store(42, Ordering::Relaxed);
        
        println!("Producer: Data written.");

        // 2. Set the ready flag.
        // --- The Pitfall Fix ---
        // If we used `Ordering::Relaxed` here, the compiler or CPU could reorder 
        // these instructions. The Consumer might see `ready_flag = true` 
        // BEFORE `data = 42` propagates to its cache.
        //
        // `Ordering::Release` ensures that all previous writes (including data_producer)
        // are visible to any thread that Acquires this flag.
        ready_producer.store(true, Ordering::Release);
        println!("Producer: Flag raised (Release).");
    });

    // --- Consumer Thread ---
    let consumer_handle = thread::spawn(move || {
        // 3. Wait for the flag.
        // --- The Pitfall Fix ---
        // We MUST use `Ordering::Acquire`. This ensures that any memory operations 
        // that happen *after* this load in the code (like reading the data) 
        // actually happen after we observe the flag is true.
        // If we used Relaxed, we might see the flag true but read stale data.
        while !ready_flag.load(Ordering::Acquire) {
            std::thread::yield_now(); // Spin politely without burning CPU
        }

        // 4. Read the data.
        // Because of the Release-Acquire synchronization on `ready_flag`,
        // we are guaranteed to see the value 42 written by the producer.
        let val = data.load(Ordering::Relaxed);
        println!("Consumer: Flag seen (Acquire), data is {}", val);
    });

    producer_handle.join().unwrap();
    consumer_handle.join().unwrap();
}