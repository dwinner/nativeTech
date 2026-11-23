// 17_21_inefficient_scheduling.rs
// Dependency required in Cargo.toml:
// [dependencies]
// rayon = "1.10"

use std::thread;
use rayon::prelude::*; // Import Rayon's parallel iterators

fn main() {
    let num_tasks = 100;

    // --- The Pitfall: Inefficient Scheduling ---
    // Spawning 100 OS threads for short CPU-bound tasks creates massive overhead
    // (context switching, stack allocation).
    // Uncomment to run the inefficient version:
    // run_inefficient_raw_threads(num_tasks);

    // --- The Fix: Rayon Thread Pool ---
    // Rayon uses a "work-stealing" pool that adapts to the number of logical cores.
    // It creates just enough threads to saturate the CPU, avoiding overhead.
    
    println!("Starting Rayon (efficient) execution...");
    let results: Vec<usize> = (0..num_tasks)
        .into_par_iter() // Convert range into a parallel iterator
        .map(|i| perform_cpu_task(i)) // Tasks are distributed across the pool
        .collect();

    println!("Completed {} tasks using Rayon.", results.len());
}

// CPU-bound task simulation
fn perform_cpu_task(id: usize) -> usize {
    let mut sum = 0;
    // Vary work slightly per task
    for i in 0..(1_000_000 + id * 100) { 
        sum = (sum + i) % 1_000_000_007;
    }
    sum
}

// Bad Practice: Manual Thread Spawning
#[allow(dead_code)]
fn run_inefficient_raw_threads(num_tasks: usize) {
    println!("Starting raw threads (inefficient) execution...");
    let mut handles = vec![];

    for i in 0..num_tasks {
        handles.push(thread::spawn(move || {
            perform_cpu_task(i)
        }));
    }

    let mut results = Vec::with_capacity(num_tasks);
    for handle in handles {
        results.push(handle.join().unwrap());
    }
    println!("Completed {} tasks using raw threads.", results.len());
}