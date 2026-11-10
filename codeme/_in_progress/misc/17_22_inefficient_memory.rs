// 17_22_inefficient_memory.rs
use std::time::Instant;

fn main() {
    let n = 1_000_000; // Number of elements

    // --- The Pitfall: Inefficient Memory Management ---
    // Using Vec::new() starts with 0 capacity. As we push, the vector fills up,
    // allocates a larger chunk of memory, copies all items over, and frees the old chunk.
    println!("--- Inefficient Approach ---");
    let start = Instant::now();
    
    let mut vec_inefficient = Vec::new(); 
    for i in 0..n {
        vec_inefficient.push(i); 
    }
    
    let duration = start.elapsed();
    println!("Time: {:?}, Final Capacity: {}", duration, vec_inefficient.capacity());
    // Note: The capacity is often larger than 'n' because it doubles (growth strategy).

    // --- The Fix: Pre-allocation ---
    // We tell the vector exactly how much memory to reserve upfront.
    // This avoids the costly cycle of allocate-copy-free.
    println!("\n--- Efficient Approach (with_capacity) ---");
    let start = Instant::now();
    
    let mut vec_efficient = Vec::with_capacity(n);
    for i in 0..n {
        vec_efficient.push(i);
    }
    
    let duration = start.elapsed();
    println!("Time: {:?}, Final Capacity: {}", duration, vec_efficient.capacity());
    
    // --- Visualizing Reallocation (Optional) ---
    // Uncomment to see the "doubling" strategy in action
    // check_reallocations();
}

#[allow(dead_code)]
fn check_reallocations() {
    let mut v = Vec::new();
    let mut old_cap = 0;
    println!("\n--- Reallocation Log ---");
    for i in 0..1025 {
        v.push(i);
        if v.capacity() != old_cap {
            println!("Resized: {} -> {}", old_cap, v.capacity());
            old_cap = v.capacity();
        }
    }
}