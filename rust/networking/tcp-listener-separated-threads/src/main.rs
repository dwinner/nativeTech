use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()>
{
   println!("Новое соединение: {}", stream.peer_addr()?);
   let mut buffer = [0; 1024];
   stream.read(&mut buffer)?;
   println!("Получено: {}", String::from_utf8_lossy(&buffer[..]));
   stream.write_all(b"Hello from server!")?;

   Ok(())
}

fn main() -> std::io::Result<()>
{
   let listener = TcpListener::bind("127.0.0.1:8080")?;
   println!("Сервер запущен на 127.0.0.1:8080");

   for stream in listener.incoming()
   {
      match stream
      {
         Ok(stream) =>
         {
            thread::spawn(|| {
               if let Err(err) = handle_client(stream)
               {
                  eprintln!("Ошибка в потоке: {}", err);
               }
            });
         }
         Err(err) =>
         {
            eprintln!("Ошибка соединения: {}", err);
         }
      }
   }

   Ok(())
}
