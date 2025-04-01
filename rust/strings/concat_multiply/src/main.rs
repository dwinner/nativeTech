fn main()
{
   let s1 = String::from("tic");
   let s2 = String::from("tac");
   let s3 = String::from("toe");

   let s = s1 + "-" + &s2 + "-" + &s3;

   println!("{}", s);
}
