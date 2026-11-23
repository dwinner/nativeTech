// 17_33_ineffective_logging.rs
// Dependencies required in Cargo.toml:
// [dependencies]
// log = "0.4"
// env_logger = "0.10"

use log::{info, debug, warn, error, trace};

fn main() {
    // --- The Fix: Structured Logging ---
    // Initialize the logger.
    // To see logs, run with env var: `RUST_LOG=trace cargo run`
    // or `RUST_LOG=info cargo run` to filter out lower levels.
    env_logger::init();

    info!("Program started (logged).");

    let val1 = complex_calculation_logged(5, 3);
    let val2 = complex_calculation_logged(2, 7);

    info!("Final values (logged): val1={}, val2={}", val1, val2);

    if val1 + val2 > 50 {
        error!("Sum ({}) exceeded 50 - error condition met.", val1 + val2);
    }

    info!("Program ended (logged).");

    // --- The Pitfall: Ineffective Logging ---
    // Uncommenting this would clutter stdout without way to filter it.
    // complex_calculation_print(5, 3);
}

// Good Practice: Leveled logging
fn complex_calculation_logged(a: i32, b: i32) -> i32 {
    // Trace: Very verbose, good for stepping through logic
    trace!("Entering calc with a={}, b={}", a, b);
    
    let intermediate = a * b + (a - b);
    
    // Debug: Useful info for developers
    debug!("Intermediate result: {}", intermediate);
    
    if intermediate < 0 {
        // Warn: Something unexpected but handled happened
        warn!("Result {} is negative, applying correction.", intermediate);
        return intermediate.abs() + 10;
    }
    
    trace!("Exiting calc.");
    intermediate
}

// Bad Practice: println! for debug
// Hard to disable in production, no severity levels, clutters standard output.
#[allow(dead_code)]
fn complex_calculation_print(a: i32, b: i32) -> i32 {
    println!("Starting complex_calculation with a={}, b={}", a, b);
    let intermediate = a * b + (a - b);
    println!("Intermediate result: {}", intermediate);
    
    if intermediate < 0 {
        println!("Result is negative, applying correction.");
        return intermediate.abs() + 10;
    }
    println!("Calculation finished.");
    intermediate
}