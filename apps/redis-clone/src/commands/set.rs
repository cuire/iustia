use bytes::Bytes;
use tokio::time::Duration;

use crate::db::Db;
use crate::next_arg;
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
    async fn execute(&self, db: &Db) -> Option<RespValue> {
        db.set(self.key.clone(), self.value.clone(), self.expire)
            .await;

        Some(RespValue::SimpleString("OK".to_string()))
    }
}

impl TryFrom<Vec<RespValue>> for Set {
    type Error = anyhow::Error;

    fn try_from(args: Vec<RespValue>) -> Result<Self, Self::Error> {
        let mut args = args.into_iter();

        let _command = args.next();

        let key = next_arg!(args)?;
        let value = next_arg!(args)?;

        let expire = next_arg!(args,
            optional,
            "ex" => {
                |v| Duration::from_secs(v)
            },
            "px" => {
                |v| Duration::from_millis(v)
            }
        );

        return Ok(Self::new(key, value, expire));
    }
}
