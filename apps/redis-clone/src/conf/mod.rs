use std::{
    fmt::{Display, Formatter},
    net::Ipv4Addr,
    str::FromStr,
};

use crate::Cli;

#[derive(Debug, Clone)]
pub struct Config {
    persistence: PersistenceConfig,
    replication: ReplicationConfig,
}

impl Config {
    pub(crate) fn replication(&self) -> ReplicationConfig {
        self.replication.clone()
    }

    pub(crate) fn persistence(&self) -> PersistenceConfig {
        self.persistence.clone()
    }
}

impl From<Cli> for Config {
    fn from(cli: Cli) -> Self {
        let role = match cli.replicaof {
            Some(replicaof) => {
                let replicaof: Vec<String> = replicaof.split(' ').map(|s| s.to_string()).collect();
                ReplicationRole::from(replicaof)
            }
            None => ReplicationRole::Master,
        };

        let dir = cli.dir.unwrap_or(std::env::current_dir().unwrap());
        let dbfilename = cli.dbfilename;

        Self {
            persistence: PersistenceConfig { dir, dbfilename },
            replication: ReplicationConfig {
                role,
                master_replid: crate::utils::random_string(40),
                master_repl_offset: 0,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum ReplicationRole {
    Master,
    Slave {
        master_host: std::net::IpAddr,
        master_port: u16,
    },
}

impl Display for ReplicationRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ReplicationRole::Master => write!(f, "master"),
            ReplicationRole::Slave { .. } => write!(f, "slave"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReplicationConfig {
    pub(crate) role: ReplicationRole,
    pub(crate) master_replid: String,
    master_repl_offset: u64,
}

impl Display for ReplicationConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "#Replication\r\nrole:{}\r\nmaster_replid:{}\r\nmaster_repl_offset:{}\r\n",
            self.role, self.master_replid, self.master_repl_offset
        )
    }
}

impl From<Vec<String>> for ReplicationRole {
    fn from(v: Vec<String>) -> Self {
        let master_host = match v[0].as_str() {
            "localhost" => Ok(std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
            _ => std::net::IpAddr::from_str(&v[0]),
        };

        let master_port = v[1].parse::<u16>();

        match (master_host, master_port) {
            (Ok(master_host), Ok(master_port)) => ReplicationRole::Slave {
                master_host,
                master_port,
            },
            _ => panic!("Invalid master host or port"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PersistenceConfig {
    // internal section
    dir: std::path::PathBuf,
    dbfilename: String,
}

impl PersistenceConfig {
    pub(crate) fn dir(&self) -> &std::path::PathBuf {
        &self.dir
    }

    pub(crate) fn dbfilename(&self) -> &str {
        &self.dbfilename
    }

    pub(crate) fn dbfile(&self) -> std::path::PathBuf {
        self.dir.join(&self.dbfilename)
    }
}
