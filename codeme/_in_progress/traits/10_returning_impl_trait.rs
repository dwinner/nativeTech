// 10_returning_impl_trait.rs
use std::fmt::Display;

// 1. Valid Usage: Returning "impl Trait"
// The caller doesn't know the exact concrete type (here, &str),
// only that the returned value satisfies the 'Display' trait.
fn get_status() -> impl Display {
    "System All Green"
}

// 2. Invalid Usage: Conditional Types
// Even though both &str and i32 implement Display, this fails.
// 'impl Trait' requires the function to return a SINGLE concrete type.
/*
fn invalid_return(flag: bool) -> impl Display {
    if flag {
        "Success" // Type: &str
    } else {
        100       // Type: i32
    }
}
*/

fn main() {
    let status = get_status();
    // We can print it because it implements Display
    println!("Current Status: {}", status);
}