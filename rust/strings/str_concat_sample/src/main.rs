fn main()
{
   let s1 = String::from("Hello, ");
   let s2 = String::from("world!");

   // s1 has been moved here and can no longer be used
   let s3 = s1 + &s2;

   println!("{}", s3);
}
