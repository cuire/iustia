mod commands;
mod connection;
mod db;
mod resp;

use tokio::net::TcpListener;

use crate::commands::CommandTrait;
use crate::connection::Connection;
use crate::resp::RespValue;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let db_holder = db::DbHolder::new();

    println!("Server started at 127.0.0.1:6379");

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
                    Err(_) => {
                        connection
                            .write(RespValue::SimpleError("ERR unknown command".to_string()))
                            .await
                    }
                };
            }
        });
    }
}
