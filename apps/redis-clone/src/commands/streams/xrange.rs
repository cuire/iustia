use crate::db::Db;
use crate::next_arg;
use crate::resp::RespValue;

use super::super::CommandTrait;

pub struct XRange {
    stream_key: String,
    start_id: String,
    end_id: String,
}

impl CommandTrait for XRange {
    async fn execute(&self, db: &Db) -> Option<RespValue> {
        let range = db
            .xrange(
                &self.stream_key.as_str(),
                &self.start_id.as_str(),
                &self.end_id.as_str(),
                None,
            )
            .await
            .unwrap_or(vec![]);

        Some(RespValue::Array(
            range
                .iter()
                .map(|(id, fields)| {
                    RespValue::Array(
                        vec![RespValue::BulkString(id.as_bytes().to_vec())]
                            .into_iter()
                            .chain(vec![RespValue::Array(vec![
                                RespValue::BulkString(fields.key().as_bytes().to_vec()),
                                RespValue::BulkString(fields.data().to_vec()),
                            ])])
                            .collect(),
                    )
                })
                .collect(),
        ))
    }
}

impl TryFrom<Vec<RespValue>> for XRange {
    type Error = anyhow::Error;

    fn try_from(args: Vec<RespValue>) -> Result<Self, Self::Error> {
        let mut args = args.into_iter();

        let _command = args.next();

        let stream_key = next_arg!(args)?;
        let start_id = next_arg!(args)?;
        let end_id = next_arg!(args)?;

        return Ok(Self {
            stream_key,
            start_id,
            end_id,
        });
    }
}
