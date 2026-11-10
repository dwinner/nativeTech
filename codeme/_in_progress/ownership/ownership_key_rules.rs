fn main() {
    let book_title = String::from("The Rust Programming Guide");
    println!("{}", book_title); // book_title is valid here

    {
        let another_title = String::from("Rust and Beyond");
        println!("{}", another_title); // another_title is valid here
    } // another_title goes out of scope and is dropped here
} // book_title goes out of scope and is dropped here