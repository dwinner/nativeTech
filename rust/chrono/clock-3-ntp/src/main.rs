#[cfg(windows)]
use kernel32;

#[cfg(not(windows))]
use libc;

#[cfg(windows)]
use winapi;

use byteorder::{BigEndian, ReadBytesExt};
use chrono::{DateTime, Datelike, Duration as ChronoDuration, TimeZone, Timelike};
use chrono::{Local, Utc};
use clap::{App, Arg};
use std::mem::zeroed;
use std::net::UdpSocket;
use std::time::Duration;

const NTP_MESSAGE_LENGTH: usize = 48;
const NTP_TO_UNIX_SECONDS: i64 = 2_208_988_800;
const LOCAL_ADDR: &'static str = "0.0.0.0:12300";

#[derive(Default, Debug, Copy, Clone)]
struct NTPTimestamp {
   seconds: u32,
   fraction: u32,
}

struct NTPMessage {
   data: [u8; NTP_MESSAGE_LENGTH],
}

#[derive(Debug)]
struct NTPResult {
   t1: DateTime<Utc>,
   t2: DateTime<Utc>,
   t3: DateTime<Utc>,
   t4: DateTime<Utc>,
}

impl NTPResult {
   fn offset(&self) -> i64 {
      let delta = self.delay();
      delta.abs() / 2
   }

   fn delay(&self) -> i64 {
      let duration = (self.t4 - self.t1) - (self.t3 - self.t2);
      duration.num_milliseconds()
   }
}

impl From<NTPTimestamp> for DateTime<Utc> {
   fn from(ntp: NTPTimestamp) -> Self {
      let secs = ntp.seconds as i64 - NTP_TO_UNIX_SECONDS;
      let mut nanos = ntp.fraction as f64;
      nanos *= 1e9;
      nanos /= 2_f64.powi(32);

      Utc.timestamp_opt(secs, nanos as u32).unwrap()
   }
}

impl From<DateTime<Utc>> for NTPTimestamp {
   fn from(utc: DateTime<Utc>) -> Self {
      let secs = utc.timestamp() + NTP_TO_UNIX_SECONDS;
      let mut fraction = utc.nanosecond() as f64;
      fraction *= 2_f64.powi(32);
      fraction /= 1e9;

      NTPTimestamp {
         seconds: secs as u32,
         fraction: fraction as u32,
      }
   }
}

impl NTPMessage {
   fn new() -> Self {
      NTPMessage {
         data: [0; NTP_MESSAGE_LENGTH],
      }
   }

   fn client() -> Self {
      const VERSION: u8 = 0b00_011_000;
      const MODE: u8 = 0b00_000_011;

      let mut msg = NTPMessage::new();

      msg.data[0] |= VERSION;
      msg.data[0] |= MODE;
      msg
   }

   fn parse_timestamp(&self, i: usize) -> Result<NTPTimestamp, std::io::Error> {
      let mut reader = &self.data[i..i + 8];
      let seconds = reader.read_u32::<BigEndian>()?;
      let fraction = reader.read_u32::<BigEndian>()?;

      Ok(NTPTimestamp { seconds, fraction })
   }

   fn rx_time(&self) -> Result<NTPTimestamp, std::io::Error> {
      self.parse_timestamp(32)
   }

   fn tx_time(&self) -> Result<NTPTimestamp, std::io::Error> {
      self.parse_timestamp(40)
   }
}

fn weighted_mean(values: &[f64], weights: &[f64]) -> f64 {
   let mut result = 0.0;
   let mut sum_of_weights = 0.0;

   for (value, weight) in values.iter().zip(weights) {
      result += value * weight;
      sum_of_weights += weight;
   }

   result / sum_of_weights
}

fn ntp_round_trip(host: &str, port: u16) -> Result<NTPResult, std::io::Error> {
   let destination = format!("{}:{}", host, port);
   let timeout = Duration::from_secs(1);
   let request = NTPMessage::client();
   let mut response = NTPMessage::new();
   let message = request.data;
   let udp = UdpSocket::bind(LOCAL_ADDR)?;
   udp.connect(&destination).expect("unable to connect");
   let t1 = Utc::now();

   udp.send(&message)?;
   udp.set_read_timeout(Some(timeout))?;
   udp.recv_from(&mut response.data)?;
   let t4 = Utc::now();
   let t2: DateTime<Utc> = response.rx_time()?.into();
   let t3: DateTime<Utc> = response.tx_time()?.into();

   Ok(NTPResult { t1, t2, t3, t4 })
}

