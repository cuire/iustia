use super::CommandTrait;

pub struct Echo {
    message: String,
}

impl Echo {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl CommandTrait for Echo {
    async fn execute(&self, _: &crate::db::Db, connection: &mut crate::connection::Connection) {
        let response = crate::resp::RespValue::BulkString(self.message.as_bytes().to_vec());
        connection.write(response).await;
    }

    fn from_resp(resp: crate::resp::RespValue) -> Result<Self, anyhow::Error> {
        if let crate::resp::RespValue::Array(arr) = resp {
            if arr.len() > 1 {
                let message = arr[1].clone();
                if let crate::resp::RespValue::BulkString(message) = message {
                    let message = String::from_utf8_lossy(&message).to_string();
                    return Ok(Self::new(message));
                }
            };
        }
        Err(anyhow::anyhow!("Invalid arguments"))
    }
}
