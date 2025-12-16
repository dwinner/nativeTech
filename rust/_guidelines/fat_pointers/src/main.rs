use std::rc::Rc;

fn main()
{
   // slices are fat
   let array: [u64; 5] = [0, 1, 2, 3, 4];
   let array_slice = &array[1..3];
   println!("{:?}", array_slice);

   let mut vector = Vec::<u64>::with_capacity(8);
   for element in 0..5
   {
      vector.push(element);
   }

   let vec_slice = &vector[1..3];
   println!("{:?}", vec_slice);

   // trait objects are also fat
   let mod3 = Modulo(3);

   // you should use explicit type for dynamic dispatch
   let t_obj: &dyn Calculate = &mod3;
   let add_result = t_obj.add(2, 2);
   assert_eq!(add_result, 1);
   let mul_result = t_obj.mul(2, 2);
   assert_eq!(mul_result, 1);

   // Borrow<T> is also fat
   assert_eq!(add_four(&2), 6);
   assert_eq!(add_four(2), 6);

   // smart pointers are fat
   let rc1: Rc<u64> = Rc::new(42);
   let rc2 = rc1.clone();
   let weak = Rc::downgrade(&rc1);
   println!("{:p}", rc2);
   println!("{:p}", weak.as_ptr());
}

trait Calculate
{
   fn add(&self, left: u64, right: u64) -> u64;

   fn mul(&self, left: u64, right: u64) -> u64;
}

struct Modulo(pub u64);

impl Calculate for Modulo
{
   fn add(&self, left: u64, right: u64) -> u64
   {
      (left + right) % self.0
   }

   fn mul(&self, left: u64, right: u64) -> u64
   {
      (left * right) % self.0
   }
}

fn add_four<T: std::borrow::Borrow<i32>>(val: T) -> i32
{
   val.borrow() + 4
}
