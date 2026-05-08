use std::io::{Read, Write};
use std::net::TcpListener;

fn main() -> std::io::Result<()>
{
   // Привязываем сервер к адресу 127.0.0.1:8080
   let listener = TcpListener::bind("127.0.0.1:8080")?;
   println!("Сервер запущен на 127.0.0.1:8080");

   // Бесконечный цикл для обработки входящих соединений
   for stream in listener.incoming()
   {
      match stream
      {
         Ok(mut stream) =>
         {
            println!("Новое соединение: {}", stream.peer_addr()?);
            
            // Читаем данные от клиента
            let mut buffer = [0; 1024];
            stream.read(&mut buffer)?;
            println!("Получено: {}", String::from_utf8_lossy(&buffer[..]));
            
            // Отправляем ответ
            stream.write_all(b"Hello from server!")?;
         }
         Err(e) =>
         {
            eprintln!("Ошибка соединения: {}", e);
         }
      }
   }

   Ok(())
}
