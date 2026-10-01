use std::fmt;
use std::io;

#[derive(Debug)]
pub struct StatsError
{
   pub _message: String,
}

impl fmt::Display for StatsError
{
   fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
   {
      write!(f, "{}", self)
   }
}

impl From<&str> for StatsError
{
   fn from(s: &str) -> Self
   {
      StatsError {
         _message: s.to_string(),
      }
   }
}

impl From<io::Error> for StatsError
{
   fn from(e: io::Error) -> Self
   {
      StatsError {
         _message: e.to_string(),
      }
   }
}

impl From<std::num::TryFromIntError> for StatsError
{
   fn from(_e: std::num::TryFromIntError) -> Self
   {
      StatsError {
         _message: "Number conversion error".to_string(),
      }
   }
}
