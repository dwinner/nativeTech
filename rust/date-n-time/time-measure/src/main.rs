use std::time::Instant;

fn measure_time<F>(task: F) -> std::time::Duration
where
   F: FnOnce(),
{
   let start = Instant::now();
   task();
   start.elapsed()
}

fn main()
{
   let duration = measure_time(|| {
      for _ in 0..1_000_000
      {
         // Долгая операция
      }
   });
   println!("Задача выполнена за: {:?}", duration);
}
