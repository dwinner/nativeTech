fn print_string(a_string: String)
{
   println!("print_String: {}", a_string);
}

fn print_str(a_str: &str)
{
   println!("print_str: {}", a_str);
}

fn main()
{
   // let s: str = "impossible str";
   print_string(String::from("String"));
   print_str(&String::from("String"));
   print_str("str");
   // print_String("str"); error! does not compile
}
