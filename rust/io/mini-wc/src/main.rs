use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()>
{
   let file_name = "fox.txt";
   match count_words(file_name)
   {
      Ok(count) if count == 0 =>
      {
         println!("file '{}' has no content", file_name)
      }
      Ok(count) =>
      {
         println!("Words count in file '{}': {}", file_name, count)
      }
      Err(err) =>
      {
         eprintln!("Error: {}", err)
      }
   }

   Ok(())
}

fn count_words(a_filename: &str) -> std::io::Result<usize>
{
   let file = File::open(a_filename)?;
   let reader = BufReader::new(file);
   let mut word_count = 0;

   for line in reader.lines()
   {
      let line = line?;
      let l_words_count = line.split_whitespace().count();
      word_count += l_words_count;
   }

   Ok(word_count)
}
