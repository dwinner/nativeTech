use std::ops::Deref;

struct Wrapper<T>(T);

impl<T> From<T> for Wrapper<T>
{
   fn from(value: T) -> Self
   {
      Wrapper(value)
   }
}

/*
impl<T> Into<T> for Wrapper<T>
{
   fn into(self) -> T
   {
      self.0 // Возвращаем внутреннее значение, потребляя Wrapper
   }
}
*/

impl<T> Deref for Wrapper<T>
{
   type Target = T;

   fn deref(&self) -> &Self::Target
   {
      &self.0 // Возвращаем ссылку на внутреннее значение
   }
}

fn main()
{
   let wrapper = i32::from(42) /*Wrapper(42)*/;
   let x: i32 = wrapper.into(); // Преобразование в i32
   println!("x = {}", x); // Вывод: x = 42

   let w = Wrapper(42);
   println!("*w = {}", *w); // Разыменование
   assert_eq!(42, *w);
}
