fn main()
{
   use std::slice;

   let address = 0x01234usize;
   let r = address as *mut i32;

   let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10_000) };

   println!("{:?}", values);
}
