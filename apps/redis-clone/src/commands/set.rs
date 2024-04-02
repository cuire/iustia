use bytes::Bytes;
use tokio::time::Duration;

use crate::db::Db;
use crate::resp::RespValue;

use super::CommandTrait;

pub struct Set {
    key: String,
    value: Bytes,
    expire: Option<Duration>,
}

impl Set {
    pub fn new(key: String, value: Bytes, expire: Option<Duration>) -> Self {
        Self { key, value, expire }
    }
}

impl CommandTrait for Set {
    async fn execute(&self, db: &Db, connection: &mut crate::connection::Connection) {
        db.set(self.key.clone(), self.value.clone(), self.expire)
            .await;
        connection
            .write(RespValue::SimpleString("OK".to_string()))
            .await;
    }

    fn from_resp(resp: crate::resp::RespValue) -> Result<Self, anyhow::Error> {
        if let RespValue::Array(arr) = resp {
            let key_resp = arr[1].clone();

            let key_string = if let RespValue::BulkString(key) = key_resp {
                String::from_utf8_lossy(&key).to_string()
            } else {
                return Err(anyhow::anyhow!("Invalid arguments"));
            };

            if arr.len() < 2 {
                return Err(anyhow::anyhow!("Invalid arguments"));
            }

            let value = match &arr[2] {
                RespValue::BulkString(value) => Bytes::from(value.to_vec()),
                _ => {
                    return Err(anyhow::anyhow!("Invalid arguments"));
                }
            };

            let expire = if arr.len() > 4 {
                match &arr[3] {
                    RespValue::BulkString(s) => {
                        match String::from_utf8_lossy(&s).to_lowercase().as_str() {
                            "ex" => {
                                let expire_resp = &arr[4].as_integer()?;
                                Some(Duration::from_secs(*expire_resp as u64))
                            }

                            "px" => {
                                let expire_resp = &arr[4].as_integer()?;
                                Some(Duration::from_millis(*expire_resp as u64))
                            }
                            _ => {
                                return Err(anyhow::anyhow!(
                                    "Invalid arguments, expected 'ex' or 'px'"
                                ))
                            }
                        }
                    }
                    _ => None,
                }
            } else {
                None
            };

            return Ok(Self::new(key_string, value, expire));
        }
        Err(anyhow::anyhow!("Invalid arguments"))
    }
}
