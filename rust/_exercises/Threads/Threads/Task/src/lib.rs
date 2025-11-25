use std::thread;

pub fn sum(a_vector: Vec<i32>) -> i32
{
   let mid = a_vector.len() / 2;
   let (half_1st, half_2nd) = a_vector.split_at(mid);
   let half_1st = half_1st.to_vec();
   let half_2nd = half_2nd.to_vec();

   let handle1 = thread::spawn(move || half_1st.into_iter().sum::<i32>());
   let handle2 = thread::spawn(move || half_2nd.into_iter().sum::<i32>());

   handle1.join().unwrap() + handle2.join().unwrap()
}
