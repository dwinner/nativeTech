fn main() {
   let mut sum = 0;
   for i in 0..100_000_000 {
      sum += i;
   }

   println!("Sum: {}", sum);
}
