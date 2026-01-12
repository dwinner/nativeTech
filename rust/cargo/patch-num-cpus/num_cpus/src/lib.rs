pub fn add(left: u64, right: u64) -> u64
{
   left + right
}

pub fn get() -> usize
{
   100 // Return some arbitrary value, for test purposes
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
