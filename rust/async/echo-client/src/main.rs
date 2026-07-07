use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time::{timeout, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>
{
   let mut stream = timeout(Duration::from_secs(5), TcpStream::connect("127.0.0.1:8080")).await??;
   println!("Подключение к серверу установлено!");

   let (reader, mut writer) = stream.split();
   let mut reader = BufReader::new(reader);

   // Канал для передачи сообщений от stdin к writer
   let (tx, mut rx) = mpsc::channel::<String>(100);

   // Задача для чтения ответов
   let read_task = tokio::spawn(async move {
      loop
      {
         let mut response = String::new();
         match timeout(Duration::from_secs(2), reader.read_line(&mut response)).await
         {
            Ok(Ok(_)) => println!("Получено: {}", response.trim()),
            Ok(Err(e)) => eprintln!("Ошибка чтения: {}", e),
            Err(_) => eprintln!("Таймаут при чтении"),
         }
      }
   });

   // Задача для отправки сообщений
   let write_task = tokio::spawn(async move {
      while let Some(message) = rx.recv().await
      {
         if let Err(e) = writer.write_all(message.as_bytes()).await
         {
            eprintln!("Ошибка записи: {}", e);
            break;
         }
         if let Err(e) = writer.flush().await
         {
            eprintln!("Ошибка flush: {}", e);
            break;
         }
         println!("Отправлено: {}", message.trim());
      }
   });

   // Чтение ввода пользователя
   let mut stdin = BufReader::new(tokio::io::stdin());
   loop
   {
      let mut input = String::new();
      println!("Введите сообщение (или 'quit' для выхода):");
      stdin.read_line(&mut input).await?;
      let input = input.trim().to_string();
      if input == "quit"
      {
         break;
      }

      tx.send(input + "\n").await?;
   }

   // Завершение задач
   drop(tx); // Закрываем канал
   read_task.await?;
   write_task.await?;

   Ok(())
}
