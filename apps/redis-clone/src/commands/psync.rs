use crate::{next_arg, resp::RespValue};

use super::CommandTrait;

pub struct Psync {
    id: String,
    offset: Option<u64>,
}

impl Psync {
    pub fn new(id: String, offset: Option<u64>) -> Self {
        Self { id, offset }
    }
}

impl CommandTrait for Psync {
    async fn execute(&self, db: &crate::db::Db) -> Option<RespValue> {
        match (self.id.as_str(), self.offset) {
            (id, None) if id == "?" => {
                let id = db.config().await.replication().master_replid.clone();

                let response = crate::resp::RespValue::SimpleString(
                    format!("FULLRESYNC {} 0", id).to_string(),
                );

                Some(response)
            }
            _ => unimplemented!(),
        }
    }
}

impl TryFrom<Vec<crate::resp::RespValue>> for Psync {
    type Error = anyhow::Error;

    fn try_from(args: Vec<crate::resp::RespValue>) -> Result<Self, Self::Error> {
        let mut args = args.into_iter();
        let _command = args.next();

        let id = next_arg!(args)?;

        let offset: i64 = next_arg!(args)?;

        if offset >= 0 {
            return Ok(Self::new(id, Some(offset as u64)));
        }

        return Ok(Self::new(id, None));
    }
}
