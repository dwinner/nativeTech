use std::fs::File;
use std::path::PathBuf;

struct TempFile
{
   file: File,
   path: PathBuf,
}

impl TempFile
{
   fn new(path: PathBuf) -> std::io::Result<Self>
   {
      // Note: File::create() will overwrite existing files
      let file = File::create(&path)?;

      Ok(Self { file, path })
   }
}

// When TempFile is dropped:
// 1. First, the File will be automatically closed (Drop for File)
// 2. Then our drop implementation will remove the file
impl Drop for TempFile
{
   fn drop(&mut self)
   {
      // Note: File is already closed at this point
      if let Err(e) = std::fs::remove_file(&self.path)
      {
         eprintln!("Failed to remove temporary file: {}", e);
      }

      println!("> Dropped temporary file: {:?}", self.path);
   }
}

fn main() -> std::io::Result<()>
{
   // Create a new scope to demonstrate drop behavior
   {
      let temp = TempFile::new("test.txt".into())?;
      println!("Temporary file created");
      // File will be automatically cleaned up when temp goes out of scope
   }
   println!("End of scope - file should be cleaned up");

   // We can also manually drop if needed
   let temp2 = TempFile::new("another_test.txt".into())?;
   drop(temp2); // Explicitly drop the file
   println!("Manually dropped file");

   Ok(())
}
