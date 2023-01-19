use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::RwLock};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Index {
    pub created_at: RwLock<BTreeMap<i64, Uuid>>,
    pub key: RwLock<BTreeMap<String, Uuid>>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            created_at: RwLock::new(BTreeMap::new()),
            key: RwLock::new(BTreeMap::new()),
        }
    }
}

impl Default for Index {
    fn default() -> Self {
        Self::new()
    }
}
