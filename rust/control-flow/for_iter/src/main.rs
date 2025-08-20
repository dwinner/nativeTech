fn main()
{
   let names = vec!["Bob", "Frank", "Ferris"];

   // borrowing
   for name in names.iter()
   {
      match name
      {
         &"Ferris" => println!("There is a rustacean among us!"),
         // TODO ^ Try deleting the & and matching just "Ferris"
         _ => println!("Hello {}", name),
      }
   }

   println!("names: {:?}", names);

   // mutable borrowing
   let mut names = vec!["Bob", "Frank", "Ferris"];
   for name in names.iter_mut()
   {
      *name = match name
      {
         &mut "Ferris" => "There is a rustacean among us!",
         _ => "Hello",
      }
   }

   // owning
   for name in names.into_iter()
   {
      match name
      {
         "Ferris" => println!("There is a rustacean among us!"),
         _ => println!("Hello {}", name),
      }
   }

   // ERROR: println!("names: {:?}", names);
}
