// 17_24_unoptimized_data_structures.rs
use std::collections::VecDeque;
use std::time::Instant;

fn main() {
    // We use a moderately large N to demonstrate the O(N^2) impact clearly.
    // Try increasing this to 100,000 if your machine is very fast.
    let n = 50_000;

    // --- The Pitfall: Using Vec as a Queue (Front Insertion) ---
    // Inserting at index 0 requires shifting ALL existing elements in memory 
    // to the right to make space. 
    // Complexity for loop: O(N^2) (Very Slow)
    
    println!("--- Inefficient: Vec::insert(0, ...) ---");
    let start_vec = Instant::now();

    let mut vec_queue = Vec::new();
    for i in 0..n {
        vec_queue.insert(0, i); // Costly shift operation every time
    }

    println!("Time: {:?} | Final len: {}", start_vec.elapsed(), vec_queue.len());


    // --- The Fix: Using VecDeque ---
    // VecDeque is implemented as a circular buffer (ring buffer).
    // It can grow in both directions. Pushing to the front is O(1).
    // Complexity for loop: O(N) (Very Fast)

    println!("\n--- Efficient: VecDeque::push_front(...) ---");
    let start_deque = Instant::now();

    let mut deque_queue = VecDeque::new();
    for i in 0..n {
        deque_queue.push_front(i); // Constant time operation
    }

    println!("Time: {:?} | Final len: {}", start_deque.elapsed(), deque_queue.len());
}