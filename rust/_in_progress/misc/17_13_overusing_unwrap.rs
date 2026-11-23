// 17_13_overusing_unwrap.rs
use std::collections::HashMap;

fn main() {
    let mut settings = HashMap::new();
    settings.insert("timeout", "30s");

    // --- The Pitfall: Overusing unwrap() ---
    // If we uncomment the lines below, the program will panic because "retries" is missing.
    // let retries = get_setting(&settings, "retries");
    // println!("Retries: {}", retries);
    
    println!("Skipped panic example. Uncomment code above to test.");

    // --- The Fix: Safe Access & Defaults ---
    
    // 1. Using Option matching (get_setting_safe)
    match get_setting_safe(&settings, "retries") {
        Some(val) => println!("Retries found: {}", val),
        None => println!("Retries setting not found (handled safely)."),
    }

    // 2. Using Combinators for Defaults (get_setting_with_default)
    let retries_val = get_setting_with_default(&settings, "retries", "3");
    println!("Retries (with default): {}", retries_val);
    
    let timeout_val = get_setting_with_default(&settings, "timeout", "60s");
    println!("Timeout (found existing): {}", timeout_val);
}

// Bad Practice: Panics on missing key
fn get_setting<'a>(settings: &'a HashMap<&str, &str>, key: &str) -> &'a str {
    // .unwrap() is a blunt instrument here. It crashes if the key isn't found.
    *settings.get(key).unwrap()
}

// Good Practice: Returns Option, forcing caller to handle absence
fn get_setting_safe<'a>(settings: &'a HashMap<&str, &str>, key: &str) -> Option<&'a str> {
    // We map the Option<&&str> to Option<&str> for easier usage
    settings.get(key).map(|&v| v) 
}

// Good Practice: Provides a fallback value
fn get_setting_with_default<'a>(
    settings: &'a HashMap<&str, &str>, 
    key: &str, 
    default: &'a str
) -> &'a str {
    // map_or returns the value if present, or the default if None
    settings.get(key).map_or(default, |&v| v)
}