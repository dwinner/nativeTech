use regex::Regex;

fn main()
{
   let log = "2025-03-28 10:15:32 ERROR: Disk full\n2025-03-28 10:16:01 INFO: Backup started";
   let re = Regex::new(r"(?m)^(\d{4}-\d{2}-\d{2}) \d{2}:\d{2}:\d{2} (\w+): (.+)$").unwrap();

   for captures in re.captures_iter(log)
   {
      println!(
         "Дата: {}, Уровень: {}, Сообщение: {}",
         &captures[1], &captures[2], &captures[3]
      );
   }
}
