// Given a vector of integers, leak its heap allocation.
//  Then split the resulting static slice into two halves and
//  sum each half in a separate thread.
//  Hint: check out `Vec::leak`.

use std::thread;

pub fn sum(vector: Vec<i32>) -> i32
{
   let leak_vec = vector.leak();
   let mid = leak_vec.len() / 2;
   let (vec1, vec2) = leak_vec.split_at(mid);

   let handle1 = thread::spawn(move || vec1.into_iter().sum::<i32>());
   let handle2 = thread::spawn(move || vec2.into_iter().sum::<i32>());

   handle1.join().unwrap() + handle2.join().unwrap()
}
