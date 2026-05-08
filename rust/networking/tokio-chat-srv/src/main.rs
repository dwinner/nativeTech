use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> std::io::Result<()>
{
   let listener = TcpListener::bind("127.0.0.1:8080").await?;
   println!("Чат-сервер запущен на 127.0.0.1:8080");

   let (tx, _rx) = broadcast::channel(100); // Канал на 100 сообщений

   loop
   {
      let (mut stream, addr) = listener.accept().await?;
      let tx = tx.clone();
      let mut rx = tx.subscribe();

      tokio::spawn(async move {
         let mut buffer = [0; 1024];
         loop
         {
            tokio::select! {
                result = stream.read(&mut buffer) => {
                    match result {
                        Ok(n) if n > 0 => {
                            let msg = String::from_utf8_lossy(&buffer[..n]).to_string();
                            println!("{}: {}", addr, msg);
                            tx.send(msg).unwrap(); // Отправляем всем
                        }
                        _ => break, // Клиент отключился
                    }
                }
                result = rx.recv() => {
                    if let Ok(msg) = result {
                        stream.write_all(msg.as_bytes()).await.unwrap(); // Отправляем клиенту
                    }
                }
            }
         }
      });
   }
}
