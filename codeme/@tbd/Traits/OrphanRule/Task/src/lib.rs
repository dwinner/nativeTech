// TODO: this is an example of an orphan rule violation.
//  We're implementing a foreign trait (`PartialEq`, from `std`) on
//  a foreign type (`u32`, from `std`).
//  Look at the compiler error to get familiar with what it looks like.
//  Then delete the code below and move on to the next exercise.

struct U32Wrapper
{
   item: u32,
}

impl PartialEq for U32Wrapper
{
   fn eq(&self, other: &Self) -> bool
   {
      self.item == other.item
   }
}
