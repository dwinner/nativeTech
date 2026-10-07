// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 bytes.
//   Implement the traits required to make the tests pass too.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TicketNewError
{
   #[error("The title cannot be empty")]
   TitleCannotBeEmpty,

   #[error("The title cannot be longer than 50 bytes")]
   TitleTooLong,
}

/* TODO */
#[derive(Debug, PartialEq, Clone)]
pub struct TicketTitle(String);

impl TicketTitle
{
   pub fn value(&self) -> &str
   {
      &self.0
   }
}

/* TODO */
impl TryFrom<&str> for TicketTitle
{
   type Error = TicketNewError;

   fn try_from(value: &str) -> Result<Self, Self::Error>
   {
      if value.is_empty()
      {
         return Err(TicketNewError::TitleCannotBeEmpty);
      }

      if value.len() > 50
      {
         return Err(TicketNewError::TitleTooLong);
      }

      Ok(TicketTitle(value.to_string()))
   }
}

impl TryFrom<String> for TicketTitle
{
   type Error = TicketNewError;

   fn try_from(value: String) -> Result<Self, Self::Error>
   {
      value.as_str().try_into()
   }
}
