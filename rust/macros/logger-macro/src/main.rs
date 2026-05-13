macro_rules! log {
   ($level:expr, $msg:expr) => {
      println!("[{}] {}", $level, $msg);
   };
   ($msg:expr) => {
      log!("INFO", $msg);
   };
}

macro_rules! repeat_n {
   ($n:expr, $action:expr) => {
      for _ in 0..$n
      {
         $action;
      }
   };
}

fn main()
{
   log!("Starting program...");
   log!("ERROR", "Something went wrong!");
   // Вывод:
   // [INFO] Starting program...
   // [ERROR] Something went wrong!

   repeat_n!(3, println!("Hello"));
   let mut counter = 0;
   repeat_n!(2, counter += 1);
   println!("Counter: {}", counter); // Вывод: Counter: 2
}
