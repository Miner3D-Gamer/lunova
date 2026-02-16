//! What the admin/server gets to deal with

// use tokio::io::AsyncReadExt;
// use lunova_lib;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // let listener = tokio::net::TcpListener::bind("127.0.0.1:7878").await?;
    // println!("Server listening on 127.0.0.1:7878");

    // loop {
    //     let (mut socket, addr) = listener.accept().await?;
    //     println!("New client: {addr}");

    //     tokio::spawn(async move {
    //         let mut buffer = [0; 1024];
    //         loop {
    //             let n = match socket.read(&mut buffer).await {
    //                 Ok(0) => break, // connection closed
    //                 Ok(n) => n,
    //                 Err(e) => {
    //                     eprintln!("Failed to read from socket; err = {e:?}");
    //                     break;
    //                 }
    //             };
    //             println!(
    //                 "Received from {}: {}",
    //                 addr,
    //                 String::from_utf8_lossy(&buffer[..n])
    //             );
    //         }
    //     });
    // }

    // let server_state = lunova_lib::server::ServerState::default();

    Ok(())
}
