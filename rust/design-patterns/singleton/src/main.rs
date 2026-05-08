use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

struct Singleton
{
   value: i32,
}

impl Singleton
{
   fn new() -> Self
   {
      Singleton { value: 42 }
   }

   fn get_value(&self) -> i32
   {
      self.value
   }
}

// Ленивая инициализация Singleton
lazy_static! {
   static ref INSTANCE: Arc<Mutex<Singleton>> = Arc::new(Mutex::new(Singleton::new()));
}

fn main()
{
   let instance = INSTANCE.lock().unwrap();
   println!("Singleton value: {}", instance.get_value());
}
