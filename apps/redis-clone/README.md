# 🏮 Redis Clone

This is a simple Redis clone written in Rust. Mainly for learning purposes. 

## Running

```bash
cargo run

# then use the redis-cli to connect to the server and run commands
redis-cli ping
```

## Features

- Async Tokio based server implementation
- Simple Redis protocol implementation, command parsing using rust macros (see [src/macros/](./src/macros/))
- Replication. To start slave run `cargo run -- --replicaof "127.0.0.1:6379"`
- Persistence (dump and load RDB files), to use call with `cargo run -- --dir "./data"`
- WIP Data streams using Radix trees
