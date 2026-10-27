#![feature(specialization)]

trait Container
{
   type Item;
   fn get(&self) -> Self::Item;
}

impl<T> Container for Vec<T>
where
   T: Clone,
{
   type Item = T;
   fn get(&self) -> T
   {
      self[0].clone()
   }
}

impl Container for Vec<i32>
{
   type Item = i32;
   fn get(&self) -> i32
   {
      self.iter().sum()
   }
}

fn main()
{
   let vec1 = vec![1, 2, 3];
   println!("Vec<i32>: {}", vec1.get()); // Вывод: 6

   let vec2 = vec!["a", "b", "c"];
   println!("Vec<&str>: {}", vec2.get()); // Вывод: a
}
