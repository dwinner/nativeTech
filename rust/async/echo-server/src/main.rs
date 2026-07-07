use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>
{
   let listener = TcpListener::bind("127.0.0.1:8080").await?;
   println!("Сервер запущен на 127.0.0.1:8080");

   loop
   {
      let (mut socket, addr) = listener.accept().await?;
      println!("Новое подключение: {}", addr);

      tokio::spawn(async move {
         let mut buffer = [0; 1024];

         loop
         {
            match socket.read(&mut buffer).await
            {
               Ok(0) =>
               {
                  println!("Клиент {} отключился", addr);
                  return;
               }
               Ok(n) =>
               {
                  // Преобразуем полученные байты в строку, игнорируя не-UTF8
                  let request = String::from_utf8_lossy(&buffer[..n]).trim().to_lowercase();
                  let response = match request.as_str()
                  {
                     "hello" => "Привет, клиент!\n",
                     "time" =>
                     {
                        let time = chrono::Local::now().format("%H:%M:%S").to_string();
                        &format!("Текущее время: {}\n", time)
                     }
                     "exit" =>
                     {
                        let _ = socket.write_all(b"Bye!\n").await;
                        println!("Клиент {} запросил завершение", addr);
                        return;
                     }
                     _ => "Неизвестная команда. Попробуй: hello, time, exit\n",
                  };

                  if let Err(e) = socket.write_all(response.as_bytes()).await
                  {
                     eprintln!("Ошибка при записи клиенту {}: {}", addr, e);
                     return;
                  }
               }
               Err(e) =>
               {
                  eprintln!("Ошибка при чтении от клиента {}: {}", addr, e);
                  return;
               }
            }
         }
      });
   }
}
