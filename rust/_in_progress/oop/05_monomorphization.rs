// 05_monomorphization.rs

use std::fmt::Debug;
use std::ops::Add;

// A generic function that takes any type `T` that can be added to itself
// and can be printed with the Debug trait.
// The `Copy` trait is needed so `value` can be used after being moved into the `+` operation.
fn add_and_print<T>(value: T)
where
    T: Add<Output = T> + Copy + Debug,
{
    let result = value + value; // `value` must implement `Add` and `Copy`
    println!("[Generic Function] {:?} + {:?} = {:?}", value, value, result);
}

fn main() {
    let my_integer: i32 = 10;
    let my_float: f64 = 5.5;

    // When the compiler sees this call, it generates a specialized version of `add_and_print` for `i32`.
    // It's as if you had written a function `add_and_print_i32(value: i32)`.
    add_and_print(my_integer);

    // When the compiler sees this call, it generates another specialized version for `f64`.
    // It's as if you had written a function `add_and_print_f64(value: f64)`.
    add_and_print(my_float);
    
    println!("Done! The compiler created two separate functions behind the scenes.");
}