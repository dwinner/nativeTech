use std::fmt::Debug;

static SCREEN_BOUNDS: Bounds = Bounds {
   top_left: Point { x: 0, y: 0 },
   bottom_right: Point { x: 640, y: 480 },
};

fn main()
{
   use_generics();
   use_trait_obj();
   use_bounds();
}

#[derive(Debug, Copy, Clone)]
pub struct Point
{
   x: i64,
   y: i64,
}

#[derive(Debug, Copy, Clone)]
pub struct Bounds
{
   top_left: Point,
   bottom_right: Point,
}

#[derive(Clone)]
struct Square
{
   top_left: Point,
   size: i64,
}

#[derive(Clone, Debug)]
struct Circle
{
   center: Point,
   radius: i64,
}

fn overlap(_a: Bounds, _b: Bounds) -> Option<Bounds>
{
   todo!()
}

pub trait Draw
{
   fn bounds(&self) -> Bounds;
}

impl Draw for Square
{
   fn bounds(&self) -> Bounds
   {
      Bounds {
         top_left: self.top_left,
         bottom_right: Point {
            x: self.top_left.x + self.size,
            y: self.top_left.y + self.size,
         },
      }
   }
}

impl Draw for Circle
{
   fn bounds(&self) -> Bounds
   {
      todo!()
   }
}

// 1. Using generics

pub fn on_screen<T>(draw: &T) -> bool
where
   T: Draw,
{
   overlap(SCREEN_BOUNDS, draw.bounds()).is_some()
}

/*pub fn on_screen_2(draw: &impl Draw)->bool{
   overlap(SCREEN_BOUNDS, draw.bounds()).is_some()
}*/

// area is accessible for all containers implemented `Draw`
fn area<T>(draw: &T) -> i64
where
   T: Draw,
{
   let bounds = draw.bounds();
   (bounds.bottom_right.x - bounds.top_left.x) * (bounds.bottom_right.y - bounds.top_left.y)
}

// `show` ia accessible if trait `Debug` is implemented
fn show<T>(draw: &T)
where
   T: Debug + Draw,
{
   println!("{:?} has bounds {:?}", draw, draw.bounds());
}

fn use_generics()
{
   let square = Square {
      top_left: Point { x: 1, y: 2 },
      size: 2,
   };

   // call `on_screen::<Square>(&Square) -> bool`
   _ = on_screen(&square);

   let circle = Circle {
      center: Point { x: 3, y: 4 },
      radius: 1,
   };

   // call `on_screen::<Circle>(&Circle) -> bool`
   _ = on_screen(&circle);

   // `Square`, `Circle` implement `Draw`
   println!("area of square = {}", area(&square));
   println!("area of circle = {}", area(&circle));

   // `Circle` implements `Debug`
   show(&circle);

   // ... But `Square` doesn't implement `Debug`
   //show(&square);
}

// 2. Using trait objects

pub fn on_screen_dyn(draw: &dyn Draw) -> bool
{
   overlap(SCREEN_BOUNDS, draw.bounds()).is_some()
}

fn use_trait_obj()
{
   let square = Square {
      top_left: Point { x: 1, y: 2 },
      size: 2,
   };

   let circle = Circle {
      center: Point { x: 3, y: 4 },
      radius: 1,
   };

   // call `on_screen(&dyn Draw) -> bool`
   _ = on_screen(&square);

   // also call `on_screen(&dyn Draw) -> bool`
   _ = on_screen(&circle);
}

// 3. Extra bounds of traits

/// All implemented `Shape` should also implement `Draw`
trait Shape: Draw
{
   fn render_in(&self, bounds: Bounds);

   fn render(&self)
   {
      if let Some(visible) = overlap(SCREEN_BOUNDS, self.bounds())
      {
         self.render_in(visible);
      }
   }
}

/// `Stamp` can be copied and draw for several times
trait Stamp: Draw
{
   fn make_copy(&self) -> Self
   where
      Self: Sized;
}

fn use_bounds()
{
   let square = Square {
      top_left: Point { x: 1, y: 2 },
      size: 2,
   };

   let circle = Circle {
      center: Point { x: 3, y: 4 },
      radius: 1,
   };

   let _draw: &dyn Draw = &square;
   // Impossible: let _shape: &dyn Shape = &square;

   // `Square` implements `Stamp`, so it can call `make_copy()`
   //let copy = square.make_copy();

   // Due to `Self` contains `Sized` bounds,
   // it's possible to create trait-object `Stamp`
   //let _stamp: &dyn Stamp = &square;

   // type erasure
   let shapes: Vec<&dyn Draw> = vec![&square, &circle];
   for shape in shapes
   {
      shape.bounds();
   }
}
