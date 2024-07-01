use std::io::Cursor;

use bytes::{Buf, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;

use crate::resp::{RespParseError, RespValue};

#[derive(Debug)]
pub struct Connection {
    read_connection: ConnectionRead,
    write_connection: ConnectionWrite,
}

#[derive(Debug)]
pub struct ConnectionRead {
    stream: OwnedReadHalf,
    buffer: BytesMut,

    id: String,
}

#[derive(Debug)]
pub struct ConnectionWrite {
    stream: BufWriter<OwnedWriteHalf>,

    #[allow(dead_code)]
    id: String,
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
        use nanoid::nanoid;

        let (read_connection, write_connection) = socket.into_split();

        let id = nanoid!(10);

        Self {
            read_connection: ConnectionRead {
                stream: read_connection,
                buffer: BytesMut::with_capacity(4096),
                id: id.clone(),
            },
            write_connection: ConnectionWrite {
                stream: BufWriter::new(write_connection),
                id: id.clone(),
            },
        }
    }

    pub(crate) async fn read(&mut self) -> Result<(RespValue, usize), ConnectionError> {
        self.read_connection.read().await
    }

    pub(crate) async fn write(&mut self, response: &RespValue) -> usize {
        self.write_connection.write(response).await
    }

    pub(crate) async fn write_bytes(&mut self, response: &[u8]) {
        self.write_connection.write_bytes(response).await
    }

    pub(crate) async fn flush(&mut self) {
        self.write_connection.flush().await
    }

    pub(crate) fn split(self) -> (ConnectionRead, ConnectionWrite) {
        (self.read_connection, self.write_connection)
    }

    #[allow(dead_code)]
    pub(crate) fn id(&self) -> String {
        self.read_connection.id()
    }
}

impl ConnectionRead {
    pub(crate) async fn read(&mut self) -> Result<(RespValue, usize), ConnectionError> {
        loop {
            if let Some((resp, len)) = self.parse_resp()? {
                return Ok((resp, len));
            }

            match self.stream.read_buf(&mut self.buffer).await {
                Ok(0) => {
                    return Err(ConnectionError::ResetByPeer);
                }
                Ok(_) => {}
                Err(_) => {
                    return Err(ConnectionError::ReadFailed);
                }
            }
        }
    }

    fn parse_resp(&mut self) -> Result<Option<(RespValue, usize)>, ConnectionError> {
        let mut buf: Cursor<&[u8]> = Cursor::new(&self.buffer[..]);

        match RespValue::from_bytes(&mut buf) {
            Ok(resp) => {
                let len = buf.position() as usize;
                self.buffer.advance(len);

                Ok(Some((resp, len)))
            }
            Err(RespParseError::Incomplete) => Ok(None),
            Err(RespParseError::MissingNewline) => Ok(None),
            Err(_) => Err(ConnectionError::ReadFailed),
        }
    }

    pub(crate) fn id(&self) -> String {
        self.id.clone()
    }
}

impl ConnectionWrite {
    pub(crate) async fn write(&mut self, response: &RespValue) -> usize {
        let response = response.to_buf();

        let len = response.len();

        self.stream
            .write_all(response.as_slice())
            .await
            .expect("Failed to write to connection");

        // Flush the stream to ensure the response is sent immediately. This is necessary because
        // the stream is buffered and the response may not be sent immediately.
        self.stream
            .flush()
            .await
            .expect("Failed to flush connection");

        len
    }

    pub(crate) async fn write_bytes(&mut self, response: &[u8]) {
        self.stream
            .write_all(response)
            .await
            .expect("Failed to write to connection");
    }

    pub(crate) async fn flush(&mut self) {
        self.stream
            .flush()
            .await
            .expect("Failed to flush connection");
    }

    #[allow(dead_code)]
    pub(crate) fn id(&self) -> String {
        self.id.clone()
    }
}
