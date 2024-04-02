use super::CommandTrait;

pub struct Ping {
    message: Option<String>,
}

impl Ping {
    pub fn new(message: Option<String>) -> Self {
        Self { message }
    }
}

impl CommandTrait for Ping {
    async fn execute(&self, _: &crate::db::Db, connection: &mut crate::connection::Connection) {
        let response = match &self.message {
            Some(message) => crate::resp::RespValue::BulkString(message.as_bytes().to_vec()),
            None => crate::resp::RespValue::SimpleString("PONG".to_string()),
        };
        connection.write(response).await;
    }

    fn from_resp(resp: crate::resp::RespValue) -> Result<Self, anyhow::Error> {
        if let crate::resp::RespValue::Array(arr) = resp {
            let message = if arr.len() > 1 {
                let message = arr[1].clone();
                if let crate::resp::RespValue::BulkString(message) = message {
                    Some(String::from_utf8_lossy(&message).to_string())
                } else {
                    return Err(anyhow::anyhow!("Invalid arguments"));
                }
            } else {
                None
            };
            return Ok(Self::new(message));
        }
        Err(anyhow::anyhow!("Invalid arguments"))
    }
}
