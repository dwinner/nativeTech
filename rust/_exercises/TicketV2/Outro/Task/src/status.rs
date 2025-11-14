// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

use thiserror::Error;

/* TODO */
#[derive(Debug, PartialEq, Clone)]
pub enum Status
{
   ToDo,
   InProgress,
   Done,
}

/* TODO */
#[derive(Debug, Error)]
#[error("{invalid_status} is not a valid status")]
pub struct ParseStatusError
{
   invalid_status: String,
}

impl TryFrom<&str> for Status
{
   type Error = ParseStatusError;

   fn try_from(value: &str) -> Result<Self, Self::Error>
   {
      match value.to_lowercase().as_str()
      {
         "todo" => Ok(Status::ToDo),
         "inprogress" => Ok(Status::InProgress),
         "done" => Ok(Status::Done),
         _ => Err(ParseStatusError {
            invalid_status: value.to_string(),
         }),
      }
   }
}

impl TryFrom<String> for Status
{
   type Error = ParseStatusError;

   fn try_from(value: String) -> Result<Self, Self::Error>
   {
      value.as_str().try_into()
   }
}
