fn compute(n: i32) -> i32 {
   let mut result = 0;
   for i in 0..n {
      result += i * i;
   }
   result
}

fn main() {
   let mut total = 0;
   for _ in 0..1000 {
      total += compute(1000);
   }
   println!("Total: {}", total);
}
