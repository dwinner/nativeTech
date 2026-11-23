fn main() {
    let mut s = String::from("hello");

    takes_mutable_reference(&mut s);

    println!("{}", s); // s has been modified
}

fn takes_mutable_reference(some_string: &mut String) {
    some_string.push_str(", world");
}