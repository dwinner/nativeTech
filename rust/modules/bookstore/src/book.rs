pub struct Book
{
   pub title: String,
   pub price: f64,
}

impl Book
{
   pub fn new(title: &str, price: f64) -> Book
   {
      Book {
         title: String::from(title),
         price,
      }
   }
}
