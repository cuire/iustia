use tokio::sync::Mutex;
use tokio::time::Instant;

use bytes::Bytes;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug)]
pub(crate) struct DbHolder {
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
}

/// Entry in the key-value store
#[derive(Debug)]
struct Entry {
    data: Bytes,
    expires_at: Option<Instant>,
}

impl DbHolder {
    pub(crate) fn new() -> DbHolder {
        DbHolder { db: Db::new() }
    }

    pub(crate) fn db(&self) -> Db {
        self.db.clone()
    }
}

impl Db {
    pub(crate) fn new() -> Db {
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                entries: HashMap::new(),
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
}
