mod book;
mod store;

use book::Book;
use store::Store;

fn main()
{
   let mut store = Store::new();
   store.add_book(Book::new("Война и мир", 15.99));
   store.add_book(Book::new("1984", 9.99));
   println!("Общая стоимость книг: ${:.2}", store.total_price());
}
