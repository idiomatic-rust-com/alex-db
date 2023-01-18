use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, sync::RwLock};

#[derive(Debug, Deserialize, Serialize)]
pub struct Index {
    pub created_at: RwLock<BTreeMap<i64, String>>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            created_at: RwLock::new(BTreeMap::new()),
        }
    }
}

impl Default for Index {
    fn default() -> Self {
        Self::new()
    }
}
