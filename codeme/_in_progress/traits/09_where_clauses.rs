// 09_where_clauses.rs
use std::fmt::{Debug, Display};

// VERSION 1: Inline Bounds
// Hard to read: The bounds clutter the function name and parameters.
fn compare_prints_messy<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {
    println!("Messy: t is '{}', u is '{:?}'", t, u);
}

// VERSION 2: Where Clause
// Easier to read: The bounds are moved to the end, separating the "what" from the "how".
fn compare_prints_clean<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("Clean: t is '{}', u is '{:?}'", t, u);
}

fn main() {
    let my_string = String::from("Rust");
    let my_int = 42;

    // Both functions work exactly the same way.
    // T = String (implements Display + Clone)
    // U = i32 (implements Clone + Debug)
    compare_prints_messy(&my_string, &my_int);
    compare_prints_clean(&my_string, &my_int);
}