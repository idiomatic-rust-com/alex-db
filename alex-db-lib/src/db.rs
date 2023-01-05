use crate::{
    db_record::{DbRecord, ValuePost, ValuePut, ValueResponse},
    Result,
};
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

    pub fn select_all(&self) -> Result<Vec<ValueResponse>> {
        let database = self.database.lock().unwrap();
        let mut result = vec![];

        for (_key, val) in database.iter() {
            let val = val.clone();
            result.append(&mut vec![val.into()]);
        }

        Ok(result)
    }

    pub fn try_delete(&self, key: &str) -> Result<Option<ValueResponse>> {
        let mut database = self.database.lock().unwrap();
        let result = database.remove(key);

        match result {
            None => Ok(None),
            Some(result) => Ok(Some(result.into())),
        }
    }

    pub fn try_insert(&self, key: String, value: ValuePost) -> Result<Option<ValueResponse>> {
        let mut database = self.database.lock().unwrap();
        database.insert(key.clone(), value.into());
        let result = database.get(&key).cloned();

        match result {
            None => Ok(None),
            Some(result) => Ok(Some(result.into())),
        }
    }

    pub fn try_select(&self, key: &str) -> Result<Option<ValueResponse>> {
        let database = self.database.lock().unwrap();
        let result = database.get(key).cloned();

        match result {
            None => Ok(None),
            Some(result) => Ok(Some(result.into())),
        }
    }

    pub fn try_upsert(&self, key: String, value: ValuePut) -> Result<Option<ValueResponse>> {
        let mut database = self.database.lock().unwrap();
        database.insert(key.clone(), value.into());
        let result = database.get(&key).cloned();

        match result {
            None => Ok(None),
            Some(result) => Ok(Some(result.into())),
        }
    }
}
