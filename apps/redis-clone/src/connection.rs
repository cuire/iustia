use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

use crate::resp::{RespParseError, RespValue};

pub struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

#[derive(Debug, thiserror::Error)]
pub enum ConnectionError {
    #[error("Connection reset by peer")]
    ResetByPeer,
    #[error("Failed to read from connection")]
    ReadFailed,
}

impl Connection {
    pub(crate) fn new(socket: TcpStream) -> Self {
        Self {
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(1024 * 4),
        }
    }

    pub(crate) async fn read(&mut self) -> Result<RespValue, ConnectionError> {
        loop {
            let n = self.stream.read_buf(&mut self.buffer).await;
            match n {
                Ok(0) => {
                    return Err(ConnectionError::ResetByPeer);
                }
                Ok(_) => {}
                Err(_) => {
                    return Err(ConnectionError::ReadFailed);
                }
            }

            match RespValue::from_bytes(&self.buffer) {
                Ok(resp) => {
                    return Ok(resp);
                }
                // If we don't have a full response yet,
                // continue reading from the streamto get the rest of the response.
                // This is necessary because the response may not be fully read in a single read call.
                Err(RespParseError::MissingNewline) => continue,
                Err(_) => {
                    return Err(ConnectionError::ReadFailed);
                }
            }
        }
    }

    pub(crate) async fn write(&mut self, response: RespValue) {
        self.stream
            .write_all(response.to_buf().as_slice())
            .await
            .expect("Failed to write to connection");

        // Flush the stream to ensure the response is sent immediately. This is necessary because
        // the stream is buffered and the response may not be sent immediately.
        self.stream
            .flush()
            .await
            .expect("Failed to flush connection");
    }
}
