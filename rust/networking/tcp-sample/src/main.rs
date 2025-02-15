use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
   // We need to specify the port (80) explicitly,
   // TcpStream does not know that this will become an
   // HTTP request.
   let mut connection = TcpStream::connect("www.rustinaction.com:80")?;

   // GET is the HTTP method, / is the resource we're
   // attempting to access and HTTP/1.0 is the protocol
   // version we're requesting. Why 1.0? It does not
   // support "keep alive" requests, which will allow
   // our stream to close without difficulty.
   connection.write_all(b"GET / HTTP/1.0")?;

   // In many networking protocols, \r\n is how a new lines
   connection.write_all(b"\r\n")?;

   // The hostname provided on line 5 is actually
   // discarded once it is converted to an IP address.
   // The Host HTTP header allows the server to know
   // which host we're connecting to
   connection.write_all(b"Host: www.rustinaction.com")?;

   // Two blank lines signifies that we've finished the request.
   connection.write_all(b"\r\n\r\n")?;

   // std::io::copy() streams bytes from a Reader to a writer
   std::io::copy(&mut connection, &mut std::io::stdout())?;

   Ok(())
}
