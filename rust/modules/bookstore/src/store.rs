use crate::book::Book;

pub struct Store
{
   books: Vec<Book>,
}

impl Store
{
   pub fn new() -> Store
   {
      Store { books: Vec::new() }
   }

   pub fn add_book(&mut self, book: Book)
   {
      self.books.push(book);
   }

   pub fn total_price(&self) -> f64
   {
      self.books.iter().map(|book| book.price).sum()
   }
}
