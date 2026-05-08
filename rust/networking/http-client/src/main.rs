use reqwest;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error>
{
   let response = reqwest::get("https://api.github.com/users/rust-lang")
      .await?
      .text()
      .await?;
   println!("Ответ: {}", response);
   Ok(())
}
