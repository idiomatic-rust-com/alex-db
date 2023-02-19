use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::RwLock};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct ValueIndex {
    pub created_at: RwLock<BTreeMap<i64, Uuid>>,
    pub delete_at: RwLock<BTreeMap<i64, Uuid>>,
    pub key: RwLock<BTreeMap<String, Uuid>>,
    pub updated_at: RwLock<BTreeMap<i64, Uuid>>,
}

impl ValueIndex {
    pub fn new() -> Self {
        Self {
            created_at: RwLock::new(BTreeMap::new()),
            delete_at: RwLock::new(BTreeMap::new()),
            key: RwLock::new(BTreeMap::new()),
            updated_at: RwLock::new(BTreeMap::new()),
        }
    }
}

impl Default for ValueIndex {
    fn default() -> Self {
        Self::new()
    }
}
