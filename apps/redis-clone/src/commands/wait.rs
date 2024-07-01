use crate::{next_arg, resp::RespValue};

pub struct Wait {
    pub(crate) num_of_replicas: u32,
    pub(crate) timeout: u64,
}

impl TryFrom<Vec<RespValue>> for Wait {
    type Error = anyhow::Error;

    fn try_from(args: Vec<RespValue>) -> Result<Self, Self::Error> {
        let mut args = args.into_iter();

        let _command = args.next();

        let num_of_replicas: u64 = next_arg!(args)?;

        let timeout = next_arg!(args)?;

        Ok(Self {
            num_of_replicas: num_of_replicas as u32,
            timeout,
        })
    }
}
