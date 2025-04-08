struct Point<X1, Y1>
{
   x: X1,
   y: Y1,
}

impl<X1, Y1> Point<X1, Y1>
{
   fn mix_up<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2>
   {
      Point { x: self.x, y: other.y }
   }
}

fn main()
{
   let p1 = Point { x: 5, y: 10.4 };
   let p2 = Point { x: "Hello", y: 'c' };

   let p3 = p1.mix_up(p2);

   println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