fn check_time() -> Result<f64, std::io::Error> {
   const NTP_PORT: u16 = 123;

   let servers = [
      "time.nist.gov",
      "time.apple.com",
      "time.euro.apple.com",
      "time.google.com",
      "time2.google.com",
      //"time.windows.com",
   ];

   let mut times = Vec::with_capacity(servers.len());

   for &server in servers.iter() {
      print!("{} =>", server);
      let calc = ntp_round_trip(&server, NTP_PORT);
      match calc {
         Ok(time) => {
            println!(" {}ms away from local system time", time.offset());
            times.push(time);
         }
         Err(_) => {
            println!(" ? [response took too long]")
         }
      };
   }

   let mut offsets = Vec::with_capacity(servers.len());
   let mut offset_weights = Vec::with_capacity(servers.len());

   for time in &times {
      let offset = time.offset() as f64;
      let delay = time.delay() as f64;
      let weight = 1_000_000.0 / (delay * delay);
      if weight.is_finite() {
         offsets.push(offset);
         offset_weights.push(weight);
      }
   }

   let avg_offset = weighted_mean(&offsets, &offset_weights);

   Ok(avg_offset)
}

struct Clock;

impl Clock {
   fn get() -> DateTime<Local> {
      Local::now()
   }

   #[cfg(windows)]
   fn set<Tz: TimeZone>(a_date_time: DateTime<Tz>) -> () {
      use chrono::Weekday;
      use kernel32::SetSystemTime;
      use winapi::{SYSTEMTIME, WORD};

      let local_date_time = a_date_time.with_timezone(&Local);
      let mut sys_time: SYSTEMTIME = unsafe { zeroed() };

      let dow = match local_date_time.weekday() {
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
      sys_time.wDayOfWeek = dow as WORD;
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
      use libc::settimeofday;
      use libc::{suseconds_t, time_t, timeval, timezone};

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
      .version("0.1.3")
      .about("Gets and sets the time.")
      .after_help(
         "Note: UNIX timestamps are parsed as whole seconds since 1st \
       January 1970 0:00:00 UTC. For more accuracy, use another \
       format.",
      )
      .arg(
         Arg::with_name("action")
            .takes_value(true)
            .possible_values(&["get", "set", "check-ntp"])
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
      .arg(Arg::with_name("datetime").help("When <action> is 'set', apply <datetime>. Otherwise, ignore."));

   let args = app.get_matches();
   let action = args.value_of("action").unwrap();
   let std = args.value_of("std").unwrap();

   if action == "set" {
      let date_time_str = args.value_of("datetime").unwrap();
      let parser = match std {
         "rfc2822" => DateTime::parse_from_rfc2822,
         "rfc3339" => DateTime::parse_from_rfc3339,
         _ => unimplemented!(),
      };

      let err_msg = format!("Unable to parse {} according to {}", date_time_str, std);
      let parsed_date_time = parser(date_time_str).expect(&err_msg);

      Clock::set(parsed_date_time);
   } else if action == "check-ntp" {
      let offset = check_time().unwrap() as isize;
      let adjust_ms_ = offset.signum() * offset.abs().min(200) / 5;
      let adjust_ms = ChronoDuration::milliseconds(adjust_ms_ as i64);
      let now: DateTime<Utc> = Utc::now() + adjust_ms;

      Clock::set(now);
   }

   let maybe_error = std::io::Error::last_os_error();
   let os_error_code = &maybe_error.raw_os_error();

   match os_error_code {
      Some(0) => (),
      Some(_) => eprintln!("Unable to set the time: {:?}", maybe_error),
      None => (),
   }

   let now = Clock::get();

   match std {
      "timestamp" => println!("{}", now.timestamp()),
      "rfc2822" => println!("{}", now.to_rfc2822()),
      "rfc3339" => println!("{}", now.to_rfc3339()),
      _ => unreachable!(),
   }
}
