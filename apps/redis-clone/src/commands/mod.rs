use crate::resp::RespValue;

use self::{
    config::Config, echo::Echo, get::Get, info::Info, keys::Keys, ping::Ping, r#type::Type,
    set::Set, wait::Wait,
};

mod config;
mod echo;
mod get;
mod info;
mod keys;
mod ping;
mod psync;
mod replconf;
mod set;
mod streams;
mod r#type;
mod wait;

use streams::{xadd::XAdd, xrange::XRange, xread::XRead};

pub use psync::Psync;
pub use replconf::Replconf;

pub trait CommandTrait {
    async fn execute(&self, db: &crate::db::Db) -> Option<RespValue>;
}

pub enum Command {
    Config(Config),
    Get(Get),
    Set(Set),
    Ping(Ping),
    Echo(Echo),
    Keys(Keys),
    Info(Info),
    Type(Type),
    Replconf(Replconf),
    Psync(Psync),
    Wait(Wait),

    XAdd(XAdd),
    XRange(XRange),
    XRead(XRead),

    CliEntry,
}

impl Command {
    pub(crate) fn is_propagated(&self) -> bool {
        match self {
            Command::Set(_) => true,
            _ => false,
        }
    }
}

impl TryFrom<RespValue> for Command {
    type Error = anyhow::Error;

    fn try_from(resp: crate::resp::RespValue) -> Result<Self, anyhow::Error> {
        if let crate::resp::RespValue::Array(args) = resp {
            let command = args
                .iter()
                .next()
                .ok_or(anyhow::anyhow!("Invalid command"))?;

            if let crate::resp::RespValue::BulkString(command) = command {
                let command = String::from_utf8_lossy(&command).to_lowercase();

                let command_handler = match command.as_str() {
                    "get" => Command::Get(Get::try_from(args)?),
                    "ping" => Command::Ping(Ping::try_from(args)?),
                    "echo" => Command::Echo(Echo::try_from(args)?),
                    "set" => Command::Set(Set::try_from(args)?),
                    "info" => Command::Info(Info::try_from(args)?),
                    "replconf" => Command::Replconf(Replconf::try_from(args)?),
                    "psync" => Command::Psync(Psync::try_from(args)?),
                    "wait" => Command::Wait(Wait::try_from(args)?),
                    "config" => Command::Config(Config::try_from(args)?),
                    "type" => Command::Type(Type::try_from(args)?),
                    "keys" => Command::Keys(Keys::try_from(args)?),

                    "xadd" => Command::XAdd(XAdd::try_from(args)?),
                    "xrange" => Command::XRange(XRange::try_from(args)?),
                    "xread" => Command::XRead(XRead::try_from(args)?),

                    "command" => Command::CliEntry,
                    _ => return Err(anyhow::anyhow!("Invalid command")),
                };

                return Ok(command_handler);
            }
        }
        Err(anyhow::anyhow!("Invalid command format"))
    }
}

impl CommandTrait for Command {
    async fn execute(&self, db: &crate::db::Db) -> Option<RespValue> {
        let resp = match self {
            Command::Get(cmd) => cmd.execute(db).await,
            Command::Ping(cmd) => cmd.execute(db).await,
            Command::Set(cmd) => cmd.execute(db).await,
            Command::Echo(cmd) => cmd.execute(db).await,
            Command::Info(cmd) => cmd.execute(db).await,
            Command::Replconf(cmd) => cmd.execute(db).await,
            Command::Psync(cmd) => cmd.execute(db).await,
            Command::Config(cmd) => cmd.execute(db).await,
            Command::Type(cmd) => cmd.execute(db).await,
            Command::Keys(cmd) => cmd.execute(db).await,
            Command::Wait(_) => None,

            Command::XAdd(cmd) => cmd.execute(db).await,
            Command::XRange(cmd) => cmd.execute(db).await,
            Command::XRead(cmd) => cmd.execute(db).await,

            Command::CliEntry => None,
        };

        resp
    }
}
