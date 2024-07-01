mod commands;
mod conf;
mod connection;
mod db;
mod macros;
mod rdb;
mod resp;
mod utils;

use std::collections::HashMap;
use std::sync::Arc;

use clap::Parser;
use tokio::net::TcpListener;
use tokio::sync::broadcast::{self, Sender};
use tokio::sync::Mutex;

use crate::commands::CommandTrait;
use crate::conf::Config;
use crate::connection::Connection;
use crate::connection::ConnectionRead;
use crate::connection::ConnectionWrite;
use crate::db::DbBuilder;
use crate::resp::RespValue;

// const DEFAULT_ACK_EVERY: u64 = 1000;

#[derive(clap::Parser, Debug)]
struct Cli {
    #[clap(long, default_value = "6379")]
    port: u16,
    #[clap(long)]
    replicaof: Option<String>,
    #[clap(long)]
    dir: Option<std::path::PathBuf>,
    #[clap(long, default_value = "dump.rdb")]
    dbfilename: String,
}

async fn propaginate_slave(connection: &mut ConnectionWrite, sender: Arc<Sender<RespValue>>) {
    let mut receiver = sender.subscribe();

    while let Ok(f) = receiver.recv().await {
        println!("Sending {:?}", f);
        connection.write(&f).await;
    }
}

async fn read_slave(
    connection: &mut ConnectionRead,
    replica_offsets: Arc<Mutex<HashMap<String, (u64, u64)>>>,
) {
    loop {
        let request_result = connection.read().await;

        let (resp, _) = match &request_result {
            Ok(request) => request,
            Err(connection::ConnectionError::ResetByPeer) => {
                println!("Connection reset by peer");
                let mut replica_offsets = replica_offsets.lock().await;
                replica_offsets.remove(&connection.id());

                break;
            }
            Err(_) => {
                println!("Failed to read from connection");
                continue;
            }
        };

        match commands::Command::try_from(resp.clone()) {
            Ok(commands::Command::Replconf(command)) => match command {
                commands::Replconf::Ack(offset) => {
                    let mut replica_offsets = replica_offsets.lock().await;

                    let (_, start_offset) = replica_offsets.get(&connection.id()).unwrap().clone();
                    replica_offsets.remove(&connection.id());
                    replica_offsets.insert(connection.id(), (offset, start_offset));
                }
                _ => {}
            },
            _ => {}
        }
    }
}

async fn count_sync_replicas(
    replica_offsets: Arc<Mutex<HashMap<String, (u64, u64)>>>,
    master_offset: Arc<Mutex<u64>>,
) -> u32 {
    let replica_offsets = replica_offsets.lock().await;
    let master_offset = master_offset.lock().await;

    let mut count = 0;

    let master_offset = *master_offset;

    println!("master offset: {}", master_offset);
    println!("replica offsets: {:?}", replica_offsets);

    for (_, (offset, start_offset)) in replica_offsets.iter() {
        let offset = *offset;

        if offset + *start_offset >= master_offset {
            count += 1;
        }
    }

    count
}

