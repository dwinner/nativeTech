enum Error
{
   DivisionByZero,
   NegativeResult,
}

fn divide(a: i32, b: i32) -> Result<i32, Error>
{
   if b == 0
   {
      return Err(Error::DivisionByZero);
   }

   let result = a / b;
   if result < 0
   {
      return Err(Error::NegativeResult);
   }

   Ok(result)
}

fn main()
{
   match divide(10, 2)
   {
      Ok(result) =>
      {
         println!("Result: {}", result)
      }
      Err(Error::DivisionByZero) =>
      {
         println!("Error: Division by zero")
      }
      Err(Error::NegativeResult) =>
      {
         println!("Error: Negative result")
      }
   }

   match divide(10, 0)
   {
      Ok(result) =>
      {
         println!("Result: {}", result)
      }
      Err(Error::DivisionByZero) =>
      {
         println!("Error: Division by zero")
      }
      Err(Error::NegativeResult) =>
      {
         println!("Error: Negative result")
      }
   }
}
