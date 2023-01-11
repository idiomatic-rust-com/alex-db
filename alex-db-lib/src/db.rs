use crate::{
    value_record::{ValuePost, ValuePut, ValueRecord, ValueResponse},
    Result,
};
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use serde::{Deserialize, Serialize};
use std::{
    collections::{hash_map::RandomState, HashMap},
    fs,
    path::Path,
    sync::{RwLock, RwLockWriteGuard},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Db {
    pub database: RwLock<HashMap<String, ValueRecord>>,
    data_dir: Option<String>,
}

impl Db {
    pub fn new(data_dir: Option<String>) -> Self {
        Self {
            database: RwLock::new(HashMap::new()),
            data_dir,
        }
    }

    pub fn restore(&mut self) -> Result<()> {
        if let Some(data_dir) = &self.data_dir {
            let database_file_path = format!("{dir}{file}", dir = data_dir, file = "db.dat");

            if Path::new(&database_file_path).exists() {
                let compressed = fs::read(database_file_path)?;
                let uncompressed = decompress_size_prepended(&compressed)?;
                let serialized = String::from_utf8(uncompressed)?;

                self.database = serde_json::from_str(&serialized)?;
            }
        }

        Ok(())
    }

    pub fn save(
        &self,
        database: RwLockWriteGuard<HashMap<String, ValueRecord, RandomState>>,
    ) -> Result<()> {
        if let Some(data_dir) = &self.data_dir {
            let database_file_path = format!("{dir}{file}", dir = data_dir, file = "db.dat");
            let serialized = serde_json::to_vec(&*database)?;
            let compressed = compress_prepend_size(&serialized);

            fs::write(database_file_path, compressed)?;
        }

        Ok(())
    }

    pub fn select_all(&self) -> Result<Vec<ValueResponse>> {
        let database = self.database.read().unwrap();
        let mut result = vec![];

        for (_key, val) in database.iter() {
            let val = val.clone();
            result.append(&mut vec![val.into()]);
        }

        Ok(result)
    }

    pub fn try_delete(&self, key: &str) -> Result<Option<ValueResponse>> {
        let mut database = self.database.write().unwrap();
        let result = database.remove(key);

        match result {
            None => Ok(None),
            Some(result) => Ok(Some(result.into())),
        }
    }

    pub fn try_insert(&self, key: String, value: ValuePost) -> Result<Option<ValueResponse>> {
        let mut database = self.database.write().unwrap();
        database.insert(key.clone(), value.into());
        let result = database.get(&key).cloned();
        self.save(database)?;

        match result {
            None => Ok(None),
            Some(result) => Ok(Some(result.into())),
        }
    }

    pub fn try_select(&self, key: &str) -> Result<Option<ValueResponse>> {
        let database = self.database.read().unwrap();
        let result = database.get(key).cloned();

        match result {
            None => Ok(None),
            Some(result) => Ok(Some(result.into())),
        }
    }

    pub fn try_upsert(&self, key: String, value: ValuePut) -> Result<Option<ValueResponse>> {
        let mut database = self.database.write().unwrap();
        database.insert(key.clone(), value.into());
        let result = database.get(&key).cloned();
        self.save(database)?;

        match result {
            None => Ok(None),
            Some(result) => Ok(Some(result.into())),
        }
    }
}
