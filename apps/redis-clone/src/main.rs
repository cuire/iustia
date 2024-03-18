use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Server started at 127.0.0.1:6379");

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            println!("Accepted new connection");

            let mut buffer = [0; 1024];

            loop {
                match stream.read(&mut buffer).await {
                    Ok(0) => {
                        // The client has closed the connection
                        println!("Connection closed");
                        break;
                    }
                    Ok(_n) => {
                        // let received = String::from_utf8_lossy(&buffer[..n]);
                        // println!("Received: {}", received);

                        let response = "+PONG\r\n";
                        if let Err(e) = stream.write_all(response.as_bytes()).await {
                            println!("Failed to write to connection: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Failed to read from connection: {}", e);
                        break;
                    }
                }
            }
        });
    }
}
