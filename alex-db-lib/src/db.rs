use crate::{
    stat_record::StatRecord,
    value_record::{ValuePost, ValuePut, ValueRecord, ValueResponse},
    Result,
};
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path, sync::RwLock};

#[derive(Debug, Deserialize, Serialize)]
pub struct Db {
    data_dir: Option<String>,
    saved_writes_threshold: u16,
    pub stats: RwLock<StatRecord>,
    pub values: RwLock<HashMap<String, ValueRecord>>,
}

impl Db {
    pub fn new(data_dir: Option<String>, saved_writes_threshold: u16) -> Self {
        Self {
            data_dir,
            saved_writes_threshold,
            stats: RwLock::new(StatRecord::default()),
            values: RwLock::new(HashMap::new()),
        }
    }

    pub fn get_stats(&self) -> Result<StatRecord> {
        let stats = self.stats.read().unwrap().to_owned();

        Ok(stats)
    }

    pub fn restore(&mut self) -> Result<()> {
        if let Some(data_dir) = &self.data_dir {
            let values_file_path = format!("{dir}{file}", dir = data_dir, file = "values.dat");

            if Path::new(&values_file_path).exists() {
                let compressed = fs::read(values_file_path)?;
                let uncompressed = decompress_size_prepended(&compressed)?;
                let serialized = String::from_utf8(uncompressed)?;

                self.values = serde_json::from_str(&serialized)?;
            }
        }

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        if let Some(data_dir) = &self.data_dir {
            let mut stats = self.stats.write().unwrap();

            if stats.can_save(self.saved_writes_threshold) {
                let values = self.values.read().unwrap();

                let values_file_path = format!("{dir}{file}", dir = data_dir, file = "values.dat");
                let serialized = serde_json::to_vec(&*values)?;
                let compressed = compress_prepend_size(&serialized);

                fs::write(values_file_path, compressed)?;

                stats.update_saved_writes();
            }
        }

        Ok(())
    }

    pub fn select_all(&self) -> Result<Vec<ValueResponse>> {
        let mut stats = self.stats.write().unwrap();
        stats.inc_requests();
        let values = self.values.read().unwrap();
        let mut result = vec![];

        for (_key, val) in values.iter() {
            let val = val.clone();
            result.append(&mut vec![val.into()]);
            stats.inc_reads();
        }

        Ok(result)
    }

    pub fn try_delete(&self, key: &str) -> Result<Option<ValueResponse>> {
        let mut stats = self.stats.write().unwrap();
        stats.inc_requests();
        let mut values = self.values.write().unwrap();
        let result = values.remove(key);

        match result {
            None => Ok(None),
            Some(result) => {
                stats.inc_writes();

                Ok(Some(result.into()))
            }
        }
    }

    pub fn try_insert(&self, key: String, value: ValuePost) -> Result<Option<ValueResponse>> {
        let mut stats = self.stats.write().unwrap();
        stats.inc_requests();
        let mut values = self.values.write().unwrap();
        values.insert(key.clone(), value.into());
        let result = values.get(&key).cloned();

        match result {
            None => Ok(None),
            Some(result) => {
                stats.inc_writes();

                Ok(Some(result.into()))
            }
        }
    }

    pub fn try_select(&self, key: &str) -> Result<Option<ValueResponse>> {
        let mut stats = self.stats.write().unwrap();
        stats.inc_requests();
        let values = self.values.read().unwrap();
        let result = values.get(key).cloned();

        match result {
            None => Ok(None),
            Some(result) => {
                stats.inc_reads();

                Ok(Some(result.into()))
            }
        }
    }

    pub fn try_upsert(&self, key: String, value: ValuePut) -> Result<Option<ValueResponse>> {
        let mut stats = self.stats.write().unwrap();
        stats.inc_requests();
        let mut values = self.values.write().unwrap();
        values.insert(key.clone(), value.into());
        let result = values.get(&key).cloned();

        match result {
            None => Ok(None),
            Some(result) => {
                stats.inc_writes();

                Ok(Some(result.into()))
            }
        }
    }
}
