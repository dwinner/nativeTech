use reqwest;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
   let url = "http://www.rustinaction.com/";
   let mut response = reqwest::get(url)?;

   let content = response.text()?;
   print!("{}", content);

   Ok(())
}
