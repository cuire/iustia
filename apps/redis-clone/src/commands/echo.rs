use crate::{next_arg, resp::RespValue};

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
    async fn execute(&self, _: &crate::db::Db) -> Option<RespValue> {
        Some(crate::resp::RespValue::BulkString(
            self.message.as_bytes().to_vec(),
        ))
    }

    // fn from_resp(resp: crate::resp::RespValue) -> Result<Self, anyhow::Error> {
}

impl TryFrom<Vec<crate::resp::RespValue>> for Echo {
    type Error = anyhow::Error;

    fn try_from(args: Vec<crate::resp::RespValue>) -> Result<Self, Self::Error> {
        let mut args = args.into_iter();

        let _command = args.next();

        let message = next_arg!(args)?;

        return Ok(Self::new(message));
    }
}
