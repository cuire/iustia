use crate::resp::RespValue;

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
    async fn execute(&self, _: &crate::db::Db) -> Option<RespValue> {
        let response = match &self.message {
            Some(message) => RespValue::BulkString(message.as_bytes().to_vec()),
            None => RespValue::SimpleString("PONG".to_string()),
        };

        Some(response)
    }
}

impl TryFrom<Vec<RespValue>> for Ping {
    type Error = anyhow::Error;

    fn try_from(args: Vec<RespValue>) -> Result<Self, Self::Error> {
        let mut args = args.into_iter();

        let _command = args.next();

        let message = match args.next() {
            Some(RespValue::BulkString(message)) => Some(String::from_utf8(message)?),
            Some(_) => return Err(anyhow::anyhow!("Invalid argument")),
            None => None,
        };

        return Ok(Self::new(message));
    }
}
