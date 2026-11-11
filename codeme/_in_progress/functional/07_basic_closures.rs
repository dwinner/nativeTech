// 07_basic_closures.rs

fn main() {
    // --- 1. Basic closure, relying on type inference ---
    // The closure's types are inferred based on the first time it is called (with '5').
    let add_one = |x| x + 1;
    println!("5 + 1 = {}", add_one(5)); // Output: 6

    // --- 2. Closure with explicit type annotations ---
    let multiply = |a: i32, b: i32| -> i32 {
        // No semicolon needed if it's the return expression
        a * b 
    };
    println!("3 * 4 = {}", multiply(3, 4)); // Output: 12

    // --- 3. Closure with a block body ---
    let complex_closure = |x: i32| {
        println!("Calculating for input: {}", x);
        let result = x * x + 2 * x + 1;
        // 'return' keyword is optional for the last expression in a block
        result
    };
    println!("Complex result for 3: {}", complex_closure(3)); // Output: Calculating... 16
}