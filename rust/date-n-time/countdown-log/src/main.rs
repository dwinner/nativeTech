use chrono::Utc;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn log(message: &str)
{
   let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S");
   println!("[{}] {}", timestamp, message);
}

fn countdown_with_logging(seconds: u64)
{
   log("Таймер запущен");
   let duration = Duration::from_secs(seconds);
   let start = Instant::now();

   while start.elapsed() < duration
   {
      let remaining = duration - start.elapsed();
      log(&format!("Осталось {} секунд", remaining.as_secs()));
      sleep(Duration::from_secs(1));
   }

   log("Таймер завершён!");
}

fn main()
{
   countdown_with_logging(10);
}
