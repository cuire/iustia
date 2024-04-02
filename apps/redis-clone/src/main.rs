mod commands;
mod connection;
mod db;
mod resp;

use clap::Parser;
use tokio::net::TcpListener;

use crate::commands::CommandTrait;
use crate::connection::Connection;
use crate::resp::RespValue;

const DEFAULT_PORT: u16 = 6379;

#[derive(clap::Parser, Debug)]
#[clap()]
struct Cli {
    #[clap(long)]
    port: Option<u16>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let port = cli.port.unwrap_or(DEFAULT_PORT);
    let address = &format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(address).await.unwrap();

    let db_holder = db::DbHolder::new();

    println!("Server started at {}", address);

    loop {
        let db = db_holder.db();

        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            println!("Accepted new connection");

            let mut connection = Connection::new(stream);

            loop {
                let request = match connection.read().await {
                    Ok(request) => request,
                    Err(connection::ConnectionError::ResetByPeer) => {
                        println!("Connection reset by peer");
                        break;
                    }
                    Err(_) => {
                        println!("Failed to read from connection");
                        continue;
                    }
                };

                match commands::Command::from_resp(request) {
                    Ok(command) => {
                        command.execute(&db, &mut connection).await;
                    }
                    Err(e) => {
                        println!("ERR unknown command {:?}", e);
                        connection
                            .write(RespValue::SimpleError("ERR unknown command".to_string()))
                            .await
                    }
                };
            }
        });
    }
}
