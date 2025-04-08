pub trait Summary
{
   fn summarize_author(&self) -> String;

   fn summarize(&self) -> String
   {
      format!("(Read more from {}...)", self.summarize_author())
   }
}

pub struct SocialPost
{
   pub username: String,
   pub content: String,
   pub reply: bool,
   pub repost: bool,
}

impl Summary for SocialPost
{
   fn summarize_author(&self) -> String
   {
      format!("@{}", self.username)
   }
}
