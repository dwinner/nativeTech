use tokio::net::TcpListener;

// TODO: write an echo server that accepts TCP connections on two listeners, concurrently.
//  Multiple connections (on the same listeners) should be processed concurrently.
//  The received data should be echoed back to the client.
pub async fn echoes(first: TcpListener, second: TcpListener) -> Result<(), anyhow::Error> {
    /* TODO */
}

/* TODO */
