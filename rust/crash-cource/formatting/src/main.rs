use std::fmt::{self, Display, Formatter};

struct City
{
   name: &'static str,
   // Latitude
   lat: f32,
   // Longitude
   lon: f32,
}

impl Display for City
{
   // `f` is a buffer, and this method must write the formatted string into it.
   fn fmt(&self, f: &mut Formatter) -> fmt::Result
   {
      let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
      let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

      // `write!` is like `format!`, but it will write the formatted string
      // into a buffer (the first argument).
      write!(
         f,
         "{}: {:.3}°{} {:.3}°{}",
         self.name,
         self.lat.abs(),
         lat_c,
         self.lon.abs(),
         lon_c
      )
   }
}

#[derive(Debug)]
struct Color
{
   red: u8,
   green: u8,
   blue: u8,
}

impl Display for Color
{
   fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
   {
      let red_val = self.red as u32;
      let green_val = self.green as u32;
      let blue_val = self.blue as u32;
      let rgb_value: u32 = red_val * 0x10000 + green_val * 0x100 + blue_val;
      write!(
         f,
         "RGB ({}, {}, {}) 0x{:0>6X}",
         self.red, self.green, self.blue, rgb_value
      )
   }
}

fn main()
{
   for city in [
      City {
         name: "Dublin",
         lat: 53.347778,
         lon: -6.259722,
      },
      City {
         name: "Oslo",
         lat: 59.95,
         lon: 10.75,
      },
      City {
         name: "Vancouver",
         lat: 49.25,
         lon: -123.1,
      },
   ]
   {
      println!("{}", city);
   }

   for color in [
      Color {
         red: 128,
         green: 255,
         blue: 90,
      },
      Color {
         red: 0,
         green: 3,
         blue: 254,
      },
      Color {
         red: 0,
         green: 0,
         blue: 0,
      },
   ]
   {
      // Switch this to use {} once you've added an implementation
      // for fmt::Display.
      println!("{}", color);
   }
}
