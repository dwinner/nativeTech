// 07_tcp_echo_server.rs
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

/// Handles a single client connection by reading from the stream and echoing back.
fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    println!("Accepted connection from: {}", stream.peer_addr()?);

    // A buffer to hold incoming data.
    // 1024 bytes (1KB) is a common size for simple examples. In a real
    // application, buffer size is a trade-off:
    // - Too small (e.g., 64 bytes) can lead to many system calls, which is inefficient.
    // - Too large (e.g., 1MB) wastes memory, especially if you have many
    // concurrent connections.
    // Common sizes for I/O buffers are often 4KB (4096) or 8KB (8192).
    let mut buffer = [0u8; 1024]; // A buffer to hold incoming data

    // Loop to read data and echo it back
    loop {
        // Read data from the client into the buffer
        let bytes_read = stream.read(&mut buffer)?;

        // If read() returns 0 bytes, the client has closed the connection
        if bytes_read == 0 {
            println!("Client disconnected.");
            return Ok(());
        }

        // Echo the received data back to the client
        stream.write_all(&buffer[..bytes_read])?;
        println!("Echoed {} bytes.", bytes_read);
    }
}

fn main() -> std::io::Result<()> {
    let listener_address = "127.0.0.1:8080";
    let listener = TcpListener::bind(listener_address)?;
    println!("Simple Echo Server listening on {}", listener_address);
    println!("Waiting for connections...");

    // listener.incoming() is an iterator that blocks until a new connection arrives.
    // This loop processes one client connection fully before accepting the next.
    for stream_result in listener.incoming() {
        match stream_result {
            Ok(stream) => {
                // A new client has connected successfully.
                if let Err(e) = handle_client(stream) {
                    eprintln!("Error handling client: {}", e);
                }
            }
            Err(e) => {
                // An error occurred while accepting a new connection.
                eprintln!("Failed to accept incoming connection: {}", e);
            }
        }
    }
    Ok(())
}