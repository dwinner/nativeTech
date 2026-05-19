use std::alloc::{Layout, alloc, dealloc};

fn main() {
   let layout = Layout::new::<i32>(); // Layout для i32 (размер 4, выравнивание 4)
   let ptr = unsafe { alloc(layout) } as *mut i32;

   if ptr.is_null() {
      panic!("Allocation failed");
   }

   unsafe {
      *ptr = 42;
      println!("Value: {}", *ptr);
      dealloc(ptr as *mut u8, layout);
   }
}
