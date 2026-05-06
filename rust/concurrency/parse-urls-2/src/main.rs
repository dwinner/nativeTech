use reqwest::blocking::get;
use reqwest::Error;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

fn fetch_url(url: &str) -> Result<usize, Error>
{
   let response = get(url)?;
   Ok(response.text()?.len())
}

fn main()
{
   let urls = vec![
      "https://www.rust-lang.org",
      "https://www.google.com",
      "https://www.github.com",
   ];

   let results = Arc::new(Mutex::new(Vec::new()));
   let (tx, rx) = mpsc::channel();
   let mut handles = vec![];

   for url in urls
   {
      let tx = tx.clone();
      let url = url.to_string();
      handles.push(thread::spawn(move || match fetch_url(&url)
      {
         Ok(len) => tx.send((url, len)).unwrap(),
         Err(err) => eprintln!("Error for {}: {}", url, err),
      }));
   }

   // Закрываем ящик после отправки
   drop(tx);

   for handle in handles
   {
      handle.join().unwrap();
   }

   while let Ok((url, len)) = rx.recv()
   {
      let mut res = results.lock().unwrap();
      res.push((url, len));
   }

   for (url, len) in results.lock().unwrap().iter()
   {
      println!("Page: {}, Length: {}", url, len);
   }
}
