struct Library {
    books: Vec<String>,
}

impl Library {
    fn new() -> Library {
        Library { books: Vec::new() }
    }

    fn add_book(&mut self, book: String) {
        self.books.push(book);
    }

    fn print_books(&self) {
        for book in &self.books {
            println!("{}", book);
        }
    }
}

fn main() {
    let mut library = Library::new();

    library.add_book(String::from("Rust Programming"));
    library.add_book(String::from("Advanced Rust"));

    library.print_books(); // Immutable borrow to print books
}