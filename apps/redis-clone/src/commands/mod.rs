use self::{echo::Echo, get::Get, ping::Ping, set::Set};

mod echo;
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
    Echo(Echo),
    CliEntry,
}

impl CommandTrait for Command {
    async fn execute(&self, db: &crate::db::Db, connection: &mut crate::connection::Connection) {
        match self {
            Command::Get(cmd) => cmd.execute(db, connection).await,
            Command::Ping(cmd) => cmd.execute(db, connection).await,
            Command::Set(cmd) => cmd.execute(db, connection).await,
            Command::Echo(cmd) => cmd.execute(db, connection).await,
            Command::CliEntry => {}
        }
    }

    fn from_resp(resp: crate::resp::RespValue) -> Result<Self, anyhow::Error> {
        if let crate::resp::RespValue::Array(arr) = &resp {
            let command = &arr[0];
            if let crate::resp::RespValue::BulkString(command) = command {
                let command = String::from_utf8_lossy(&command).to_lowercase();

                let command_handler = match command.as_str() {
                    "get" => Command::Get(Get::from_resp(resp)?),
                    "ping" => Command::Ping(Ping::from_resp(resp)?),
                    "echo" => Command::Echo(Echo::from_resp(resp)?),
                    "set" => Command::Set(Set::from_resp(resp)?),
                    "command" => Command::CliEntry,
                    _ => return Err(anyhow::anyhow!("Invalid command")),
                };

                return Ok(command_handler);
            }
        }
        Err(anyhow::anyhow!("Invalid command"))
    }
}
