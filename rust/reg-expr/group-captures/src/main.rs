use regex::Regex;

fn main()
{
   // Захватывающая группа
   let re_capture = Regex::new(r"(\w+)-(\d+)").unwrap();
   let text = "dog-123";
   if let Some(captures) = re_capture.captures(text)
   {
      println!("Группа 1 (захват): {}", &captures[1]); // "dog"
      println!("Группа 2 (захват): {}", &captures[2]); // "123"
   }

   // Незахватывающая группа
   let re_non_capture = Regex::new(r"(?:\w+)-(\d+)").unwrap();
   if let Some(captures) = re_non_capture.captures(text)
   {
      println!("Группа 1 (захват): {}", &captures[1]); // "123"
      // caps[2] недоступно, т.к. (?:\w+) не захватывает
   }
}
