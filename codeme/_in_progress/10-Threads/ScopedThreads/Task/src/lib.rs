// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.

pub fn sum(vector: Vec<i32>) -> i32
{
   /* TODO */
   let mid = vector.len() / 2;
   let (left, right) = vector.split_at(mid);

   std::thread::scope(|scoped| {
      let left = scoped.spawn(|| left.iter().sum::<i32>());
      let right = scoped.spawn(|| right.iter().sum::<i32>());

      left.join().unwrap() + right.join().unwrap()
   })
}
