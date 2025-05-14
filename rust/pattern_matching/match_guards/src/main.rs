fn main()
{
   let num = Some(4);

   match num
   {
      Some(x) if x % 2 == 0 => println!("The number {x} is even"),
      Some(x) => println!("The number {x} is odd"),
      None => (),
   }
}
