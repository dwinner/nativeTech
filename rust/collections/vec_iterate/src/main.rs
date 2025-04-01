fn main()
{
   let mut vector = vec![100, 32, 57];

   // changing elements during iteration
   for i in &mut vector
   {
      *i += 50;
   }

   // forward only iteration
   for i in &vector
   {
      println!("{i}");
   }
}
