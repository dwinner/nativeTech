// 05_ignoring_tests.rs

// ---------------------------------------------------------
// INSTRUCTIONS:
// To run the tests in this file, compile and run the test harness:
// 1. Compile: rustc --test 05_ignoring_tests.rs -o test_runner
//
// 2. Run (Default): 
//    ./test_runner 
//    (This skips the ignored test)
//
// 3. Run (Include Ignored): 
//    ./test_runner --ignored
//    (This runs ONLY the ignored tests)
// ---------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    #[test]
    fn normal_fast_test() {
        assert_eq!(2 + 2, 4);
    }

    // This test is marked with #[ignore].
    // It will be printed as "ignored" in the output and won't execute 
    // unless you specifically ask for it.
    #[test]
    #[ignore]
    fn very_resource_intensive_and_slow_test() {
        // Simulate a slow operation (sleep for 2 seconds)
        thread::sleep(Duration::from_secs(2));
        assert!(true);
    }

    // You can also document WHY it is ignored
    #[test]
    #[ignore = "requires specific hardware setup"]
    fn hardware_dependent_test() {
        // ...
        assert!(true);
    }
}

fn main() {
    println!("This file contains unit tests.");
    println!("Run with `rustc --test 05_ignoring_tests.rs`");
}