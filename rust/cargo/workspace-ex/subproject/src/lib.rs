pub fn add(left: u64, right: u64) -> u64
{
   left + right
}

pub fn hello_world() -> String
{
   String::from("Hello, world!")
}

#[cfg(test)]
mod tests
{
   use super::*;

   #[test]
   fn it_works()
   {
      let result = add(2, 2);
      assert_eq!(result, 4);
   }
}
