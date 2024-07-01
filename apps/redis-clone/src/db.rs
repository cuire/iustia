use radix_trie::{Trie, TrieCommon};
use tokio::sync::Mutex;
use tokio::time::Instant;

use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use crate::conf::Config;

#[derive(Debug)]
pub(crate) struct DbBuilder {
    db: Db,
}

#[derive(Debug, Clone)]
pub(crate) struct Db {
    shared: Arc<Shared>,
}

#[derive(Debug)]
struct Shared {
    state: Mutex<State>,
}

#[derive(Debug)]
struct State {
    entries: HashMap<String, Entry>,
    streams: HashMap<String, Stream>,
    config: Config,
}

/// Entry in the key-value store
#[derive(Debug)]
struct Entry {
    data: Bytes,
    expires_at: Option<Instant>,
}

#[derive(Debug, Clone)]
struct StreamID {
    millis: u128,
    seq: u64,
}

impl StreamID {
    fn new(string: String, last_id: &Option<Self>) -> Result<Self, anyhow::Error> {
        if string == "*" && last_id.is_some() {
            let mut last_id = last_id.clone().unwrap();
            last_id.seq += 1;
            return Ok(last_id);
        }

        if string == "*" {
            return Ok(Self {
                millis: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u128,
                seq: 0,
            });
        }

        let parts = string.split("-").collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid Stream ID"));
        }

        let millis = parts[0].parse().unwrap();

        let seq = match parts[1] {
            "*" => {
                if last_id.is_none() || millis != last_id.clone().unwrap().millis {
                    if millis > 0 {
                        0
                    } else {
                        1
                    }
                } else {
                    last_id.clone().unwrap().seq + 1
                }
            }
            _ => parts[1].parse().unwrap(),
        };

        if let Some(last_id) = last_id {
            if millis < last_id.millis {
                return Err(anyhow::anyhow!("Invalid Stream ID"));
            }

            if millis == last_id.millis && seq <= last_id.seq {
                return Err(anyhow::anyhow!("Invalid Stream ID"));
            }
        }

        Ok(Self { millis, seq })
    }

    fn xrange(string: String, seq: Option<u64>) -> Result<Self, anyhow::Error> {
        // if seq is not specified, it is 0
        if !string.contains("-") && seq.is_some() {
            return Ok(Self {
                millis: string.parse().unwrap(),
                seq: seq.unwrap(),
            });
        }

        let parts = string.split("-").collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid Stream ID"));
        }

        let millis = parts[0].parse().unwrap();
        let seq = parts[1].parse().unwrap();

        Ok(Self { millis, seq })
    }
}

impl Into<String> for StreamID {
    fn into(self) -> String {
        format!("{}-{}", self.millis, self.seq)
    }
}

#[derive(Debug)]
struct Stream {
    entries: Trie<String, StreamEntry>,
    last_id: Option<StreamID>,
}

#[derive(Debug, Clone)]
pub struct StreamEntry {
    key: String,
    data: Bytes,
}

impl StreamEntry {
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn data(&self) -> &Bytes {
        &self.data
    }
}

impl DbBuilder {
    pub(crate) fn new(config: Config) -> DbBuilder {
        DbBuilder {
            db: Db::new(config),
        }
    }

    pub(crate) fn db(&self) -> Db {
        self.db.clone()
    }
}

