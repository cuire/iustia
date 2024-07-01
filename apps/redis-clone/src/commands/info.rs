use crate::resp::RespValue;

use super::CommandTrait;

pub struct Info;

impl Info {
    pub fn new(_: Option<String>) -> Self {
        Self {}
    }
}

impl CommandTrait for Info {
    async fn execute(&self, db: &crate::db::Db) -> Option<RespValue> {
        let config = db.config().await;

        let replication = config.replication().to_string();

        let response = crate::resp::RespValue::from(replication.as_str());

        Some(response)
    }
}

impl TryFrom<Vec<crate::resp::RespValue>> for Info {
    type Error = anyhow::Error;

    fn try_from(_args: Vec<crate::resp::RespValue>) -> Result<Self, Self::Error> {
        return Ok(Self::new(None));
    }
}
