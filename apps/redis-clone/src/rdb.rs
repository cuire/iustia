use std::{io::Cursor, time::Duration};

use tokio::io::AsyncReadExt;

use crate::db::Db;

/// https://rdb.fnordig.de/file_format.html
pub struct RDBParser {
    cursor: Cursor<Vec<u8>>,
}

#[derive(Debug, thiserror::Error)]
pub enum RDBParsingError {
    #[error("Invalid RDB header, expected REDIS")]
    InvalidHeader,
    #[error("Invalid opcode")]
    InvalidOpcode,
    #[error("Invalid RDB file: {0}")]
    InvalidRDBFile(String),
    #[error("Unimplemented")]
    Unimplemented,
}

impl RDBParser {
    pub(crate) fn new(cursor: Cursor<Vec<u8>>) -> Self {
        Self { cursor }
    }

    pub(crate) async fn load(&mut self, db: &Db) -> Result<(), RDBParsingError> {
        // check header
        let header = self.read_n(5).await?;

        if header != *b"REDIS" {
            return Err(RDBParsingError::InvalidHeader);
        }

        // TODO: check version
        let _version = self.read_n(4).await?;

        // TODO: check checksum

        // parse data
        while self.cursor.position() < self.cursor.get_ref().len() as u64 {
            let opcode = self.read_n(1).await?[0];

            match opcode {
                // # Auxiliary field, containing a db metadata, such as version, creation time, etc.
                0xFA => {
                    let _ = self.read_string().await;
                    let _ = self.read_string().await;
                }
                // # Database selector
                0xFE => {
                    let _ = self.read_n(4).await?;
                }
                // # Resize DB field
                0xFB => {
                    self.read_int().await?;
                    self.read_int().await?;
                }
                // # Expiry time field
                0xFC | 0xFD => {
                    let expiry = Duration::from_millis(self.read_expiry(opcode).await?);

                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::SystemTime::UNIX_EPOCH)
                        .unwrap();

                    let _value_type = self.read_n(1).await?[0];
                    let key = self.read_string().await?;

                    let value = self.read_string().await?.into();

                    if expiry < now {
                        continue;
                    }

                    let expiry = expiry;

                    db.set(key, value, Some(expiry)).await;
                }

                // # Key-Value pair
                0..=14 => {
                    let key = self.read_string().await?;
                    let value = self.read_string().await?.into();

                    db.set(key, value, None).await;
                }

                // # End of RDB file
                0xFF => {
                    // last 8 bytes are checksum that are checked before
                    let _ = self.read_n(8).await?;
                    return Ok(());
                }

                _ => {
                    Err(RDBParsingError::InvalidOpcode)?;
                }
            }
        }

        Err(RDBParsingError::InvalidRDBFile("unexpected EOF".into()))
    }

    async fn read_int(&mut self) -> Result<i64, RDBParsingError> {
        let len = self.read_length_encoding().await?;

        match len {
            LengthEncoding::Length(_) => Err(RDBParsingError::InvalidRDBFile("invalid int".into())),
            LengthEncoding::Format(f) => match f {
                0..=2 => Ok(self.read_int_format(f).await?),
                _ => Err(RDBParsingError::Unimplemented),
            },
        }
    }

    // 00000000  52 45 44 49 53 30 30 31  31 fa 09 72 65 64 69 73  |REDIS0011..redis|
    // 00000010  2d 76 65 72 05 37 2e 32  2e 35 fa 0a 72 65 64 69  |-ver.7.2.5..redi|
    // 00000020  73 2d 62 69 74 73 c0 40  fa 05 63 74 69 6d 65 c2  |s-bits.@..ctime.|
    // 00000030  6b 6b 58 66 fa 08 75 73  65 64 2d 6d 65 6d c2 90  |kkXf..used-mem..|
    // 00000040  f8 0d 00 fa 08 61 6f 66  2d 62 61 73 65 c0 00 fe  |.....aof-base...|
    // 00000050  00 fb 02 01 fc 65 8c 66  c9 8f 01 00 00 00 06 68  |.....e.f.......h|
    // 00000060  65 6c 6c 6f 32 05 77 6f  72 6c 64 00 05 68 65 6c  |ello2.world..hel|
    // 00000070  6c 6f 05 77 6f 72 6c 64  ff 1e bf a8 14 bc 85 51  |lo.world.......Q|
    // 00000080  96                                                |.|
    // 00000081

    async fn read_string(&mut self) -> Result<String, RDBParsingError> {
        let len = self.read_length_encoding().await?;

        match len {
            LengthEncoding::Length(n) => {
                let buf = self.read_n(n as usize).await?;
                Ok(std::str::from_utf8(&buf)
                    .map_err(|_| RDBParsingError::InvalidRDBFile("invalid string".into()))?
                    .into())
            }
            LengthEncoding::Format(format) => match format {
                0..=2 => Ok(self.read_int_format(format).await?.to_string()),
                _ => Err(RDBParsingError::InvalidOpcode),
            },
        }
    }

    async fn read_expiry(&mut self, opcode: u8) -> Result<u64, RDBParsingError> {
        match opcode {
            0xFC => {
                let buf = self.read_n(8).await?;
                Ok(u64::from_le_bytes(buf.try_into().unwrap()))
            }
            0xFD => {
                let buf = self.read_n(4).await?;
                Ok(u32::from_le_bytes(buf.try_into().unwrap()) as u64 * 1000)
            }
            _ => Err(RDBParsingError::InvalidOpcode),
        }
    }

    async fn read_int_format(&mut self, format: u8) -> Result<i64, RDBParsingError> {
        match format {
            0 => Ok(i8::from_le_bytes(self.read_n(1).await?.try_into().unwrap()) as i64),
            1 => Ok(i16::from_le_bytes(self.read_n(2).await?.try_into().unwrap()) as i64),
            2 => Ok(i32::from_le_bytes(self.read_n(4).await?.try_into().unwrap()) as i64),
            _ => Err(RDBParsingError::Unimplemented),
        }
    }

    async fn read_n(&mut self, n: usize) -> Result<Vec<u8>, RDBParsingError> {
        let mut buf = vec![0; n];
        self.cursor
            .read_exact(&mut buf)
            .await
            .map_err(|e| RDBParsingError::InvalidRDBFile(e.to_string()))?;

        Ok(buf)
    }

    async fn read_length_encoding(&mut self) -> Result<LengthEncoding, RDBParsingError> {
        let buf = self.read_n(1).await?[0];

        let top_bits = (buf & 0b11000000) >> 6;
        let rest_bits = buf & 0b00111111;

        match top_bits {
            0b00 => {
                let len = rest_bits as u32;
                Ok(LengthEncoding::Length(len))
            }
            0b01 => {
                let byte = self.read_n(1).await?[0];
                let len = ((rest_bits as u32) << 8) + byte as u32;
                Ok(LengthEncoding::Length(len))
            }
            0b10 => {
                let buf = self.read_n(4).await?;
                let len = u32::from_le_bytes(buf.try_into().unwrap());
                Ok(LengthEncoding::Length(len))
            }
            0b11 => Ok(LengthEncoding::Format(rest_bits)),
            _ => Err(RDBParsingError::InvalidOpcode),
        }
    }
}

#[derive(Debug)]
enum LengthEncoding {
    Length(u32),
    Format(u8),
}
