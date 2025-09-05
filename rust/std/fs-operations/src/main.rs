use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
#[cfg(target_family = "unix")]
use std::os::unix;
#[cfg(target_family = "windows")]
use std::os::windows;
use std::path::Path;

// A simple implementation of `% cat path`
fn cat(path: &Path) -> io::Result<String>
{
   let mut file = File::open(path)?;
   let mut file_content = String::new();
   match file.read_to_string(&mut file_content)
   {
      Ok(_) => Ok(file_content),
      Err(e) => Err(e),
   }
}

// A simple implementation of `% echo s > path`
fn echo(str_output: &str, path: &Path) -> io::Result<()>
{
   let mut file = File::create(path)?;
   file.write_all(str_output.as_bytes())
}

// A simple implementation of `% touch path` (ignores existing files)
fn touch(path: &Path) -> io::Result<()>
{
   match OpenOptions::new().create(true).write(true).open(path)
   {
      Ok(_) => Ok(()),
      Err(e) => Err(e),
   }
}

fn main()
{
   println!("`mkdir a`");

   // Create a directory, returns `io::Result<()>`
   match fs::create_dir("a")
   {
      Err(why) => println!("! {:?}", why.kind()),
      Ok(_) =>
      {}
   }

   println!("`echo hello > a/b.txt`");
   // The previous match can be simplified using the `unwrap_or_else` method
   echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
      println!("! {:?}", why.kind());
   });

   println!("`mkdir -p a/c/d`");
   // Recursively create a directory, returns `io::Result<()>`
   fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
      println!("! {:?}", why.kind());
   });

   println!("`touch a/c/e.txt`");
   touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
      println!("! {:?}", why.kind());
   });

   println!("`ln -s ../b.txt a/c/b.txt`");
   // Create a symbolic link, returns `io::Result<()>`
   #[cfg(target_family = "unix")]
   {
      unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
         println!("! {:?}", why.kind());
      });
   }
   #[cfg(target_family = "windows")]
   {
      windows::fs::symlink_file("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
         println!("! {:?}", why.to_string());
      });
   }

   println!("`cat a/c/b.txt`");
   match cat(&Path::new("a/c/b.txt"))
   {
      Err(why) => println!("! {:?}", why.kind()),
      Ok(s) => println!("> {}", s),
   }

   println!("`ls a`");
   // Read the contents of a directory, returns `io::Result<Vec<Path>>`
   match fs::read_dir("a")
   {
      Err(why) => println!("! {:?}", why.kind()),
      Ok(paths) =>
      {
         for path in paths
         {
            println!("> {:?}", path.unwrap().path());
         }
      }
   }

   println!("`rm a/c/e.txt`");
   // Remove a file, returns `io::Result<()>`
   fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
      println!("! {:?}", why.kind());
   });

   println!("`rmdir a/c/d`");
   // Remove an empty directory, returns `io::Result<()>`
   fs::remove_dir("a/c/d").unwrap_or_else(|why| {
      println!("! {:?}", why.kind());
   });
}
