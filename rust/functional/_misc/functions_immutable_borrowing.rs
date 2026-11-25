fn main() {
    let s = String::from("hello");

    takes_reference(&s);

    println!("{}", s); // s is still valid here
}

fn takes_reference(some_string: &String) {
    println!("{}", some_string);
}