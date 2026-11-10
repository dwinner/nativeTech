// 17_12_unwrapping_without_care.rs

fn main() {
    // --- The Pitfall: Unwrapping without care ---
    // Using .unwrap() causes a panic if the result is Err or None.
    
    // If you run this binary without arguments (or with a non-integer),
    // the following line will crash the program.
    // let number = get_first_arg_as_int(); 
    // println!("Pitfall result: {}", number);
    
    println!("Skipped dangerous unwrap example to avoid panic. Uncomment code above to test.");

    // --- The Fix: Graceful Handling ---
    // We handle the possibility of missing args or parsing errors explicitly.
    match get_first_arg_as_int_safe() {
        Ok(number) => println!("Safe result: First argument as integer is {}", number),
        Err(e) => eprintln!("Safe error handled: {}", e),
    }

    // --- Note on .expect() ---
    // If a panic is truly intended (e.g., for critical configuration that MUST exist),
    // use .expect() to provide a clear error message instead of a generic unwrap panic.
    
    // let critical_config = std::env::var("MUST_EXIST_ENV_VAR")
    //    .expect("Critical environment variable MUST_EXIST_ENV_VAR is not set!");
}

// Bad Practice: High risk of panic
fn get_first_arg_as_int() -> i32 {
    let args: Vec<String> = std::env::args().collect();
    // 1. args.get(1) might be None -> unwrap() panics
    // 2. parse() might be Err -> unwrap() panics
    args.get(1).unwrap().parse().unwrap()
}

// Good Practice: Using Combinators for Safety
fn get_first_arg_as_int_safe() -> Result<i32, String> {
    let args: Vec<String> = std::env::args().collect();
    
    args.get(1) // Returns Option<&String>
        .ok_or_else(|| String::from("No first argument provided.")) // Convert Option to Result
        .and_then(|arg_str| { // If Ok, try to parse
            arg_str.parse::<i32>()
                .map_err(|parse_err| format!("Failed to parse '{}': {}", arg_str, parse_err))
        })
}