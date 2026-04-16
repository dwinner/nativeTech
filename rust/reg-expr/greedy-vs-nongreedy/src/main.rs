use regex::Regex;

fn main()
{
   let text = "<tag>content</tag><tag>more</tag>";
   let greedy = Regex::new(r"<tag>.*</tag>").unwrap();
   let lazy = Regex::new(r"<tag>.*?</tag>").unwrap();

   println!("Жадный: {:?}", greedy.find(text).unwrap().as_str()); // "<tag>content</tag><tag>more</tag>"
   println!("Не жадный: {:?}", lazy.find(text).unwrap().as_str()); // "<tag>content</tag>"
}
