use crate::db::Db;
use crate::next_arg;
use crate::resp::RespValue;

use super::CommandTrait;

pub struct Type {
    key: String,
}

impl CommandTrait for Type {
    async fn execute(&self, db: &Db) -> Option<RespValue> {
        Some(RespValue::SimpleString(db.value_type(&self.key).await))
    }
}

impl TryFrom<Vec<RespValue>> for Type {
    type Error = anyhow::Error;

    fn try_from(args: Vec<RespValue>) -> Result<Self, Self::Error> {
        let mut args = args.into_iter();

        let _command = args.next();

        let key = next_arg!(args)?;

        return Ok(Self { key });
    }
}
