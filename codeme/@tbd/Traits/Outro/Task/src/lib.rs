use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SaturatingU16
{
   value: u16,
}

impl From<u16> for SaturatingU16
{
   fn from(value: u16) -> Self
   {
      Self { value }
   }
}

impl From<u8> for SaturatingU16
{
   fn from(value: u8) -> Self
   {
      Self { value: value.into() }
   }
}

impl From<&u16> for SaturatingU16
{
   fn from(value: &u16) -> Self
   {
      (*value).into()
   }
}

impl From<&u8> for SaturatingU16
{
   fn from(value: &u8) -> Self
   {
      (*value).into()
   }
}

impl Add for SaturatingU16
{
   type Output = SaturatingU16;

   fn add(self, rhs: Self) -> Self::Output
   {
      self + rhs.value
   }
}

impl Add<&SaturatingU16> for SaturatingU16
{
   type Output = SaturatingU16;

   fn add(self, rhs: &SaturatingU16) -> Self::Output
   {
      self + *rhs
   }
}

impl Add<u16> for SaturatingU16
{
   type Output = Self;

   fn add(self, rhs: u16) -> Self::Output
   {
      let sum = self.value.saturating_add(rhs);
      Self { value: sum }
   }
}

impl Add<&u16> for SaturatingU16
{
   type Output = SaturatingU16;

   fn add(self, rhs: &u16) -> Self::Output
   {
      self + *rhs
   }
}

impl PartialEq<u16> for SaturatingU16
{
   fn eq(&self, other: &u16) -> bool
   {
      self.value == *other
   }
}
