fn main()
{
   let vector = vec![1, 2, 3, 4, 5];
   let third: &i32 = &vector[2];
   println!("The third element is {third}");
   let third: Option<&i32> = vector.get(2);
   match third
   {
      Some(third) => println!("The third element is {third}"),
      None => println!("There is no third element."),
   }
}
