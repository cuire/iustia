use std::fmt::Display;
use std::io::Cursor;
use std::str::FromStr;

use anyhow::Error;
use bytes::{Buf, Bytes};

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
    pub(crate) fn from_bytes(buf: &mut Cursor<&[u8]>) -> Result<RespValue, RespParseError> {
        let value = decode_resp(buf)?;

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

    pub fn as_bulk(&self) -> Result<RespValue, Error> {
        match self {
            RespValue::SimpleString(string) => {
                Ok(RespValue::BulkString(string.as_bytes().to_vec()))
            }
            _ => Err(anyhow::anyhow!("Invalid value")),
        }
    }

    pub fn size(&self) -> usize {
        // TODO: optimize this to avoid encoding the value
        let encoded = encode_resp(self);
        encoded.len()
    }
}

// for positional matchers
impl From<&RespValue> for RespValue {
    fn from(value: &RespValue) -> Self {
        value.clone()
    }
}

impl From<&str> for RespValue {
    fn from(s: &str) -> Self {
        if s.is_empty() {
            return RespValue::Null;
        };

        // if string contains any whitespace or newline, treat it as a bulk string
        if s.contains(|c: char| c.is_whitespace() || c == '\r' || c == '\n') {
            return RespValue::BulkString(s.as_bytes().to_vec());
        }

        RespValue::SimpleString(s.to_string())
    }
}

impl TryFrom<RespValue> for Bytes {
    type Error = RespParseError;

