/**
 * Use idiomatic error or:
 * - use anyhow crate for wrapping nested errors;
 * - use thiserror crate for conversions;
 */
use std::error::Error;
use std::fmt::Formatter;
use std::io::BufRead;
use std::string::FromUtf8Error;


const MAX_LEN: usize = 1024;

#[derive(Debug)]
pub enum MyError
{
   Io(std::io::Error),
   Utf8(FromUtf8Error),
   General(String),
}

impl std::fmt::Display for MyError
{
   fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
   {
      match self
      {
         MyError::Io(e) =>
         {
            write!(f, "IO error: {}", e)
         }
         MyError::Utf8(e) =>
         {
            write!(f, "UTF-8 error: {}", e)
         }
         MyError::General(s) =>
         {
            write!(f, "General error: {}", s)
         }
      }
   }
}

impl Error for MyError
{
   fn source(&self) -> Option<&(dyn Error + 'static)>
   {
      match self
      {
         MyError::Io(e) => Some(e),
         MyError::Utf8(e) => Some(e),
         MyError::General(_) => None,
      }
   }
}

impl From<std::io::Error> for MyError
{
   fn from(value: std::io::Error) -> Self
   {
      Self::Io(value)
   }
}

impl From<FromUtf8Error> for MyError
{
   fn from(value: FromUtf8Error) -> Self
   {
      Self::Utf8(value)
   }
}

impl From<String> for MyError
{
   fn from(value: String) -> Self
   {
      MyError::General(value)
   }
}

fn main()
{
   let first_line = get_1st_line("usr");
   match first_line
   {
      Ok(username) =>
      {
         println!("{}", username);
      }
      Err(error) =>
      {
         eprintln!("{:?}", error);
      }
   }
}

fn get_1st_line(filename: &str) -> Result<String, MyError>
{
   let file = std::fs::File::open(filename)/*.map_err(MyError::Io)*/?;
   let mut reader = std::io::BufReader::new(file);

   // in a real life you can just use `reader.read_line()`
   let mut buf = vec![];
   let len = reader.read_until(b'\n', &mut buf)/*.map_err(MyError::Io)*/?;
   let result = String::from_utf8(buf)/*.map_err(MyError::Utf8)*/?;
   if result.len() > MAX_LEN
   {
      return Err(MyError::General(format!("Line too long: {}", len)));
   }

   Ok(result)
}
