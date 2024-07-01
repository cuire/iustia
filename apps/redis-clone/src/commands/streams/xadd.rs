use bytes::Bytes;

use crate::db::Db;
use crate::next_arg;
use crate::resp::RespValue;

use super::super::CommandTrait;

pub struct XAdd {
    stream_key: String,
    id: String,
    key: String,
    value: Bytes,
}

impl CommandTrait for XAdd {
    async fn execute(&self, db: &Db) -> Option<RespValue> {
        if self.id == "0-0" {
            return Some(RespValue::SimpleError(
                "ERR The ID specified in XADD must be greater than 0-0\r\n".into(),
            ));
        }

        let id = db
            .xadd(
                &self.stream_key.as_str(),
                Some(self.id.clone()),
                self.key.clone(),
                self.value.clone(),
            )
            .await;

        match id {
            Ok(id) => Some(RespValue::SimpleString(id)),
            Err(_) => Some(RespValue::SimpleError(
                "ERR The ID specified in XADD is equal or smaller than the target stream top item"
                    .into(),
            )),
        }
    }
}

impl TryFrom<Vec<RespValue>> for XAdd {
    type Error = anyhow::Error;

    fn try_from(args: Vec<RespValue>) -> Result<Self, Self::Error> {
        let mut args = args.into_iter();

        let _command = args.next();

        let stream_key = next_arg!(args)?;
        let id = next_arg!(args)?;
        let key = next_arg!(args)?;
        let value = next_arg!(args)?;

        return Ok(Self {
            stream_key,
            id,
            key,
            value,
        });
    }
}
