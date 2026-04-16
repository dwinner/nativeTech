use lazy_regex::lazy_regex;
use regex::Regex;

fn main()
{
   let re = Regex::new(r"\d+").unwrap();
   process_text("Rust 2025", &re);
   process_text("No numbers", &re);

   using_lazy_re();
}

fn process_text(text: &str, re: &Regex)
{
   if re.is_match(text)
   {
      println!("Совпадение найдено!");
   }
}

fn using_lazy_re()
{
   let re = lazy_regex!(r"\d+"); // Компиляция отложена
   println!("{}", re.is_match("Rust 2025")); // true
}
