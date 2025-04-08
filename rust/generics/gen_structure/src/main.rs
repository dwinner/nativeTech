struct Point<T>
{
   x: T,
   y: T,
}

impl<T> Point<T>
{
   pub fn new(x: T, y: T) -> Self
   {
      Self { x, y }
   }
}

impl<T> Point<T>
{
   fn x(&self) -> &T
   {
      &self.x
   }
}

// Acts like template specialization in C++
impl Point<f32>
{
   fn distance_from_origin(&self) -> f32
   {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
   }
}

fn main()
{
   let p = Point { x: 5, y: 10 };
   println!("p.x = {}", p.x());

   let flt_point = Point::new(1.1f32, 2.2f32);
   println!("Distance = {}", flt_point.distance_from_origin());
}
