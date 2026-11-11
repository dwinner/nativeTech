fn main() {
    // `s1` is a String, which does not implement the `Copy` trait.
    let s1 = String::from("hello");

    // To create an independent duplicate of `s1`, we must explicitly call
    // .clone(). This performs a deep copy of the string data on the heap.
    let s2 = s1.clone();

    // Because we cloned `s1`, the original variable `s1` is still valid
    // and retains ownership
    // of its own data. `s2` is a brand new String that owns its own copy
    // of the data.
    println!("s1 = {}, s2 = {}", s1, s2);

    // We can modify one without affecting the other.
    // let mut s3 = s1.clone();
    // s3.push_str(", world!");
    // println!("s1 = {}, s3 = {}", s1, s3);
}