use std::fmt::Debug;

trait Swappable
{
   fn swap(&mut self);
}

#[derive(Debug)]
struct Pair<T>
{
   first: T,
   second: T,
}

impl<T: Debug> Swappable for Pair<T>
{
   fn swap(&mut self)
   {
      std::mem::swap(&mut self.first, &mut self.second);
   }
}

fn main()
{
   let mut pair = Pair { first: 42, second: 24 };
   println!("Before: {:?}", pair); // Before: Pair { first: 42, second: "hello" }
   pair.swap();
   println!("After: {:?}", pair); // After: Pair { first: "hello", second: 42 }
}
