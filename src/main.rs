use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;

#[tokio::main]
async fn main() {
    println!("hello world!;");

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    
    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {

            let (reader, mut writer) = socket.split();

            let mut bufreader = BufReader::new(reader);

            let mut line = String::new();


            loop {
                let bytes_reader = bufreader.read_line(&mut line).await.unwrap();
                if bytes_reader == 0 {
                    break;
                }

                writer.write_all(&line.as_bytes()).await.unwrap();

                line.clear();
            }
        });
    }
}
