#[macro_export]
macro_rules! next_arg {
    // for positional matchers
    ($args:expr) => {{
        let resp = $args.next();

        if let Some(resp) = resp {
            resp.try_into()
                .map_err(|_| anyhow::anyhow!("Invalid argument type"))
        } else {
            Err(anyhow::anyhow!("Invalid arguments, missing"))
        }
    }};

    ($args:expr, optional) => {{
        let resp = $args.next();

        if let Some(resp) = resp {
            resp.try_into()
                .map_err(|_| anyhow::anyhow!("Invalid argument type"))
        } else {
            Ok(None)
        }
    }};

    // for keyword matchers
    ($args:expr, keyword = $keyword:expr) => {{
        let key: String = next_arg!($args)?;

        if key.to_lowercase() != $keyword {
            return Err(anyhow::anyhow!(
                "Invalid arguments, expected {} keyword",
                stringify!($keyword)
            ));
        }

        next_arg!($args)?
    }};

    // single flag matcher
    ($args:expr, flag = $keyword:expr) => {
        {
            // create a new iterator to avoid consuming the original one
            let mut args = $args.clone();
            let key: String = next_arg!(args).unwrap_or_default();

            if key.to_lowercase() != $keyword {
                false
            } else {
                // if the flag is present, consume the next argument from the original iterator
                $args.next();

                true
            }
        }
    };

    // for keyword matchers with options
    ($args:expr, $($keyword:expr => $handler:expr),+) => {{
        let key: String = next_arg!($args);
        let value: RespValue = next_arg!($args)?;

        match key.to_lowercase().as_str() {
            $(
                $keyword => ($handler)(value.try_into().map_err(|_| anyhow::anyhow!("Invalid argument type"))? as _),
            )+
            _ => {
                return Err(anyhow::anyhow!("Invalid arguments, expected one of the following keywords: {}", stringify!($($keyword),+)));
            }
        }}};

        // for optional keyword matchers with options
        ($args:expr, optional, $($keyword:expr => $handler:expr),+) => {{
            let key: Result<String, _> = next_arg!($args);

            if let Ok(key) = key {


            let value: RespValue = next_arg!($args)?;

            match key.to_lowercase().as_str() {
                $(
                    $keyword => {
                        Some(
                            ($handler)(value.try_into().map_err(|_| anyhow::anyhow!("Invalid argument type"))?)
                        )
                    }
                )+
                _ => {
                    None
                }
            }
        } else {
            None
        }
    }};
}

#[cfg(test)]
mod tests {
    use crate::next_arg;
    use crate::resp::RespValue;
    use std::convert::TryInto;

    #[test]
    fn test_next_arg() {
        let mut args = vec![
            RespValue::BulkString(b"key".to_vec()),
            RespValue::BulkString(b"value".to_vec()),
        ]
        .into_iter();

        let key: String = next_arg!(args).unwrap();
        let value: String = next_arg!(args).unwrap();

        assert_eq!(key, "key");
        assert_eq!(value, "value");
    }
}
