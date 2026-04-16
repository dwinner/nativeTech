use regex::Regex;

fn main()
{
   let re = Regex::new(r"(?P<number>\d+)-(?P<word>\w+)").unwrap();
   let text = "456-code";

   if let Some(captures) = re.captures(text)
   {
      println!("Число: {}", captures.name("number").unwrap().as_str()); // "456"
      println!("Слово: {}", captures.name("word").unwrap().as_str()); // "code"
   }
}
