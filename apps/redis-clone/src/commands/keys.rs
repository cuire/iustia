use crate::db::Db;
use crate::next_arg;
use crate::resp::RespValue;

use super::CommandTrait;

pub struct Keys {
    pattern: String,
}

impl CommandTrait for Keys {
    async fn execute(&self, db: &Db) -> Option<RespValue> {
        if self.pattern == "*" {
            let keys = db.keys().await;
            let response = RespValue::Array(
                keys.iter()
                    .map(|key| RespValue::BulkString(key.as_bytes().to_vec()))
                    .collect(),
            );
            return Some(response);
        }

        Some(RespValue::SimpleError(
            ("Pattern not supported yet: ".to_string() + &self.pattern).into(),
        ))
    }
}

impl TryFrom<Vec<RespValue>> for Keys {
    type Error = anyhow::Error;

    fn try_from(args: Vec<RespValue>) -> Result<Self, Self::Error> {
        let mut args = args.into_iter();

        let _command = args.next();

        let pattern = next_arg!(args)?;

        return Ok(Self { pattern });
    }
}