impl Db {
    pub(crate) fn new(config: Config) -> Db {
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                entries: HashMap::new(),
                streams: HashMap::new(),
                config: config,
            }),
        });

        Db { shared }
    }

    pub(crate) async fn get(&self, key: &str) -> Option<Bytes> {
        let state = self.shared.state.lock().await;

        let entry = state.entries.get(key)?;

        if let Some(expires_at) = entry.expires_at {
            if Instant::now() >= expires_at {
                return None;
            }
        }

        Some(entry.data.clone())
    }

    pub(crate) async fn set(&self, key: String, value: Bytes, expires_at: Option<Duration>) {
        let mut state = self.shared.state.lock().await;

        let expires_at = expires_at.map(|duration| Instant::now() + duration);

        state.entries.insert(
            key.clone(),
            Entry {
                data: value,
                expires_at,
            },
        );
    }

    pub(crate) async fn xadd(
        &self,
        stream: &str,
        id: Option<String>,
        key: String,
        value: Bytes,
    ) -> Result<String, anyhow::Error> {
        let mut state = self.shared.state.lock().await;

        let stream = state
            .streams
            .entry(stream.to_string())
            .or_insert_with(|| Stream {
                entries: Trie::new(),
                last_id: None,
            });

        if id.is_none() {
            unimplemented!();
        }
        let id: String = id.unwrap();

        let stream_id = StreamID::new(id.clone(), &stream.last_id)?;

        stream.last_id = Some(stream_id.clone());

        let stream_id: String = stream_id.into();

        stream
            .entries
            .insert(stream_id.clone(), StreamEntry { key, data: value });

        Ok(stream_id.into())
    }

    pub(crate) async fn xrange(
        &self,
        stream: &str,
        start: &str,
        end: &str,
        count: Option<usize>,
    ) -> Result<Vec<(String, StreamEntry)>, anyhow::Error> {
        let state = self.shared.state.lock().await;

        let stream = state
            .streams
            .get(stream)
            .ok_or(anyhow::anyhow!("Stream not found"))?;

        let mut entries = vec![];

        // find common prefix of start and end, if start or/and end are not specified
        // then common prefix is empty string and subtrie is the whole stream
        let common_prefix = start
            .chars()
            .zip(end.chars())
            .take_while(|(a, b)| a == b)
            .map(|(a, _)| a)
            .collect::<String>();

        // take subtrie of common prefix and iterate over it
        let subtrie = stream.entries.get_raw_descendant(&common_prefix);

        if subtrie.is_none() {
            return Ok(entries);
        }

        let subtrie = subtrie.unwrap();

        // if start seq is not specified, it is 0 by default
        // but it can be not present in the stream, so we need to find
        // the first entry or check all entries in the subtrie
        let start_id = match start {
            "-" => StreamID { millis: 0, seq: 0 },
            _ => StreamID::xrange(String::from(start), Some(0))?,
        };

        let end_id = match end {
            "+" => StreamID {
                millis: u128::MAX,
                seq: u64::MAX,
            },
            _ => StreamID::xrange(String::from(end), Some(u64::MAX))?,
        };

        let mut count = count.unwrap_or(usize::MAX);

        for (id, entry) in subtrie.iter().take(count) {
            if count == 0 {
                break;
            }

            let id = StreamID::xrange(id.clone(), None).unwrap();

            if id.millis < start_id.millis {
                continue;
            }

            if id.millis == start_id.millis && id.seq < start_id.seq {
                continue;
            }

            if id.millis > end_id.millis {
                continue;
            }

            if id.millis == end_id.millis && id.seq > end_id.seq {
                continue;
            }

            count -= 1;

            entries.push((id.into(), entry.clone()));
        }

        Ok(entries)
    }

    pub(crate) async fn xread(&self, stream_key: String, id: String) -> Option<StreamEntry> {
        let entry = self.xrange(&stream_key, &id, "+", Some(1)).await.ok()?[0]
            .1
            .clone();

        Some(entry)
    }

    pub(crate) async fn keys(&self) -> Vec<String> {
        let state = self.shared.state.lock().await;

        // Filter out expired keys and clone the keys
        state
            .entries
            .iter()
            .filter(|(_, entry)| {
                if let Some(expires_at) = entry.expires_at {
                    Instant::now() < expires_at
                } else {
                    true
                }
            })
            .map(|(key, _)| key.clone())
            .collect()
    }

    pub(crate) async fn value_type(&self, key: &str) -> String {
        let state = self.shared.state.lock().await;

        if state.entries.contains_key(key) {
            return "string".to_string();
        }

        if state.streams.contains_key(key) {
            return "stream".to_string();
        }

        "none".to_string()
    }

    pub(crate) async fn config(&self) -> Config {
        let state = self.shared.state.lock().await;
        state.config.clone()
    }
}
