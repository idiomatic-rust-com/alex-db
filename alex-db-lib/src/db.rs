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

    pub fn select_all(&self) -> Result<Vec<DbRecord>> {
        let database = self.database.lock().unwrap();
        let mut result = vec![];

        for (_key, val) in database.iter() {
            let val = val.clone();
            result.append(&mut vec![val]);
        }

        Ok(result)
    }

    pub fn try_delete(&self, key: &str) -> Result<Option<DbRecord>> {
        let mut database = self.database.lock().unwrap();
        let result = database.remove(key);

        Ok(result)
    }

    pub fn try_insert(&self, key: String, value: DbRecord) -> Result<Option<DbRecord>> {
        let result = self.try_upsert(key, value)?;

        Ok(result)
    }

    pub fn try_select(&self, key: &str) -> Result<Option<DbRecord>> {
        let database = self.database.lock().unwrap();
        let result = database.get(key).cloned();

        Ok(result)
    }

    pub fn try_upsert(&self, key: String, value: DbRecord) -> Result<Option<DbRecord>> {
        let mut database = self.database.lock().unwrap();
        database.insert(key.clone(), value);
        let result = database.get(&key).cloned();

        Ok(result)
    }
}
