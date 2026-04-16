use regex::Regex;

fn main()
{
   iterate();
   iterate_on_captures();
}

fn iterate()
{
   let re = Regex::new(r"\d+").unwrap();
   let text = "Year 2023, version 15";

   for mat in re.find_iter(text)
   {
      println!("Найдено: {}", mat.as_str()); // "2023", "15"
   }
}

fn iterate_on_captures()
{
   let re = Regex::new(r"(\d+)-(\w+)").unwrap();
   let text = "123-rust 456-code";

   for captures in re.captures_iter(text)
   {
      println!("Число: {}, Слово: {}", &captures[1], &captures[2]);
   }
}
