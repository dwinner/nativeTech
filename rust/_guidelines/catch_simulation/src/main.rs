fn main()
{
   let var = divide_recover(1, 0, 0);
   println!("{}", var);
}

fn divide(a: i64, b: i64) -> i64
{
   if b == 0
   {
      panic!("Divide by zero");
   }

   a / b
}

fn divide_recover(a: i64, b: i64, default: i64) -> i64
{
   let result = std::panic::catch_unwind(|| divide(a, b));
   result.unwrap_or_else(|_| default)
}
