use crate::{db_record::DbRecord, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};

#[derive(Debug, Deserialize, Serialize)]
pub struct Db {
    pub database: Mutex<HashMap<String, DbRecord>>,
    data_dir: Option<String>,
}

impl Db {
    pub fn new(data_dir: Option<String>) -> Self {
        Self {
            database: Mutex::new(HashMap::new()),
            data_dir,
        }
    }

    pub fn get(&self, key: &str) -> Result<Option<DbRecord>> {
        let database = self.database.lock().unwrap();

        let result = database.get(key).cloned();

        Ok(result)
    }

    pub fn insert(&self, key: String, value: DbRecord) -> Result<Option<DbRecord>> {
        let mut database = self.database.lock().unwrap();
        let result = database.insert(key, value);

        Ok(result)
    }
}
