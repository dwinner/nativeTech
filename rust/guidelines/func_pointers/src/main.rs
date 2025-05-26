fn sum(x: i32, y: i32) -> i32
{
   x + y
}

fn main()
{
   // explicit cast to 'fn' type
   let op: fn(i32, i32) -> i32 = sum;

   // 'fn' types implement 'Copy'
   let op1 = op;
   let op2 = op;

   // 'fn' types implement 'Eq'
   assert!(op1 == op2);

   // 'fn' implements std::fmt::Pointer in {:p}
   println!("op = {:p}", op);
}
