fn main()
{
   let rect = Rect::new(7.9, 9.0);
   let area = rect.area();
   println!("Area: {}", area);
}

struct Rect
{
   width: f64,
   height: f64,
}

impl Rect
{
   fn new(width: f64, height: f64) -> Self
   {
      Self { width, height }
   }

   fn area(&self) -> f64
   {
      self.width * self.height
   }
}
