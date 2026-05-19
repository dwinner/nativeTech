use std::alloc::{Layout, alloc, dealloc};

pub struct MyVec<T> {
   ptr: *mut T,
   len: usize,
   capacity: usize,
}

impl<T> MyVec<T> {
   pub fn new() -> Self {
      MyVec {
         ptr: std::ptr::null_mut(),
         len: 0,
         capacity: 0,
      }
   }

   pub fn push(&mut self, value: T) {
      if self.len == self.capacity {

         // Логика перевыделения памяти (упрощено)
         self.capacity = if self.capacity == 0 { 1 } else { self.capacity * 2 };
         let layout = Layout::array::<T>(self.capacity).unwrap();
         let new_ptr = unsafe { alloc(layout) } as *mut T;

         if new_ptr.is_null() {
            panic!("Allocation failed");
         }

         if !self.ptr.is_null() {
            unsafe {
               std::ptr::copy_nonoverlapping(self.ptr, new_ptr, self.len);
               dealloc(self.ptr as *mut u8, Layout::array::<T>(self.capacity / 2).unwrap());
            }
         }

         self.ptr = new_ptr;
      }

      unsafe {
         self.ptr.add(self.len).write(value);
         self.len += 1;
      }
   }

   pub fn get(&self, index: usize) -> Option<&T> {
      if index < self.len {
         unsafe { Some(&*self.ptr.add(index)) }
      } else {
         None
      }
   }
}

impl<T> Drop for MyVec<T> {
   fn drop(&mut self) {
      if !self.ptr.is_null() {
         unsafe {
            dealloc(self.ptr as *mut u8, Layout::array::<T>(self.capacity).unwrap());
         }
      }
   }
}

fn main() {
   let mut vec = MyVec::new();
   vec.push(1);
   vec.push(2);
   println!("vec[0] = {:?}", vec.get(0)); // Some(1)
}
