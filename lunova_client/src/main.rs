use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878").await?;
    let message = "Hello async server!\n";
    stream.write_all(message.as_bytes()).await?;
    Ok(())
}
