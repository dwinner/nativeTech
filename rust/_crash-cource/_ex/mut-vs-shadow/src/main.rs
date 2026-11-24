fn main()
{
   let str_var = "my string".to_string();
   let str_var = "shadowed".to_string();
   let mut mut_str: String = String::new();
   mut_str.push_str("do it or... ");
   mut_str.push_str("not");

   println!("{}", str_var);
   println!("{}", mut_str);
}
