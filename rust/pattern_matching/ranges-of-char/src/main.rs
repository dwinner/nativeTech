fn main()
{
   let x = 'c';

   match x
   {
      'a'..='j' => println!("early ASCII letter"),
      'k'..='z' => println!("late ASCII letter"),
      _ => println!("something else"),
   }
}
