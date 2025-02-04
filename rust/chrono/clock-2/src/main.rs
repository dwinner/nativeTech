#[cfg(windows)]
use kernel32;

#[cfg(not(windows))]
use libc;

#[cfg(windows)]
use winapi;

use chrono::{DateTime, Datelike, Local, TimeZone, Timelike};
use clap::{App, Arg};
use std::mem::zeroed;

struct Clock;

impl Clock {
   fn get() -> DateTime<Local> {
      Local::now()
   }

   #[cfg(windows)]
   fn set<Tz: TimeZone>(date_time: DateTime<Tz>) -> () {
      use chrono::Weekday;
      use kernel32::SetSystemTime;
      use winapi::{SYSTEMTIME, WORD};

      let local_date_time = date_time.with_timezone(&Local);
      let mut sys_time: SYSTEMTIME = unsafe { zeroed() };

      let day_of_week = match local_date_time.weekday() {
         Weekday::Mon => 1,
         Weekday::Tue => 2,
         Weekday::Wed => 3,
         Weekday::Thu => 4,
         Weekday::Fri => 5,
         Weekday::Sat => 6,
         Weekday::Sun => 0,
      };

      let mut nano_seconds = local_date_time.nanosecond();
      let is_leap_second = nano_seconds > 1_000_000_000;
      if is_leap_second {
         nano_seconds -= 1_000_000_000;
      }

      sys_time.wYear = local_date_time.year() as WORD;
      sys_time.wMonth = local_date_time.month() as WORD;
      sys_time.wDayOfWeek = day_of_week as WORD;
      sys_time.wDay = local_date_time.day() as WORD;
      sys_time.wHour = local_date_time.hour() as WORD;
      sys_time.wMinute = local_date_time.minute() as WORD;
      sys_time.wSecond = local_date_time.second() as WORD;
      sys_time.wMilliseconds = (nano_seconds / 1_000_000) as WORD;

      let sys_time_ptr = &sys_time as *const SYSTEMTIME;

      unsafe {
         SetSystemTime(sys_time_ptr);
      }
   }

   #[cfg(not(windows))]
   fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
      use libc::{settimeofday, timezone};
      use libc::{suseconds_t, time_t, timeval};

      let t = t.with_timezone(&Local);
      let mut u: timeval = unsafe { zeroed() };

      u.tv_sec = t.timestamp() as time_t;
      u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

      unsafe {
         let mock_tz: *const timezone = std::ptr::null();
         settimeofday(&u as *const timeval, mock_tz);
      }
   }
}

fn main() {
   let app = App::new("clock")
      .version("0.1.2")
      .about("Gets and (aspirationally) sets the time.")
      .after_help(
         "Note: UNIX timestamps are parsed as whole \
       seconds since 1st January 1970 0:00:00 UTC. \
       For more accuracy, use another format.",
      )
      .arg(
         Arg::with_name("action")
            .takes_value(true)
            .possible_values(&["get", "set"])
            .default_value("get"),
      )
      .arg(
         Arg::with_name("std")
            .short("s")
            .long("use-standard")
            .takes_value(true)
            .possible_values(&["rfc2822", "rfc3339", "timestamp"])
            .default_value("rfc3339"),
      )
      .arg(Arg::with_name("datetime").help(
         "When <action> is 'set', apply <datetime>. \
       Otherwise, ignore.",
      ));

   let args = app.get_matches();
   let action = args.value_of("action").unwrap();
   let std = args.value_of("std").unwrap();

   if action == "set" {
      let date_time_ = args.value_of("datetime").unwrap();
      let parser = match std {
         "rfc2822" => DateTime::parse_from_rfc2822,
         "rfc3339" => DateTime::parse_from_rfc3339,
         _ => unimplemented!(),
      };

      let err_msg = format!("Unable to parse {} according to {}", date_time_, std);
      let current_date_time = parser(date_time_).expect(&err_msg);

      Clock::set(current_date_time)
   }

   let now = Clock::get();

   match std {
      "timestamp" => println!("{}", now.timestamp()),
      "rfc2822" => println!("{}", now.to_rfc2822()),
      "rfc3339" => println!("{}", now.to_rfc3339()),
      _ => unreachable!(),
   }
}
