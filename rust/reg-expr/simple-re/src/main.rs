use regex::Regex;

fn main()
{
   let re = Regex::new(r"\d+").unwrap(); // Ищет одну или более цифр
   let text = "Rust 2025";

   println!("Есть совпадение? {}", re.is_match(text)); // true
   if let Some(mat) = re.find(text)
   {
      println!("Найдено: {}", mat.as_str()); // "2025"
   }
}
