use std::net::UdpSocket;

fn main() -> std::io::Result<()>
{
   // Создаём UDP-сокет и привязываем к адресу
   let socket = UdpSocket::bind("127.0.0.1:8080")?;
   println!("UDP-сервер запущен на 127.0.0.1:8080");

   let mut buffer = [0; 1024];
   let (bytes, sender) = socket.recv_from(&mut buffer)?;
   println!(
      "Получено {} байт от {}: {}",
      bytes,
      sender,
      String::from_utf8_lossy(&buffer[..bytes])
   );

   // Отправляем ответ
   socket.send_to(b"Hello from UDP server!", sender)?;
   Ok(())
}
