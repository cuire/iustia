use crate::db::{Db, StreamEntry};
use crate::next_arg;
use crate::resp::RespValue;

use super::super::CommandTrait;

pub struct XRead {
    entries: Vec<(String, String)>,
}

impl CommandTrait for XRead {
    async fn execute(&self, db: &Db) -> Option<RespValue> {
        let mut entries: Vec<(String, StreamEntry)> = vec![];

        for (stream_key, id) in self.entries.iter() {
            let entry = db.xread(stream_key.clone(), id.clone()).await;

            if let Some(entry) = entry {
                entries.push((stream_key.clone(), entry));
            }
        }

        Some(RespValue::Array(
            entries
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

impl TryFrom<Vec<RespValue>> for XRead {
    type Error = anyhow::Error;

    fn try_from(args: Vec<RespValue>) -> Result<Self, Self::Error> {
        let mut args: std::vec::IntoIter<RespValue> = args.into_iter();

        let _command = args.next();

        let mut next_arg: String = next_arg!(args)?;
        while String::from("streams") != next_arg.to_ascii_lowercase() {
            // TODO: handle XRead Options
            next_arg = next_arg!(args)?;
        }

        let mut entries: Vec<String> = vec![];

        while let Ok(v) = next_arg!(args) {
            entries.push(v);
        }

        let n = entries.len();
        // convert entries to a tuple of (stream_key, id) pairs
        // e.g. ["stream1", "stream2", "id1", "id2"] -> [("stream1", "id1"), ("stream2", "id2")]
        let entries: Vec<(String, String)> = entries.iter().take(n / 2).enumerate().fold(
            vec![],
            |mut acc: Vec<(String, String)>, (i, v)| {
                acc.push((v.to_string(), entries[n / 2 + i].to_string()));
                acc
            },
        );

        Ok(Self { entries })
    }
}
