use regex::Regex;
use std::fs::File;
use std::io::{self, Read, Write};

pub(crate) fn read_file_to_string(file_path: &str) -> io::Result<String>
{
   let mut file = File::open(file_path)?;
   let mut contents = String::new();
   file.read_to_string(&mut contents)?;

   Ok(contents)
}

pub(crate) fn find_and_replace(content: &mut String, pattern: &str, replacement: &str, ignore_case: bool)
{
   let regex = if ignore_case
   {
      Regex::new(&format!(r"(?i){}", pattern)).unwrap()
   }
   else
   {
      Regex::new(pattern).unwrap()
   };

   *content = regex.replace_all(content, replacement).to_string();
}

pub(crate) fn write_string_to_file(file_path: &str, content: &str) -> io::Result<()>
{
   let mut file = File::create(file_path)?;
   file.write_all(content.as_bytes())?;

   Ok(())
}
