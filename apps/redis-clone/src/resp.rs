use std::str::FromStr;

use anyhow::Error;

const CRLF: &[u8] = b"\r\n";

/*
https://redis.io/docs/reference/protocol-spec/
 */

#[derive(Debug, PartialEq, Clone)]
pub enum RespValue {
    SimpleString(String),
    SimpleError(String),
    Integer(i64),
    BulkString(Vec<u8>),
    Null,
    Array(Vec<RespValue>),
}

/// Represents a Redis Protocol (RESP) value.
///
/// This struct provides methods for creating a new `RespValue` from a byte slice and
/// serializing a `RespValue` to a byte vector.
impl RespValue {
    /// Creates a new `RespValue` from a byte slice.
    ///
    /// # Arguments
    ///
    /// * `buf` - A byte slice containing the RESP value.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the parsed `RespValue` if successful, or a `RespParseError` if parsing fails.
    pub(crate) fn from_bytes(buf: &[u8]) -> Result<RespValue, RespParseError> {
        let (value, rest) = decode_resp(buf)?;
        assert!(rest.is_empty());

        return Ok(value);
    }

    /// Serializes the `RespValue` to a byte vector.
    ///
    /// # Returns
    ///
    /// Returns a byte vector containing the serialized `RespValue`.
    pub(crate) fn to_buf(&self) -> Vec<u8> {
        encode_resp(self)
    }

