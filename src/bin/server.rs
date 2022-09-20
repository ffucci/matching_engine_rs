use tokio::{net::{TcpListener, TcpStream}, io::AsyncReadExt};
use tokio::io::{self, AsyncWriteExt};

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6001").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (mut socket, _) = listener.accept().await.unwrap();
        process(&mut socket).await;
    }
}

const MESSAGE_SIZE : usize = 12;

async fn process(socket: &mut TcpStream) {
    println!("socket {:?}", socket);
    let mut rx_bytes = Vec::new();
    socket.read_to_end(&mut rx_bytes).await;
    let received = std::str::from_utf8(&rx_bytes).expect("valid utf8");
    println!("Received: {0}", received);
}
