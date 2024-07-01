use crate::{next_arg, resp::RespValue};

use super::CommandTrait;

pub enum Config {
    Get(String),
}

impl CommandTrait for Config {
    async fn execute(&self, db: &crate::db::Db) -> Option<RespValue> {
        let config = db.config().await;

        let persistence = config.persistence();

        let value: Option<RespValue> = match self {
            Config::Get(param) => match param.as_str() {
                "dir" => Some(persistence.dir().to_str().unwrap().into()),
                "dbfilename" => Some(persistence.dbfilename().into()),
                _ => None,
            },
        };

        if value.is_none() {
            return Some(RespValue::SimpleError("ERR parameter not supported".into()));
        }

        Some(RespValue::Array(vec![
            RespValue::BulkString("dir".into()),
            value.unwrap().as_bulk().unwrap(),
        ]))
    }
}

impl TryFrom<Vec<RespValue>> for Config {
    type Error = anyhow::Error;

    fn try_from(args: Vec<RespValue>) -> Result<Self, Self::Error> {
        let mut args = args.into_iter();

        let _command = args.next();

        let subcommand: String = next_arg!(args)?;

        match subcommand.to_lowercase().as_str() {
            "get" => {
                let param = next_arg!(args)?;
                Ok(Self::Get(param))
            }
            _ => Err(anyhow::anyhow!("Not Implemented")),
        }
    }
}