    fn try_from(val: RespValue) -> Result<Self, Self::Error> {
        match val {
            RespValue::BulkString(b) => Ok(Bytes::from(b)),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl TryFrom<&RespValue> for String {
    type Error = RespParseError;

    fn try_from(val: &RespValue) -> Result<Self, Self::Error> {
        match val {
            RespValue::SimpleString(s) => Ok(s.clone()),
            RespValue::SimpleError(e) => Ok(e.clone()),
            RespValue::Integer(i) => Ok(i.to_string()),
            RespValue::BulkString(b) => Ok(String::from_utf8_lossy(b).to_string()),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl TryFrom<RespValue> for String {
    type Error = RespParseError;

    fn try_from(val: RespValue) -> Result<Self, Self::Error> {
        match val {
            RespValue::SimpleString(s) => Ok(s),
            RespValue::SimpleError(e) => Ok(e),
            RespValue::Integer(i) => Ok(i.to_string()),
            RespValue::BulkString(b) => Ok(String::from_utf8_lossy(&b).to_string()),
            _ => panic!("Invalid conversion"),
        }
    }
}

impl TryFrom<RespValue> for i64 {
    type Error = RespParseError;

    fn try_from(val: RespValue) -> Result<Self, Self::Error> {
        match val {
            RespValue::Integer(i) => Ok(i),
            RespValue::BulkString(b) => {
                let val_as_str =
                    std::str::from_utf8(&b).map_err(|_| RespParseError::InvalidValue)?;
                val_as_str.parse().map_err(|_| RespParseError::InvalidValue)
            }
            _ => panic!("Invalid conversion"),
        }
    }
}

impl TryFrom<RespValue> for u64 {
    type Error = RespParseError;

    fn try_from(val: RespValue) -> Result<Self, Self::Error> {
        match val {
            RespValue::Integer(i) => Ok(i as u64),
            RespValue::BulkString(b) => {
                let val_as_str =
                    std::str::from_utf8(&b).map_err(|_| RespParseError::InvalidValue)?;
                val_as_str.parse().map_err(|_| RespParseError::InvalidValue)
            }
            _ => panic!("Invalid conversion"),
        }
    }
}

impl Display for RespValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buff = encode_resp(self)
            .iter()
            .map(|b| *b as char)
            .collect::<String>();

        write!(f, "{}", buff)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RespParseError {
    #[error("Invalid type prefix byte was encountered")]
    InvalidTypePrefix,
    #[error("No corresponding newline was found for value")]
    MissingNewline,
    #[error("Buffer is incomplete")]
    Incomplete,
    #[error("Invalid value was found")]
    InvalidValue,
}

fn get_line<'a>(src: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], RespParseError> {
    let start = src.position() as usize;
    let end = src.get_ref().len() - 1;

    for i in start..end {
        if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
            // We found a line, update the position to be *after* the \n
            src.set_position((i + 2) as u64);

            // Return the line
            return Ok(&src.get_ref()[start..i]);
        }
    }

    Err(RespParseError::Incomplete)
}

fn parse_buf_to_string(buf: &[u8]) -> Result<String, RespParseError> {
    let val_as_str = std::str::from_utf8(buf).map_err(|_| RespParseError::InvalidValue)?;
    Ok(val_as_str.to_string())
}

fn parse_buf_to<T: FromStr>(buf: &mut Cursor<&[u8]>) -> Result<T, RespParseError> {
    let line = get_line(buf)?;
    let val_as_str = std::str::from_utf8(line).map_err(|_| RespParseError::InvalidValue)?;
    val_as_str.parse().map_err(|_| RespParseError::InvalidValue)
}

fn decode_resp(buf: &mut Cursor<&[u8]>) -> Result<RespValue, RespParseError> {
    if !buf.has_remaining() {
        return Err(RespParseError::Incomplete);
    }

    let discriminator = buf.get_u8();

    match discriminator {
        // Simple String: `+<data>\r\n`
        b'+' => {
            let val = get_line(buf)?;
            let string = parse_buf_to_string(val)?;
            Ok(RespValue::SimpleString(string))
        }

        // Simple Error: `-<error>\r\n`
        b'-' => {
            let val = get_line(buf)?;
            let string = parse_buf_to_string(val)?;
            Ok(RespValue::SimpleError(string))
        }

        // Integer: `:[<+|->]<value>\r\n`
        b':' => {
            let int = parse_buf_to(buf)?;
            Ok(RespValue::Integer(int))
        }

        // Bulk String: `$<length>\r\n<data>\r\n`
        b'$' => {
            // let (len, rest) = split_at_newline(rest)?;

            // handle bulk string null: `$-1\r\n`
            if buf.chunk()[0] == b'-' {
                return if buf.chunk()[1] == b'1' {
                    Ok(RespValue::Null)
                } else {
                    Err(RespParseError::InvalidValue)
                };
            }

            let len = parse_buf_to::<usize>(buf)?;

            if buf.remaining() < len {
                return Err(RespParseError::Incomplete);
            }

            // 16 is the length of the RDB file header and checksum combined
            if buf.remaining() > 16 {
                if let Some(rdb) = try_parse_rdb(buf, len) {
                    buf.advance(len);
                    return Ok(rdb);
                }
            }

            if buf.remaining() < len + 2 {
                return Err(RespParseError::Incomplete);
            }

            let data = Bytes::copy_from_slice(&buf.chunk()[..len]);

            buf.advance(len + 2);

            Ok(RespValue::BulkString(Vec::from(data)))
        }

        // Array: *<number-of-elements>\r\n<element-1>...<element-n>
        b'*' => {
            let len = parse_buf_to::<usize>(buf)?;
            let mut array = Vec::with_capacity(len);

            for _ in 0..len {
                let next = decode_resp(buf)?;
                array.push(next);
            }

            Ok(RespValue::Array(array))
        }

        _ => Err(RespParseError::InvalidTypePrefix),
    }
}

fn try_parse_rdb(buf: &mut Cursor<&[u8]>, len: usize) -> Option<RespValue> {
    // if start is REDIS (82, 69, 68, 73, 83) and end is FF + {8 bytes of checksum}

    if buf.chunk()[0..5] == [82, 69, 68, 73, 83] {
        let end = &buf.chunk()[len - 9..len];
        if end[0] == 255 {
            // TODO: check checksum

            // TODO: parse RDB file

            // TODO: make a new RespValue::Rdb type

            return Some(RespValue::BulkString(buf.chunk()[..len].to_vec()));
        }
    }

    None
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
        let buf: &mut Cursor<&[u8]> = &mut Cursor::new(b"+Hello, World!\r\n");
        let expected = RespValue::SimpleString("Hello, World!".to_string());
        assert_eq!(RespValue::from_bytes(buf).unwrap(), expected);
    }

    #[test]
    fn test_parse_message_simple_error() {
        let buf: &mut Cursor<&[u8]> = &mut Cursor::new(b"-Error occurred\r\n");
        let expected = RespValue::SimpleError("Error occurred".to_string());
        assert_eq!(RespValue::from_bytes(buf).unwrap(), expected);
    }

    #[test]
    fn test_parse_message_integer() {
        let buf: &mut Cursor<&[u8]> = &mut Cursor::new(b":42\r\n");
        let expected = RespValue::Integer(42);
        assert_eq!(RespValue::from_bytes(buf).unwrap(), expected);
    }

    #[test]
    fn test_parse_message_bulk_string() {
        let buf: &mut Cursor<&[u8]> = &mut Cursor::new(b"$5\r\nHello\r\n");
        let expected = RespValue::BulkString(b"Hello".to_vec());
        assert_eq!(RespValue::from_bytes(buf).unwrap(), expected);
    }

    #[test]
    fn test_parse_message_null() {
        let buf: &mut Cursor<&[u8]> = &mut Cursor::new(b"$-1\r\n");
        let expected = RespValue::Null;
        assert_eq!(RespValue::from_bytes(buf).unwrap(), expected);
    }

    #[test]
    fn test_parse_message_array() {
        let buf: &[u8] = b"*3\r\n+Hello\r\n:42\r\n$5\r\nWorld\r\n";
        let buf = &mut Cursor::new(buf);

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
        let buf: &mut Cursor<&[u8]> = &mut Cursor::new(b"!Hello, World!\r\n");

        assert!(RespValue::from_bytes(buf).is_err());
    }

    #[test]
    fn test_parse_message_missing_newline() {
        let buf: &mut Cursor<&[u8]> = &mut Cursor::new(b"+Hello, World!");

        assert!(RespValue::from_bytes(buf).is_err());
    }

    #[test]
    fn test_parse_message_invalid_value() {
        let buf: &mut Cursor<&[u8]> = &mut Cursor::new(b":Hello, World!\r\n");

        assert!(RespValue::from_bytes(buf).is_err());
    }

    #[test]
    fn test_parse_message_invalid_value_bulk_string() {
        let buf: &mut Cursor<&[u8]> = &mut Cursor::new(b"$5\r\nHello");

        assert!(RespValue::from_bytes(buf).is_err());
    }

    #[test]
    fn test_parse_message_invalid_value_bulk_string_null() {
        let buf: &mut Cursor<&[u8]> = &mut Cursor::new(b"$-2\r\n");

        assert!(RespValue::from_bytes(buf).is_err());
    }

    #[test]
    fn test_parse_message_invalid_value_array() {
        let buf: &[u8] = b"*3\r\n+Hello\r\n:42\r\n$5\r\nWorld";
        let buf = &mut Cursor::new(buf);

        assert!(RespValue::from_bytes(buf).is_err());
    }

    #[test]
    fn test_multiple_parse_message() {
        let buf: &[u8] = b"+Hello, World!\r\n-Error occurred\r\n:42\r\n$5\r\nHello\r\n";
        let buf = &mut Cursor::new(buf);

        let expected = RespValue::SimpleString("Hello, World!".to_string());
        assert_eq!(RespValue::from_bytes(buf).unwrap(), expected);

        let expected = RespValue::SimpleError("Error occurred".to_string());
        assert_eq!(RespValue::from_bytes(buf).unwrap(), expected);

        let expected = RespValue::Integer(42);
        assert_eq!(RespValue::from_bytes(buf).unwrap(), expected);

        let expected = RespValue::BulkString(b"Hello".to_vec());
        assert_eq!(RespValue::from_bytes(buf).unwrap(), expected);
    }

    #[test]
    fn test_parse_rdb_file() {
        let hex_string = "524544495330303131fa0972656469732d76657205372e322e30fa0a72656469732d62697473c040fa056374696d65c26d08bc65fa08757365642d6d656dc2b0c41000fa08616f662d62617365c000fff06e3bfec0ff5aa2";
        let empty_file_payload = (0..hex_string.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex_string[i..i + 2], 16).expect("hex_string is invalid"))
            .collect::<Vec<_>>();

        let len = empty_file_payload.len();

        let payload_len = format!("${}\r\n", len).as_bytes().to_vec();

        let data = [payload_len, empty_file_payload].concat();

        let buf: &mut Cursor<&[u8]> = &mut Cursor::new(&data);

        let resp = RespValue::from_bytes(buf);

        assert!(resp.is_ok());
    }
}
