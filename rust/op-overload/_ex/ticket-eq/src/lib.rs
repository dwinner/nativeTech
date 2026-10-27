use std::cmp::PartialEq;

pub struct Ticket
{
   title: String,
   description: String,
   status: String,
}

impl Ticket
{
   pub fn new(title: String, description: String, status: String) -> Ticket
   {
      Ticket {
         title,
         description,
         status,
      }
   }
}

// Implement the `PartialEq` trait for `Ticket`.
impl PartialEq for Ticket
{
   fn eq(&self, other: &Self) -> bool
   {
      self.title == other.title && self.status == other.status && self.description == other.description
   }
}
