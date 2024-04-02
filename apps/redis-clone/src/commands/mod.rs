use self::{get::Get, ping::Ping, set::Set};

mod get;
mod ping;
mod set;

pub trait CommandTrait {
    async fn execute(&self, db: &crate::db::Db, connection: &mut crate::connection::Connection);
    fn from_resp(resp: crate::resp::RespValue) -> Result<Self, anyhow::Error>
    where
        Self: Sized;
}

pub enum Command {
    Get(Get),
    Set(Set),
    Ping(Ping),
}

impl CommandTrait for Command {
    async fn execute(&self, db: &crate::db::Db, connection: &mut crate::connection::Connection) {
        match self {
            Command::Get(cmd) => cmd.execute(db, connection).await,
            Command::Ping(cmd) => cmd.execute(db, connection).await,
            Command::Set(cmd) => cmd.execute(db, connection).await,
        }
    }

    fn from_resp(resp: crate::resp::RespValue) -> Result<Self, anyhow::Error> {
        if let crate::resp::RespValue::Array(arr) = resp.clone() {
            let command = arr[0].clone();
            if let crate::resp::RespValue::BulkString(command) = command {
                let command = String::from_utf8_lossy(&command).to_lowercase();

                let command_handler = match command.as_str() {
                    "get" => Command::Get(Get::from_resp(resp)?),
                    "ping" => Command::Ping(Ping::from_resp(resp)?),
                    "set" => Command::Set(Set::from_resp(resp)?),
                    _ => return Err(anyhow::anyhow!("Invalid command")),
                };

                return Ok(command_handler);
            }
        }
        Err(anyhow::anyhow!("Invalid command"))
    }
}
