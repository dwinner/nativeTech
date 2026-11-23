// This function borrows a String and calculates its length.
fn calculate_length(s: &String) -> usize {
    s.len()
} // `s` goes out of scope here, but since it doesn't have ownership, the
  // data is not dropped.

fn main() {
    let s1 = String::from("hello");

    // We pass a reference to s1 using the `&` operator.
    // s1 is borrowed, not moved.
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    // `s1` is still valid here because its ownership was never
    // transferred.
}