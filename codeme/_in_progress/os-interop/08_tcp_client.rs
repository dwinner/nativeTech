// 08_tcp_client.rs
use std::net::TcpStream;
use std::io::{self, Write, BufRead, BufReader};

fn main() -> io::Result<()> {
    let server_address = "127.0.0.1:8080";
    println!("Connecting to echo server at {}...", server_address);

    // 1. Connect to the server. The '?' operator handles connection errors concisely.
    let mut stream = TcpStream::connect(server_address)?;
    println!("Connected! Type a message and press Enter. Type 'quit' to exit.");

    // 2. Prepare for reading and writing.
    // We can clone the stream to have separate handles for reading and writing.
    // This is a common pattern for more complex I/O.
    let mut reader = BufReader::new(stream.try_clone()?);

    loop {
        // Read a line of input from the user's keyboard.
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line)?;

        let message_to_send = input_line.trim(); // Trim whitespace and newline
        if message_to_send == "quit" || message_to_send.is_empty() {
            break; // Exit loop if user types 'quit' or just presses Enter
        }

        // 3. Send the message to the server.
        // We add a newline so the server's logic (and our read_line below) works expectedly.
        writeln!(stream, "{}", message_to_send)?;
        stream.flush()?; // Ensure the buffered data is sent immediately.

        // 4. Read the echo back from the server.
        let mut echoed_response = String::new();
        reader.read_line(&mut echoed_response)?;
        
        print!("Server echoed: {}", echoed_response); // `read_line` includes the newline
    }

    println!("Disconnecting from server.");
    Ok(())
}