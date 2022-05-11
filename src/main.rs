use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    println!("hello world!;");

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let (mut socket, _addr) = listener.accept().await.unwrap();

    let mut buffer = [0u8; 1024];

    let bytes_read = socket.read(&mut buffer).await.unwrap();

    socket.write_all(&buffer[..bytes_read]).await.unwrap();
}
