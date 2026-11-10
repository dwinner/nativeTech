// 01_zero_cost_abstraction.rs

fn main() {
    println!("--- Zero Cost Abstractions Comparison ---");

    // 1. The "Control" version: What you'd write in C/C++ (Imperative)
    // This involves manual index management and mutable state.
    let mut total_imperative = 0;
    let my_vec = vec![1, 2, 3, 4, 5];
    
    for i in 0..my_vec.len() {
        total_imperative += my_vec[i] * 2;
    }
    println!("Imperative style result: {}", total_imperative);

    // 2. The "Abstraction" version: Safe, clear, and high-level (Functional)
    // Despite looking "heavier" with method calls (iter, map, sum),
    // Rust compiles this down to assembly almost identical to the loop above.
    // This is the "Zero-Cost Abstraction".
    let my_vec_2 = vec![1, 2, 3, 4, 5];
    let total_functional: i32 = my_vec_2.iter()
        .map(|x| x * 2)
        .sum();
    
    println!("Functional style result: {}", total_functional);

    assert_eq!(total_imperative, total_functional);
    println!("Success: Both approaches yield the exact same result.");
}