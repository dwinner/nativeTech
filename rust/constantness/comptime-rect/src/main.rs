fn main()
{
   const RECT: Rectangle = Rectangle::new(3, 4);
   const AREA: u32 = RECT.area(); // 12
   const PERIMETER: u32 = RECT.perimeter(); // 14

   println!("Pre-computed area: {}", AREA);
   println!("Pre-computed perimeter: {}", PERIMETER);
}

struct Rectangle
{
   width: u32,
   height: u32,
}

impl Rectangle
{
   const fn new(width: u32, height: u32) -> Self
   {
      Self { width, height }
   }

   const fn area(&self) -> u32
   {
      self.width * self.height
   }

   const fn perimeter(&self) -> u32
   {
      2 * (self.width + self.height)
   }
}
