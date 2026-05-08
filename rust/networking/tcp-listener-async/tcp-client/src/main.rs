use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()>
{
   let mut stream = TcpStream::connect("127.0.0.1:8080")?;
   println!("Подключено к серверу!");

   // Отправляем сообщение
   stream.write_all(b"Hello from client!")?;
   // Читаем ответ
   let mut buffer = [0; 1024];
   stream.read(&mut buffer)?;
   println!("Ответ сервера: {}", String::from_utf8_lossy(&buffer[..]));
   Ok(())
}
