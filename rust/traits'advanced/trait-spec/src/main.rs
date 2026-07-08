#![feature(specialization)]

trait Addable
{
   fn add(&self, other: &Self) -> Self;
}

impl<T> Addable for T
where
   T: std::ops::Add<Output = T>,
{
   default fn add(&self, other: &T) -> T
   {
      self + other
   }
}

impl Addable for String
{
   fn add(&self, other: &String) -> String
   {
      let mut result = self.clone();
      result.push_str(other);
      result
   }
}

fn main()
{
   let a = 5;
   let b = 3;
   println!("i32: {}", a.add(&b)); // Вывод: 8

   let s1 = String::from("Hello, ");
   let s2 = String::from("world!");
   println!("String: {}", s1.add(&s2)); // Вывод: Hello, world!
}