async fn wait_for_more_replicas(
    connection: &mut Connection,
    num_of_replicas: u32,
    replica_offsets: Arc<Mutex<HashMap<String, (u64, u64)>>>,
    master_offset: Arc<Mutex<u64>>,
    sender: Arc<Sender<RespValue>>,
    timeout: u64,
) {
    const WAIT_ATTEMPTS: u64 = 75;

    let mut atempts = timeout / WAIT_ATTEMPTS;

    let mut connected = 0;

    while connected < num_of_replicas && atempts > 0 {
        let getack = RespValue::Array(vec![
            RespValue::BulkString("REPLCONF".as_bytes().to_vec()),
            RespValue::BulkString("GETACK".as_bytes().to_vec()),
            RespValue::BulkString("*".as_bytes().to_vec()),
        ]);

        sender.send(getack).unwrap();

        tokio::time::sleep(std::time::Duration::from_millis(WAIT_ATTEMPTS)).await;

        let replica_offsets = Arc::clone(&replica_offsets);
        let master_offset = Arc::clone(&master_offset);

        connected = count_sync_replicas(replica_offsets, master_offset).await;
        atempts -= 1;
    }

    connection
        .write(&RespValue::Integer(connected as i64))
        .await;
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let port = cli.port;

    let address = &format!("127.0.0.1:{}", port);

    let listener = TcpListener::bind(address).await.unwrap();

    let config = Config::from(cli);

    let db_builder = DbBuilder::new(config);

    let (sender, _rx) = broadcast::channel(16);
    let sender = Arc::new(sender);

    println!("Server started at {}", address);

    let db = db_builder.db();

    let config = db.config().await;

    let dbfile = config.persistence().dbfile();

    if dbfile.exists() {
        let rdb = tokio::fs::read(&dbfile).await;

        match rdb {
            Ok(rdb) => {
                let cursor = std::io::Cursor::new(rdb);
                println!("Loading RDB file");

                let mut parser = rdb::RDBParser::new(cursor);
                match parser.load(&db).await {
                    Ok(_) => {
                        println!("RDB loaded");
                    }
                    Err(e) => {
                        println!("Failed to load RDB {:?}", e);
                    }
                }
            }
            Err(_) => {
                println!("Failed to read RDB file");
            }
        }
    }

    match config.replication().role {
        conf::ReplicationRole::Slave {
            master_host,
            master_port,
        } => {
            tokio::spawn(async move {
                println!("Connecting to master");

                let stream =
                    tokio::net::TcpStream::connect(format!("{}:{}", master_host, master_port))
                        .await
                        .unwrap();

                let mut connection = Connection::new(stream);

                let command_resp = RespValue::from("PING").as_bulk().unwrap();

                let command = RespValue::Array(vec![command_resp]);

                connection.write(&command).await;

                let (response, _) = connection.read().await.unwrap();

                if response != RespValue::SimpleString("PONG".to_string()) {
                    println!("ERR master is not responding");
                    return;
                }

                let replconf = RespValue::Array(vec![
                    RespValue::BulkString("REPLCONF".as_bytes().to_vec()),
                    RespValue::BulkString("listening-port".as_bytes().to_vec()),
                    RespValue::BulkString(port.to_string().as_bytes().to_vec()),
                ]);

                connection.write(&replconf).await;

                let (response, _) = connection.read().await.unwrap();

                if response != RespValue::SimpleString("OK".to_string()) {
                    println!("ERR failed to set listening port on master");
                    return;
                }

                let replconf = RespValue::Array(vec![
                    RespValue::BulkString("REPLCONF".as_bytes().to_vec()),
                    RespValue::BulkString("capa".as_bytes().to_vec()),
                    RespValue::BulkString("psync2".as_bytes().to_vec()),
                ]);

                connection.write(&replconf).await;

                let (response, _) = connection.read().await.unwrap();

                if response != RespValue::SimpleString("OK".to_string()) {
                    println!("ERR failed to set capa on master");
                    return;
                }

                let psync = RespValue::Array(vec![
                    RespValue::BulkString("PSYNC".as_bytes().to_vec()),
                    RespValue::BulkString("?".as_bytes().to_vec()),
                    RespValue::BulkString("-1".as_bytes().to_vec()),
                ]);

                connection.write(&psync).await;

                let (response, _) = connection.read().await.unwrap();

                match response {
                    RespValue::SimpleString(response) => {
                        if response.starts_with("FULLRESYNC") {
                            println!("Full resync from master");
                        }
                        let rdb = connection.read().await;
                        match rdb {
                            Ok(rdb) => {
                                // TODO: apply RDB
                                println!("RDB received {:?}", rdb);
                            }
                            Err(_) => {
                                return;
                            }
                        }
                    }
                    _ => {
                        println!("ERR failed to start replication");
                        return;
                    }
                }

                println!(
                    "Connected to master at {}:{}. Starting replication",
                    master_host, master_port
                );

                let mut offset: i64 = 0;

                loop {
                    let request_result = connection.read().await;

                    let request = match &request_result {
                        Ok(request) => request,
                        Err(connection::ConnectionError::ResetByPeer) => {
                            println!("Connection reset by peer");
                            return;
                        }
                        Err(_) => {
                            println!("Failed to read from connection");
                            return;
                        }
                    };

                    let (resp, len) = request.clone();

                    match commands::Command::try_from(resp) {
                        Ok(commands::Command::Replconf(_)) => {
                            let resp = RespValue::Array(vec![
                                RespValue::BulkString("REPLCONF".as_bytes().to_vec()),
                                RespValue::BulkString("ACK".as_bytes().to_vec()),
                                RespValue::BulkString(offset.to_string().as_bytes().to_vec()),
                            ]);

                            offset += len as i64;
                            connection.write(&resp).await;
                        }
                        Ok(command) => {
                            let _resp = command.execute(&db).await.unwrap();
                            offset += len as i64;
                        }
                        Err(e) => {
                            println!("ERR unknown command {:?}", e);
                            connection
                                .write(&RespValue::SimpleError("ERR unknown command".to_string()))
                                .await;
                        }
                    };
                }
            });
        }
        _ => {}
    }

    let master_offset: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

    let replica_offsets: Arc<Mutex<HashMap<String, (u64, u64)>>> =
        Arc::new(Mutex::new(HashMap::new()));

    // if let conf::ReplicationRole::Master = config.replication().role {
    //     let sender: Arc<broadcast::Sender<RespValue>> = Arc::clone(&sender);

    //     let master_offset = Arc::clone(&master_offset);

    //     tokio::spawn(async move {
    //         loop {
    //             tokio::time::sleep(std::time::Duration::from_millis(DEFAULT_ACK_EVERY)).await;

    //             // TODO: send ack only if master has pending data\

    //             let getack = RespValue::Array(vec![
    //                 RespValue::BulkString("REPLCONF".as_bytes().to_vec()),
    //                 RespValue::BulkString("GETACK".as_bytes().to_vec()),
    //                 RespValue::BulkString("*".as_bytes().to_vec()),
    //             ]);

    //             let len = getack.size() as u64;

    //             sender.send(getack).unwrap();

    //             {
    //                 let mut master_offset = master_offset.lock().await;
    //                 *master_offset += len;
    //             }
    //         }
    //     });
    // }

    loop {
        let db = db_builder.db();

        let config = db.config().await;

        let (stream, _) = listener.accept().await.unwrap();

        let sender: Arc<broadcast::Sender<RespValue>> = Arc::clone(&sender);

        let master_offset = Arc::clone(&master_offset);

        let replica_offsets = Arc::clone(&replica_offsets);

        tokio::spawn(async move {
            println!("Accepted new connection");

            let mut connection = Connection::new(stream);

            loop {
                let request_result = connection.read().await;

                let request = match &request_result {
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

                let (resp_clone, _) = request.clone();

                match commands::Command::try_from(resp_clone) {
                    Ok(commands::Command::Psync(command)) => {
                        let resp = command.execute(&db).await.unwrap();

                        connection.write(&resp).await;

                        {
                            let hex_string = "524544495330303131fa0972656469732d76657205372e322e30fa0a72656469732d62697473c040fa056374696d65c26d08bc65fa08757365642d6d656dc2b0c41000fa08616f662d62617365c000fff06e3bfec0ff5aa2";
                            let empty_file_payload = (0..hex_string.len())
                                .step_by(2)
                                .map(|i| {
                                    u8::from_str_radix(&hex_string[i..i + 2], 16)
                                        .expect("hex_string is invalid")
                                })
                                .collect::<Vec<_>>();

                            connection
                                .write_bytes(
                                    format!("${}\r\n", empty_file_payload.len()).as_bytes(),
                                )
                                .await;

                            connection.write_bytes(empty_file_payload.as_slice()).await;
                            connection.flush().await;
                        }

                        let sender_clone = Arc::clone(&sender);

                        {
                            let mut replica_offsets = replica_offsets.lock().await;
                            let master_offset = master_offset.lock().await;
                            replica_offsets.insert(connection.id(), (0, *master_offset));
                        }

                        let (mut read_connection, mut write_connection) = connection.split();

                        tokio::spawn(async move {
                            propaginate_slave(&mut write_connection, sender_clone).await;
                        });

                        let replica_offsets = Arc::clone(&replica_offsets);

                        tokio::spawn(async move {
                            read_slave(&mut read_connection, replica_offsets).await;
                        });

                        break;
                    }
                    Ok(commands::Command::Wait(command)) => {
                        let replica_offsets = Arc::clone(&replica_offsets);
                        let master_offset = Arc::clone(&master_offset);

                        wait_for_more_replicas(
                            &mut connection,
                            command.num_of_replicas,
                            replica_offsets,
                            master_offset,
                            Arc::clone(&sender),
                            command.timeout,
                        )
                        .await;
                    }
                    Ok(command) => {
                        let resp = command.execute(&db).await.unwrap();

                        connection.write(&resp).await;

                        if command.is_propagated() {
                            if let conf::ReplicationRole::Master = config.replication().role {
                                let (resp, _) = request.clone();
                                {
                                    let mut master_offset = master_offset.lock().await;
                                    *master_offset += resp.size() as u64;
                                }
                                sender.send(resp).unwrap();
                            }
                        }
                    }
                    Err(e) => {
                        println!("ERR unknown command {:?}", e);
                        connection
                            .write(&RespValue::SimpleError("ERR unknown command".to_string()))
                            .await;
                    }
                };
            }
        });
    }
}
