fn takes_ownership(some_string: String) {
    println!("Function received ownership of: {}", some_string);
}

fn main() {
    let s1 = String::from("hello");

    // We pass a clone of `s1`. The function takes ownership of the clone,
    // not the original `s1`.
    takes_ownership(s1.clone());

    // Because we only moved a clone, `s1` is still valid and can be used
    // here.
    println!("We can still use s1 after cloning: {}", s1);
}