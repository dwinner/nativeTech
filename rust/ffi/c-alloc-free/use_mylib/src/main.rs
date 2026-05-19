#[link(name = "libmylib", kind = "raw-dylib")]
unsafe extern "C" {
   fn create_int_array(size: usize) -> *mut i32;
   fn free_int_array(arr: *mut i32);
}

fn main() {
   let size = 5;
   let arr_ptr = unsafe { create_int_array(size) };

   if arr_ptr.is_null() {
      panic!("Failed to allocate memory");
   }

   unsafe {
      for i in 0..size {
         println!("arr[{}] = {}", i, *arr_ptr.add(i));
      }
      
      free_int_array(arr_ptr);
   }
}
