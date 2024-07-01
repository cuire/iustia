use crate::db::Db;
use crate::next_arg;
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
    async fn execute(&self, db: &Db) -> Option<RespValue> {
        let value = db.get(&self.key).await;
        let response = match value {
            Some(value) => RespValue::BulkString(value.to_vec()),
            None => RespValue::Null,
        };
        Some(response)
    }
}

impl TryFrom<Vec<RespValue>> for Get {
    type Error = anyhow::Error;

    fn try_from(args: Vec<RespValue>) -> Result<Self, Self::Error> {
        let mut args = args.into_iter();

        let _command = args.next();

        let key = next_arg!(args)?;

        return Ok(Self::new(key));
    }
}
