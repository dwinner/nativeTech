use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
   let urls = vec!["https://www.rust-lang.org", "https://www.tokio.rs"];
   let mut handles = vec![];

   for url in urls
   {
      let handle = tokio::spawn(async move {
         let resp = reqwest::get(url).await?.text().await?;
         println!("Fetched {}: {} bytes", url, resp.len());
         Ok::<_, reqwest::Error>(())
      });
      handles.push(handle);
   }

   for handle in handles
   {
      handle.await??;
   }

   Ok(())
}
