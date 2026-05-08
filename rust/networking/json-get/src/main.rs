use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User
{
   login: String,
   id: u32,
   public_repos: u32,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>
{
   let user: User = reqwest::get("https://api.github.com/users/rust-lang")
      .await?
      .json()
      .await?;
   println!("Пользователь: {:?}", user);
   Ok(())
}
