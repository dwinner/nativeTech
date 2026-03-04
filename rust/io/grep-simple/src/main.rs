/**
 * grep - Search stdin or some files for lines matching a given string.
 */
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::PathBuf;

fn grep<R>(target: &str, reader: R) -> io::Result<()>
where
   R: BufRead,
{
   for line_result in reader.lines()
   {
      let line = line_result?;
      if line.contains(target)
      {
         println!("{}", line);
      }
   }

   Ok(())
}

fn grep_main() -> Result<(), Box<dyn Error>>
{
   // Get the command-line arguments. The first argument is the
   // string to search for; the rest are filenames.
   let mut args = std::env::args().skip(1);
   let target = match args.next()
   {
      Some(arg) => arg,
      None => Err("usage: grep PATTERN FILE...")?,
   };

   let path_collection: Vec<PathBuf> = args.map(PathBuf::from).collect();
   if path_collection.is_empty()
   {
      let stdin = io::stdin();
      grep(&target, stdin.lock())?;
   }
   else
   {
      for path in path_collection
      {
         let file = File::open(path)?;
         grep(&target, BufReader::new(file))?;
      }
   }

   Ok(())
}

fn main()
{
   let result = grep_main();
   if let Err(err) = result
   {
      eprintln!("{}", err);
      std::process::exit(1);
   }
}
