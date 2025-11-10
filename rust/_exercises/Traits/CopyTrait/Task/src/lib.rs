use std::ops::Add;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct WrappingU32
{
   value: u32,
}

impl WrappingU32
{
   pub const fn new(value: u32) -> Self
   {
      Self { value }
   }
}

impl Add for WrappingU32
{
   type Output = Self;

   fn add(self, rhs: Self) -> Self::Output
   {
      Self::new(self.value.wrapping_add(rhs.value))
   }
}