    pub fn as_integer(&self) -> Result<i64, Error> {
        match self {
            RespValue::Integer(i) => Ok(*i),
            RespValue::BulkString(s) => {
                let val_as_str = std::str::from_utf8(s).unwrap();
                Ok(val_as_str.parse().unwrap())
            }
            _ => Err(anyhow::anyhow!("Invalid value")),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RespParseError {
    #[error("Invalid type prefix byte was encountered")]
    InvalidTypePrefix,
    #[error("No corresponding newline was found for value")]
    MissingNewline,
    #[error("Invalid value was found")]
    InvalidValue,
}

fn split_at_newline(buf: &[u8]) -> Result<(&[u8], &[u8]), RespParseError> {
    buf.windows(2)
        .position(|w| w == CRLF)
        .map(|i| {
            let (x, y) = buf.split_at(i);

            (x, &y[2..])
        })
        .ok_or(RespParseError::MissingNewline)
}

fn parse_buf_to_string(buf: &[u8]) -> Result<String, RespParseError> {
    let val_as_str = std::str::from_utf8(buf).map_err(|_| RespParseError::InvalidValue)?;
    Ok(val_as_str.to_string())
}

fn parse_buf_to<T: FromStr>(buf: &[u8]) -> Result<T, RespParseError> {
    let val_as_str = std::str::from_utf8(buf).map_err(|_| RespParseError::InvalidValue)?;
    val_as_str.parse().map_err(|_| RespParseError::InvalidValue)
}

fn decode_resp(buf: &[u8]) -> Result<(RespValue, &[u8]), RespParseError> {
    let (discriminator, rest) = buf.split_first().unwrap();
    match discriminator {
        // Simple String: `+<data>\r\n`
        b'+' => {
            let (val, rest) = split_at_newline(rest)?;
            let val_as_string = parse_buf_to_string(val)?;
            Ok((RespValue::SimpleString(val_as_string), rest))
        }

        // Simple Error: `-<error>\r\n`
        b'-' => {
            let (val, rest) = split_at_newline(rest)?;
            let val_as_string = parse_buf_to_string(val)?;
            Ok((RespValue::SimpleError(val_as_string), rest))
        }

        // Integer: `:[<+|->]<value>\r\n`
        b':' => {
            let (val, rest) = split_at_newline(rest)?;
            let int = parse_buf_to(val)?;
            Ok((RespValue::Integer(int), rest))
        }

        // Bulk String: `$<length>\r\n<data>\r\n`
        b'$' => {
            let (len, rest) = split_at_newline(rest)?;

            // handle bulk string null: `$-1\r\n`
            if len[0] == b'-' {
                return if len[1] == b'1' {
                    Ok((RespValue::Null, rest))
                } else {
                    Err(RespParseError::InvalidValue)
                };
            }

            let len = parse_buf_to::<u64>(len)?;

            let (data, rest) = rest.split_at(len as usize);

            if rest.len() < 2 || &rest[..2] != CRLF {
                return Err(RespParseError::MissingNewline);
            }

            Ok((RespValue::BulkString(Vec::from(data)), &rest[2..]))
        }

        // Array: *<number-of-elements>\r\n<element-1>...<element-n>
        b'*' => {
            let (len, mut rest) = split_at_newline(rest)?;
            let len = parse_buf_to::<u64>(len)?;

            let mut array = Vec::with_capacity(len as usize);

            for _ in 0..len {
                let (next, next_rest) = decode_resp(rest)?;
                array.push(next);
                rest = next_rest;
            }

            Ok((RespValue::Array(array), rest))
        }

        _ => Err(RespParseError::InvalidTypePrefix),
    }
}

fn encode_resp(val: &RespValue) -> Vec<u8> {
    match val {
        RespValue::Null => b"$-1\r\n".to_vec(),
        RespValue::SimpleString(s) => format!("+{s}\r\n").into_bytes(),
        RespValue::SimpleError(e) => format!("-{e}\r\n").into_bytes(),
        RespValue::Integer(i) => format!(":{i}\r\n").into_bytes(),
        RespValue::BulkString(s) => {
            let len = s.len();
            let start = format!("${len}\r\n");
            let mut buf = Vec::with_capacity(start.len() + len + 2);
            buf.extend(start.as_bytes());
            buf.extend(s);
            buf.extend(CRLF);
            buf
        }
        RespValue::Array(a) => {
            let len = a.len();
            let start = format!("*{len}\r\n");
            let mut buf = Vec::with_capacity(start.len() + len * 3);
            buf.extend(start.as_bytes());

            for e in a {
                buf.extend(encode_resp(e));
            }

            buf
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_message_simple_string() {
        let buf = b"+Hello, World!\r\n";
        let expected = RespValue::SimpleString("Hello, World!".to_string());
        assert_eq!(RespValue::from_bytes(buf).unwrap(), expected);
    }

    #[test]
    fn test_parse_message_simple_error() {
        let buf = b"-Error occurred\r\n";
        let expected = RespValue::SimpleError("Error occurred".to_string());
        assert_eq!(RespValue::from_bytes(buf).unwrap(), expected);
    }

    #[test]
    fn test_parse_message_integer() {
        let buf = b":42\r\n";
        let expected = RespValue::Integer(42);
        assert_eq!(RespValue::from_bytes(buf).unwrap(), expected);
    }

    #[test]
    fn test_parse_message_bulk_string() {
        let buf = b"$5\r\nHello\r\n";
        let expected = RespValue::BulkString(b"Hello".to_vec());
        assert_eq!(RespValue::from_bytes(buf).unwrap(), expected);
    }

    #[test]
    fn test_parse_message_null() {
        let buf = b"$-1\r\n";
        let expected = RespValue::Null;
        assert_eq!(RespValue::from_bytes(buf).unwrap(), expected);
    }

    #[test]
    fn test_parse_message_array() {
        let buf = b"*3\r\n+Hello\r\n:42\r\n$5\r\nWorld\r\n";
        let expected = RespValue::Array(vec![
            RespValue::SimpleString("Hello".to_string()),
            RespValue::Integer(42),
            RespValue::BulkString(b"World".to_vec()),
        ]);
        assert_eq!(RespValue::from_bytes(buf).unwrap(), expected);
    }

    #[test]
    fn test_serialize_resp_value_to_buf_simple_string() {
        let val = RespValue::SimpleString("Hello, World!".to_string());
        let expected = b"+Hello, World!\r\n".to_vec();
        assert_eq!(encode_resp(&val), expected);
    }

    #[test]
    fn test_serialize_resp_value_to_buf_simple_error() {
        let val = RespValue::SimpleError("Error occurred".to_string());
        let expected = b"-Error occurred\r\n".to_vec();
        assert_eq!(encode_resp(&val), expected);
    }

    #[test]
    fn test_serialize_resp_value_to_buf_integer() {
        let val = RespValue::Integer(42);
        let expected = b":42\r\n".to_vec();
        assert_eq!(encode_resp(&val), expected);
    }

    #[test]
    fn test_serialize_resp_value_to_buf_bulk_string() {
        let val = RespValue::BulkString(b"Hello".to_vec());
        let expected = b"$5\r\nHello\r\n".to_vec();
        assert_eq!(encode_resp(&val), expected);
    }

    #[test]
    fn test_serialize_resp_value_to_buf_null() {
        let val = RespValue::Null;
        let expected = b"$-1\r\n".to_vec();
        assert_eq!(encode_resp(&val), expected);
    }

    #[test]
    fn test_serialize_resp_value_to_buf_array() {
        let val = RespValue::Array(vec![
            RespValue::SimpleString("Hello".to_string()),
            RespValue::Integer(42),
            RespValue::BulkString(b"World".to_vec()),
        ]);
        let expected = b"*3\r\n+Hello\r\n:42\r\n$5\r\nWorld\r\n".to_vec();
        assert_eq!(encode_resp(&val), expected);
    }

    #[test]
    fn test_parse_message_invalid_type_prefix() {
        let buf = b"!Hello, World!\r\n";
        assert!(RespValue::from_bytes(buf).is_err());
    }

    #[test]
    fn test_parse_message_missing_newline() {
        let buf = b"+Hello, World!";
        assert!(RespValue::from_bytes(buf).is_err());
    }

    #[test]
    fn test_parse_message_invalid_value() {
        let buf = b":Hello, World!\r\n";
        assert!(RespValue::from_bytes(buf).is_err());
    }

    #[test]
    fn test_parse_message_invalid_value_bulk_string() {
        let buf = b"$5\r\nHello";
        assert!(RespValue::from_bytes(buf).is_err());
    }

    #[test]
    fn test_parse_message_invalid_value_bulk_string_null() {
        let buf = b"$-2\r\n";
        assert!(RespValue::from_bytes(buf).is_err());
    }

    #[test]
    fn test_parse_message_invalid_value_array() {
        let buf = b"*3\r\n+Hello\r\n:42\r\n$5\r\nWorld";
        assert!(RespValue::from_bytes(buf).is_err());
    }
}
