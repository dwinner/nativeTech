use chrono::{FixedOffset, Utc};
use chrono_tz::Europe::Moscow;

fn main()
{
   // Время в UTC
   let utc_time = Utc::now();

   // Смещение +03:00 (например, Москва)
   let offset = FixedOffset::east_opt(3 * 3600).unwrap(); // 3 часа в секундах
   let local_time = utc_time.with_timezone(&offset);
   println!("Время в +03:00: {}", local_time);
   
   timezone_name();
}

fn timezone_name()
{
   let utc_time = Utc::now();
   let moscow_time = utc_time.with_timezone(&Moscow);
   println!("Время в Москве: {}", moscow_time);
}
