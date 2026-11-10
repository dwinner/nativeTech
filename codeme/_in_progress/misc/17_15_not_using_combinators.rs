// 17_15_not_using_combinators.rs

fn main() {
    // --- Test 1: Getting Length ---
    let text = Some(String::from("hello"));
    
    // Verbose approach
    println!("Length (verbose): {}", get_length_verbose(text.clone()));
    
    // Combinator approach
    println!("Length (combinator): {}", get_length_with_combinators(text));


    // --- Test 2: Parsing Optional String ---
    let valid = Some("123");
    let invalid = Some("abc");
    let missing = None;

    // Verbose approach
    println!("Parse Valid (verbose): {}", parse_optional_str_verbose(valid, 0));
    println!("Parse Invalid (verbose): {}", parse_optional_str_verbose(invalid, 0));
    
    // Combinator approach
    println!("Parse Valid (combinator): {}", parse_optional_str_combinators(valid, 0));
    println!("Parse Invalid (combinator): {}", parse_optional_str_combinators(invalid, 0));
    println!("Parse None (combinator): {}", parse_optional_str_combinators(missing, 0));
}

// --- The Pitfall: Verbose Matching ---
// Using nested match statements makes the code harder to read and maintain.

fn get_length_verbose(text_opt: Option<String>) -> usize {
    match text_opt {
        Some(s) => s.len(), // Get length if Some
        None => 0,          // Return 0 if None
    }
}

fn parse_optional_str_verbose(opt_s: Option<&str>, default_val: u32) -> u32 {
    match opt_s {
        Some(s) => {
            // Nested match to handle parsing result
            match s.parse::<u32>() {
                Ok(num) => num,
                Err(_) => default_val, 
            }
        }
        None => default_val, 
    }
}

// --- The Fix: Using Combinators ---
// Methods like map, and_then, and unwrap_or allow for cleaner, linear data flow.

fn get_length_with_combinators(text_opt: Option<String>) -> usize {
    text_opt
        .map(|s| s.len()) // Transform inner value if it exists
        .unwrap_or(0)     // Provide fallback if None
}

fn parse_optional_str_combinators(opt_s: Option<&str>, default_val: u32) -> u32 {
    opt_s
        .and_then(|s| s.parse::<u32>().ok()) // .ok() converts Result -> Option, silencing errors
        .unwrap_or(default_val)              // Flattened handling of None (from input OR parse fail)
}