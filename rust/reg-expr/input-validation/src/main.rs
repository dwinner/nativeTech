use regex::Regex;

fn is_valid_email(email: &str) -> bool
{
   let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
   re.is_match(email)
}

fn main()
{
   println!("{}", is_valid_email("user@example.com")); // true
   println!("{}", is_valid_email("invalid@.com")); // false
}
