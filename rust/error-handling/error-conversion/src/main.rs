use std::{fs::File, io::Read};

#[derive(Debug)]
struct Error(String);

impl From<std::io::Error> for Error {
   fn from(other: std::io::Error) -> Self {
      Self(other.to_string())
   }
}

fn read_file(a_name: &str) -> Result<String, Error> {
   let mut l_file = File::open(a_name)?;
   let mut l_output = String::new();
   l_file.read_to_string(&mut l_output)?;

   Ok(l_output)
}

fn main() {
   println!("{}", read_file("src/main.rs").unwrap()); // this line succeeds
   println!("{}", read_file("src/failure.rs").unwrap()); // this line fails
}
