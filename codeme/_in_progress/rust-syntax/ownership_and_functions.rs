// This function takes ownership of the String passed to it.
fn takes_ownership(some_string: String) {
    println!("Inside takes_ownership: {}", some_string);
} // `some_string` is dropped here, and its memory is freed.

// This function takes a copy of the integer.
fn makes_copy(some_integer: i32) {
    println!("Inside makes_copy: {}", some_integer);
} // `some_integer` goes out of scope, but nothing special happens.

fn main() {
    let s = String::from("hello");

    // `s`'s value is moved into the function...
    takes_ownership(s);
    // ...so `s` is no longer valid here.

    // The next line would cause a compile-time error:
    // println!("Trying to use s after move: {}", s);

    let x = 5;

    // `x`'s value is copied into the function...
    makes_copy(x);
    // ...so `x` is still valid and can be used here.
    println!("x is still valid after makes_copy: {}", x);
}