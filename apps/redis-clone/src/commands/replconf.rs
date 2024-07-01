use crate::{next_arg, resp::RespValue};

use super::CommandTrait;

pub enum Replconf {
    ListeningPort(u16),
    Capa(String),
    Getack,
    Ack(u64),
}

impl Replconf {
    pub fn new_port(port: u16) -> Self {
        Self::ListeningPort(port)
    }

    pub fn new_capa(capa: String) -> Self {
        Self::Capa(capa)
    }
}

impl CommandTrait for Replconf {
    async fn execute(&self, _db: &crate::db::Db) -> Option<RespValue> {
        let response = crate::resp::RespValue::from("OK");
        Some(response)
    }
}

impl TryFrom<Vec<RespValue>> for Replconf {
    type Error = anyhow::Error;

    fn try_from(args: Vec<RespValue>) -> Result<Self, Self::Error> {
        let mut args = args.into_iter();

        let _command = args.next();

        let subcommand: String = next_arg!(args)?;

        match subcommand.to_lowercase().as_str() {
            "listening-port" => {
                let port: u64 = next_arg!(args)?;
                return Ok(Self::new_port(port as u16));
            }
            "capa" => {
                let capa = next_arg!(args)?;
                return Ok(Self::new_capa(capa));
            }
            "getack" => {
                return Ok(Self::Getack);
            }
            "ack" => {
                let offset = next_arg!(args)?;
                return Ok(Self::Ack(offset));
            }
            _ => return Err(anyhow::anyhow!("Invalid arguments")),
        }
    }
}
