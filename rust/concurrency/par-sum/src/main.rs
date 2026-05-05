use std::sync::{Arc, Mutex};
use std::thread;

fn main()
{
   let ingredients = vec![1, 2, 3, 4, 5, 6, 7, 8];
   let chunk_size = ingredients.len() / 4;
   let total_weight = Arc::new(Mutex::new(0));
   let mut handles = vec![];

   for chunk in ingredients.chunks(chunk_size)
   {
      let chunk = chunk.to_vec();
      let total_weight = Arc::clone(&total_weight);
      handles.push(thread::spawn(move || {
         let sum: i32 = chunk.iter().sum();
         let mut total = total_weight.lock().unwrap();
         *total += sum;
      }));
   }

   for handle in handles
   {
      handle.join().unwrap();
   }

   println!("Total weight: {}", *total_weight.lock().unwrap()); // 36
}
