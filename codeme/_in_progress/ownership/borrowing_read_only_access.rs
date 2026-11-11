struct Book {
    title: String,
    author: String,
}

fn print_book_title(book: &Book) {
    println!("The book title is: {}", book.title);
}

fn main() {
    let my_book = Book {
        title: String::from("Rust Programming"),
        author: String::from("John Doe"),
    };

    print_book_title(&my_book); // Borrowing the struct immutably
    println!("The book author is: {}", my_book.author); // The original
    // data is still accessible
}