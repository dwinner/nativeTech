// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person
{
   name: String,
   age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point
{
   x: f32,
   y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle
{
   // A rectangle can be specified by where the top left and bottom right
   // corners are in space.
   top_left: Point,
   bottom_right: Point,
}

fn main()
{
   // Create struct with field init shorthand
   let name = String::from("Peter");
   let age = 27;
   let peter = Person { name, age };

   // Print debug struct
   println!("{:?}", peter);

   // Instantiate a `Point`
   let point: Point = Point { x: 5.2, y: 0.4 };
   let another_point: Point = Point { x: 10.3, y: 0.2 };

   // Access the fields of the point
   println!("point coordinates: ({}, {})", point.x, point.y);

   // Make a new point by using struct update syntax to use the fields of our
   // other one
   let bottom_right = Point {
      x: 10.3,
      ..another_point
   };

   // `bottom_right.y` will be the same as `another_point.y` because we used that field
   // from `another_point`
   println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

   // Destructure the point using a `let` binding
   let Point {
      x: left_edge,
      y: top_edge,
   } = point;

   let _rectangle = Rectangle {
      // struct instantiation is an expression too
      top_left: Point {
         x: left_edge,
         y: top_edge,
      },
      bottom_right,
   };

   // Instantiate a unit struct
   let _unit = Unit;

   // Instantiate a tuple struct
   let pair = Pair(1, 0.1);

   // Access the fields of a tuple struct
   println!("pair contains {:?} and {:?}", pair.0, pair.1);

   // Destructure a tuple struct
   let Pair(integer, decimal) = pair;

   println!("pair contains {:?} and {:?}", integer, decimal);

   let rect = Rectangle {
      top_left: Point { x: 0f32, y: 0f32 },
      bottom_right: Point { x: 10f32, y: 10f32 },
   };
   println!("Area of (0;0-10;10) rect is {}", rect_area(rect));
   println!("Calculated rect is {:?}", square(&point, 10f32));
}

fn rect_area(rect: Rectangle) -> f32
{
   let Point { x: x_left, y: y_top } = rect.top_left;
   let Point {
      x: x_right,
      y: y_bottom,
   } = rect.bottom_right;

   let height = y_bottom - y_top;
   let width = x_right - x_left;

   f32::abs(height * width)
}

fn square(point: &Point, dim: f32) -> Rectangle
{
   Rectangle {
      top_left: Point { x: point.x, y: point.y },
      bottom_right: Point {
         x: point.x + dim,
         y: point.y + dim,
      },
   }
}
