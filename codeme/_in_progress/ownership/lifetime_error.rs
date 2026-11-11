// This function attempts to return a reference to data that will be
// deallocated.
// Rust's compiler will prevent this with a lifetime error.
fn get_dangling_reference() -> &String {
    let s = String::from("hello");

    &s // We are trying to return a reference to `s`.
} // But `s` goes out of scope and is dropped here, so the memory is
  // freed.
// The returned reference would be "dangling" - pointing to invalid
// memory.

fn main() {
    // let reference_to_nothing = get_dangling_reference();
    // If the code above were allowed to compile, `reference_to_nothing`
    // would be a dangling reference, and using it would be undefined
    // behavior.
}