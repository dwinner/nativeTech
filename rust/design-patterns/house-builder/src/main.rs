#[derive(Debug)]
struct House
{
   walls: u32,
   roof: bool,
   windows: u32,
}

struct HouseBuilder
{
   walls: u32,
   roof: bool,
   windows: u32,
}

impl HouseBuilder
{
   fn new() -> Self
   {
      HouseBuilder {
         walls: 4, // Значение по умолчанию
         roof: true,
         windows: 2,
      }
   }

   fn walls(mut self, walls: u32) -> Self
   {
      self.walls = walls;
      self
   }

   fn roof(mut self, has_roof: bool) -> Self
   {
      self.roof = has_roof;
      self
   }

   fn windows(mut self, windows: u32) -> Self
   {
      self.windows = windows;
      self
   }

   fn build(self) -> House
   {
      House {
         walls: self.walls,
         roof: self.roof,
         windows: self.windows,
      }
   }
}

fn main()
{
   let house = HouseBuilder::new().walls(6).windows(4).roof(true).build();
   println!("House: {:?}", house);
}
