use regex::Regex;

fn main()
{
   replacing();
   capture_replacing();
}

fn replacing()
{
   let re = Regex::new(r"\d+").unwrap();
   let text = "Year 2023, version 15";

   let replaced = re.replace(text, "XX");
   println!("Первая замена: {}", replaced); // "Year XX, version 15"

   let replaced_all = re.replace_all(text, "XX");
   println!("Полная замена: {}", replaced_all); // "Year XX, version XX"
}

fn capture_replacing()
{
   let re = Regex::new(r"(\d+)-(\w+)").unwrap();
   let text = "123-rust";

   let result = re.replace(text, "$2-$1");
   println!("Результат: {}", result); // "rust-123"
}
