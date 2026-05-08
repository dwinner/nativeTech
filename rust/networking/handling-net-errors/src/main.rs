use std::io::{Read, Write};
use std::net::TcpStream;

fn connect_to_server() -> Result<(), Box<dyn std::error::Error>>
{
   let mut stream = TcpStream::connect("127.0.0.1:8080").map_err(|e| format!("Не удалось подключиться: {}", e))?;
   stream.write_all(b"Test")?;
   let mut buffer = [0; 1024];
   stream.read(&mut buffer)?;
   println!("Получено: {}", String::from_utf8_lossy(&buffer));
   Ok(())
}

fn main()
{
   match connect_to_server()
   {
      Ok(()) => println!("Успешно завершено"),
      Err(e) => eprintln!("Ошибка: {}", e),
   }
}
