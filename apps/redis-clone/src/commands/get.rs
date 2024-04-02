use crate::db::Db;
use crate::resp::RespValue;

use super::CommandTrait;

pub struct Get {
    key: String,
}

impl Get {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl CommandTrait for Get {
    async fn execute(&self, db: &Db, connection: &mut crate::connection::Connection) {
        let value = db.get(&self.key).await;
        let response = match value {
            Some(value) => RespValue::BulkString(value.to_vec()),
            None => RespValue::Null,
        };
        connection.write(response).await;
    }

    fn from_resp(resp: crate::resp::RespValue) -> Result<Self, anyhow::Error> {
        if let RespValue::Array(arr) = resp {
            let key = arr[1].clone();
            if let RespValue::BulkString(key) = key {
                let key = String::from_utf8_lossy(&key).to_string();
                return Ok(Self::new(key));
            }
        }
        Err(anyhow::anyhow!("Invalid arguments"))
    }
}
