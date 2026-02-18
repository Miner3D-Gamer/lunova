//! What the admin/server gets to deal with

// use tokio::io::AsyncReadExt;
// use lunova_lib;

// use mirl::prelude::LogToConsole;

use lunova_lib::{
    fs::{server_from_bin, server_to_bin},
    server::ServerState,
};

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
    // let command_arguments: Vec<String> = std::env::args().collect();

    let matches = clap::Command::new("server")
        .arg(
            clap::Arg::new("storage")
                .short('s')
                .long("storage")
                .alias("storage_folder")
                .help("Where data should be read from/written to")
                .required(true),
        )
        .get_matches();

    // matches.println_self();
    let Some(storage_folder): Option<&String> = matches.get_one("storage")
    else {
        println!("No storage folder has been defined");
        std::process::exit(1)
    };
    if std::fs::metadata(storage_folder).is_err() {
        std::fs::create_dir(storage_folder).unwrap();
    }
    let bin = storage_folder.to_owned() + "/lunova.bin";

    let mut server = if std::fs::metadata(&bin).is_err() {
        println!("Getting default server");
        ServerState::default()
    } else {
        println!("Loading server from storage");
        server_from_bin(&std::fs::read(&bin).unwrap()).unwrap().unwrap()
    };

    let account = lunova_lib::users::user::Accounts::new(
        lunova_lib::users::user_id::UserID::new(
            "test_user".to_string(),
            &server.configs.user_id_contraints,
        )
        .unwrap(),
        "User".to_string(),
    );
    if server.users.contains_account(&account) {
        server.users.users = Vec::new();
        println!("Reset user");
    } else {
        server.users.users.push(account);
        println!("Added user");
    }

    // println!("{:#?}", server.users);

    std::fs::write(
        bin,
        server_to_bin(server, lunova_lib::fs::encodings::Encodings::PostCard)
            .unwrap(),
    )
    .unwrap();

    Ok(())
}
