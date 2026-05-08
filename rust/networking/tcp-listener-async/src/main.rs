use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_client(mut stream: TcpStream) -> std::io::Result<()>
{
   println!("Новое соединение: {}", stream.peer_addr()?);
   let mut buffer = [0; 1024];
   stream.read(&mut buffer).await?;
   println!("Получено: {}", String::from_utf8_lossy(&buffer[..]));
   stream.write_all(b"Hello from server!").await?;
   Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()>
{
   let listener = TcpListener::bind("127.0.0.1:8080").await?;
   println!("Сервер запущен на 127.0.0.1:8080");

   loop
   {
      let (stream, _) = listener.accept().await?;
      tokio::spawn(async move {
         if let Err(err) = handle_client(stream).await
         {
            eprintln!("Ошибка: {}", err);
         }
      });
   }
}
