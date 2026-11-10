// 17_30_unused_variables.rs

fn main() {
    // --- The Warning: Unused Variables ---
    // Rust warns by default if you declare a variable but never use it.
    // This helps identify dead code or potential logic errors.

    // let some_data = 42; 
    // warning: unused variable: `some_data`

    // let another_value = "hello"; 
    // warning: unused variable: `another_value`

    println!("Program running...");

    // --- Fix 1: Silence with Underscore ---
    // If the variable is intentional (e.g., a placeholder or for strict signature matching),
    // prefix the name with `_`. The compiler treats this as "intentionally unused".
    let _some_data_placeholder = 42; 

    // --- Fix 2: Use the Variable ---
    // If the variable is needed, ensure it is actually read.
    let important_value = "hello";
    println!("The important value is: {}", important_value);

    // --- Fix 3: Partial Destructuring ---
    // Use `_` or `_name` to ignore specific parts of a tuple or struct.
    let (x, _y) = (1, 2); // We only need x; y is ignored
    println!("x is: {}", x);
}