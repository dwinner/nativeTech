fn main() {
    let s = String::from("hello");

    takes_ownership(s);

    // println!("{}", s); // This line would cause a compile-time error
    // because s is no longer valid
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}