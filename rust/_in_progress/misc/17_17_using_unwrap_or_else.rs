// 17_17_using_unwrap_or_else.rs
use std::thread;
use std::time::Duration;

fn main() {
    // --- Concept: Lazy Evaluation with unwrap_or_else ---
    // unwrap_or_else takes a closure that is ONLY executed if the value is missing.
    // This is crucial for performance when the default value is expensive to compute.

    // 1. Case: Option is None
    let maybe_value: Option<i32> = None;
    
    let value = maybe_value.unwrap_or_else(|| {
        println!("(Computing default value because Option was None...)");
        // Simulate an expensive computation
        thread::sleep(Duration::from_millis(50)); 
        0
    });
    println!("Value from unwrap_or_else (was None): {}", value);

    // 2. Case: Option is Some
    let has_value: Option<i32> = Some(100);
    
    let value_from_some = has_value.unwrap_or_else(|| {
        // This block is effectively skipped
        println!("This closure won't be executed for Some(100)");
        0
    });
    println!("Value from unwrap_or_else (was Some): {}", value_from_some);

    // 3. Case: Result is Err
    // For Result, the closure receives the error, allowing dynamic handling.
    let result_err: Result<i32, &str> = Err("network failure");
    
    let value_from_err = result_err.unwrap_or_else(|err_msg| {
        println!("(Result was Err: '{}'. Providing fallback.)", err_msg);
        -1
    });
    println!("Value from unwrap_or_else (was Err): {}", value_from_err);
}