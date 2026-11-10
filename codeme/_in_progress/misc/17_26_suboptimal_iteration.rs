// 17_26_suboptimal_iteration.rs

fn main() {
    let data = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    // --- The Pitfall: Manual Indexing ---
    // Using an index-based loop forces Rust to perform bounds checks 
    // (ensuring i < len) on every iteration to ensure safety.
    // While LLVM is good at optimizing this, it can sometimes inhibit 
    // advanced optimizations like auto-vectorization (SIMD).
    
    let mut sum_index = 0;
    for i in 0..data.len() {
        sum_index += data[i];
    }
    println!("Sum (indexed): {}", sum_index);

    // --- The Fix: Iterators ---
    // Iterators are a "zero-cost abstraction". They eliminate bounds checks
    // (since the iterator logic handles the boundaries) and make it easier
    // for the compiler to generate vectorized machine code.

    // Option 1: High-level methods (Cleanest & often fastest)
    let sum_method: i32 = data.iter().sum();
    println!("Sum (.sum()): {}", sum_method);

    // Option 2: Explicit Iterator Loop
    let mut sum_iter = 0;
    for &item in data.iter() {
        sum_iter += item;
    }
    println!("Sum (iter loop): {}", sum_iter);
}