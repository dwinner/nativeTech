fn main()
{
   let base = 7.5;
   let height = 9.0;
   println!("Triangle area is {}", calc_triangle_area(&base, &height));

   let sum_fn = |num1: i32, num2: i32| -> i32 { num1 + num2 };
   apply_fn(sum_fn, 2, 4);
}

fn calc_triangle_area(base: &f32, height: &f32) -> f64
{
   (0.5 * base * height) as f64
}

fn apply_fn<TFn>(callback_fn: TFn, num1: i32, num2: i32)
where
   TFn: FnOnce(i32, i32) -> i32,
{
   println!("Result: {}", callback_fn(num1, num2));
}
