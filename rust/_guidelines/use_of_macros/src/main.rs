/// Retrieves copies of all values for concrete enum variant
#[macro_export]
macro_rules! values_of_type
{
    { $values:expr, $variant:ident } =>
    {
       {
          let mut result=Vec::new();
          for val in $values
          {
             if let Multi::$variant(v) = val
             {
                result.push(v.clone());
             }
          }

          result
       }
    }
}

#[macro_export]
macro_rules! log_failure {
   { $e:expr } =>
   {
      {
         let result = $e;
         if let Err(err) = &result
         {
            eprintln!("{}:{}: operation '{}' failed: {:?}",
                    file!(),
                    line!(),
                    stringify!($e),
                    err);
         }

         result
      }
   }
}

fn main()
{
   let values = vec![
      Multi::Byte(1),
      Multi::Int(1000),
      Multi::Str("a string".to_string()),
      Multi::Byte(2),
   ];

   let ints = values_of_type!(&values, Int);
   println!("Integer values: {ints:?}");

   let bytes = values_of_type!(&values, Byte);
   println!("Byte values: {bytes:?}");

   // trace ex.
   let _x: Result<u8,_> = log_failure!(512.try_into());  // too big for `u8`
   let _y = log_failure!(std::str::from_utf8(b"\xc3\x28")); // invalid UTF-8
}

enum Multi
{
   Byte(u8),
   Int(i32),
   Str(String),
}
