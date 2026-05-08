use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::thread;

fn handle_client(mut stream: TcpStream, tx: Sender<String>)
{
   let mut buffer = [0; 1024];
   while match stream.read(&mut buffer)
   {
      Ok(n) if n > 0 =>
      {
         let msg = String::from_utf8_lossy(&buffer[..n]).to_string();
         tx.send(msg).unwrap();
         true
      }
      _ => false,
   }
   {}
}

fn main() -> std::io::Result<()>
{
   let listener = TcpListener::bind("127.0.0.1:8080")?;
   let (tx, rx) = channel::<String>();

   // Поток для сообщений
   let tx_clone = tx.clone();
   thread::spawn(move || {
      for msg in rx
      {
         println!("Сообщение: {}", msg);
      }
   });

   for stream in listener.incoming()
   {
      let stream = stream?;
      let tx = tx_clone.clone();
      thread::spawn(move || handle_client(stream, tx));
   }
   Ok(())
}
