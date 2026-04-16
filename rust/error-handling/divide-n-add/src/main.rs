fn divide(num1: i32, num2: i32) -> Result<i32, String>
{
   if num2 == 0
   {
      Err(String::from("Division by zero"))
   }
   else
   {
      Ok(num1 / num2)
   }
}

fn divide_and_add(num1: i32, num2: i32) -> Result<i32, String>
{
   let result = divide(num1, num2)?;
   Ok(result + 5)
}

fn main()
{
   match divide_and_add(10, 2)
   {
      Ok(val) =>
      {
         println!("Result: {}", val)
      }
      Err(err) =>
      {
         println!("Error: {}", err)
      }
   }

   match divide_and_add(10, 0)
   {
      Ok(val) =>
      {
         println!("Result: {}", val)
      }
      Err(err) =>
      {
         println!("Error: {}", err)
      }
   }
}
