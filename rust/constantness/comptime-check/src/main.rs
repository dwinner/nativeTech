fn main()
{
   const FACT_5_SAFE: Option<u32> = factorial_safe(5); // Some(120)
   const FACT_100_SAFE: Option<u32> = factorial_safe(100); // None, так как переполнение
   println!("{:?}", FACT_5_SAFE);
   println!("{:?}", FACT_100_SAFE);
}

const fn factorial_safe(n: u32) -> Option<u32>
{
   let mut result: i32 = 1;
   let mut i = 1;
   while i <= n
   {
      match result.checked_mul(i as i32)
      {
         Some(val) => result = val,
         None => return None,
      }

      i += 1;
   }

   Some(result as u32)
}
