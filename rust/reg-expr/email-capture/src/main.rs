use regex::Regex;
use std::fs;

fn extract_emails(file_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>>
{
   let content = fs::read_to_string(file_path)?;
   let re = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}")?;
   let emails: Vec<String> = re.find_iter(&content).map(|mat| mat.as_str().to_string()).collect();

   Ok(emails)
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{
   let emails = extract_emails("emails.txt")?;
   if emails.is_empty()
   {
      println!("Email-адреса не найдены.");
   }
   else
   {
      println!("Найденные email-адреса:");
      for email in emails
      {
         println!("- {}", email);
      }
   }

   Ok(())
}
