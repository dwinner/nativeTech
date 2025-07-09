use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Circle
{
   radius: i32,
}

impl fmt::Display for Circle
{
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
   {
      write!(f, "Circle of radius {}", self.radius)
   }
}

impl FromStr for Circle
{
   type Err = ParseIntError;
   fn from_str(s: &str) -> Result<Self, Self::Err>
   {
      match s.trim().parse()
      {
         Ok(num) => Ok(Circle { radius: num }),
         Err(e) => Err(e),
      }
   }
}

fn main()
{
   let circle = Circle { radius: 6 };

   // converting to String
   println!("{}", circle.to_string());

   // parsing a String
   let parsed: i32 = "5".parse().unwrap();
   let turbo_parsed = "10".parse::<i32>().unwrap();

   let sum = parsed + turbo_parsed;
   println!("Sum: {:?}", sum);

   let radius = "    3 ";
   let circle: Circle = radius.parse().unwrap();
   println!("{:?}", circle);
}
