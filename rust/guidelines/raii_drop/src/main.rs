use std::sync::Mutex;

fn main()
{
   {
      let int_val = TSInt::new(42);
      int_val.add(3);
      println!("{int_val:?}");
   }
}

#[derive(Debug)]
struct TSInt
{
   value: Mutex<i32>,
}

impl TSInt
{
   fn new(val: i32) -> Self
   {
      Self { value: Mutex::new(val) }
   }

   fn add(&self, delta: i32)
   {
      let mut ts_val = self.value.lock().unwrap();
      *ts_val += delta;
   }
}

impl Drop for TSInt
{
   fn drop(&mut self)
   {
      println!("Dropping {self:?}");
      // code to release resources there
   }
}
