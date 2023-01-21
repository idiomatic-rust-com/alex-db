use crate::{
    error::Error,
    index::Index,
    stat_record::StatRecord,
    value_record::{ValuePost, ValuePut, ValueRecord, ValueResponse},
    Result,
};
use chrono::Utc;
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path, sync::RwLock};
use uuid::Uuid;

pub const API_KEYS_FILE: &str = "api_keys.sec";
pub const CREATED_AT_INDEX_FILE: &str = "created_at.idx";
pub const DATABASE_FILE: &str = "values.db";
pub const KEY_INDEX_FILE: &str = "key.idx";
pub const UPDATED_AT_INDEX_FILE: &str = "updated_at.idx";

#[derive(Debug, Deserialize, Serialize)]
pub struct Db {
    api_keys: RwLock<Vec<Uuid>>,
    data_dir: Option<String>,
    pub indexes: Index,
    pub restricted_access: bool,
    saved_writes_threshold: u16,
    saved_writes_trigger_after: i64,
    pub stats: RwLock<StatRecord>,
    pub values: RwLock<HashMap<Uuid, ValueRecord>>,
}

impl Db {
    pub fn new(
        data_dir: Option<String>,
        restricted_access: bool,
        saved_writes_threshold: u16,
        saved_writes_trigger_after: i64,
    ) -> Self {
        Self {
            api_keys: RwLock::new(vec![]),
            data_dir,
            indexes: Index::default(),
            restricted_access,
            saved_writes_threshold,
            saved_writes_trigger_after,
            stats: RwLock::new(StatRecord::default()),
            values: RwLock::new(HashMap::new()),
        }
    }

    pub fn api_key_exists(&self, api_key: Uuid) -> Result<bool> {
        let api_keys = self.api_keys.read().unwrap();

        let result = api_keys.contains(&api_key);

        Ok(result)
    }

    pub fn api_key_init(&self) -> Result<Option<Uuid>> {
        let mut api_keys = self.api_keys.write().unwrap();

        if api_keys.is_empty() {
            let api_key = Uuid::new_v4();
            api_keys.append(&mut vec![api_key]);

            return Ok(Some(api_key));
        }

        Ok(None)
    }

    pub fn get_stats(&self) -> Result<StatRecord> {
        let stats = self.stats.read().unwrap().to_owned();

        Ok(stats)
    }

    pub fn restore(&mut self) -> Result<()> {
        if let Some(data_dir) = &self.data_dir {
            let values_file_path = format!("{}/{}", data_dir, DATABASE_FILE);
            if Path::new(&values_file_path).exists() {
                let compressed = fs::read(values_file_path)?;
                let uncompressed = decompress_size_prepended(&compressed)?;
                let serialized = String::from_utf8(uncompressed)?;
                self.values = serde_json::from_str(&serialized)?;
            }

            let api_keys_file_path = format!("{}/{}", data_dir, API_KEYS_FILE);
            if Path::new(&api_keys_file_path).exists() {
                let compressed = fs::read(api_keys_file_path)?;
                let uncompressed = decompress_size_prepended(&compressed)?;
                let serialized = String::from_utf8(uncompressed)?;
                self.api_keys = serde_json::from_str(&serialized)?;
            }

            let created_at_index_file_path = format!("{}/{}", data_dir, CREATED_AT_INDEX_FILE);
            if Path::new(&created_at_index_file_path).exists() {
                let compressed = fs::read(created_at_index_file_path)?;
                let uncompressed = decompress_size_prepended(&compressed)?;
                let serialized = String::from_utf8(uncompressed)?;
                self.indexes.created_at = serde_json::from_str(&serialized)?;
            }

            let key_index_file_path = format!("{}/{}", data_dir, KEY_INDEX_FILE);
            if Path::new(&key_index_file_path).exists() {
                let compressed = fs::read(key_index_file_path)?;
                let uncompressed = decompress_size_prepended(&compressed)?;
                let serialized = String::from_utf8(uncompressed)?;
                self.indexes.key = serde_json::from_str(&serialized)?;
            }

            let updated_at_index_file_path = format!("{}/{}", data_dir, UPDATED_AT_INDEX_FILE);
            if Path::new(&updated_at_index_file_path).exists() {
                let compressed = fs::read(updated_at_index_file_path)?;
                let uncompressed = decompress_size_prepended(&compressed)?;
                let serialized = String::from_utf8(uncompressed)?;
                self.indexes.updated_at = serde_json::from_str(&serialized)?;
            }
        }

        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        if let Some(data_dir) = &self.data_dir {
            let mut stats = self.stats.write().unwrap();

            if stats.can_save(self.saved_writes_threshold, self.saved_writes_trigger_after) {
                let values = self.values.read().unwrap();
                let values_file_path = format!("{}/{}", data_dir, DATABASE_FILE);
                let serialized = serde_json::to_vec(&*values)?;
                let compressed = compress_prepend_size(&serialized);
                fs::write(values_file_path, compressed)?;

                let api_keys = self.api_keys.read().unwrap().to_owned();
                let api_keys_file_path = format!("{}/{}", data_dir, API_KEYS_FILE);
                let serialized = serde_json::to_vec(&*api_keys)?;
                let compressed = compress_prepend_size(&serialized);
                fs::write(api_keys_file_path, compressed)?;

                let created_at_index = self.indexes.created_at.read().unwrap();
                let created_at_index_file_path = format!("{}/{}", data_dir, CREATED_AT_INDEX_FILE);
                let serialized = serde_json::to_vec(&*created_at_index)?;
                let compressed = compress_prepend_size(&serialized);
                fs::write(created_at_index_file_path, compressed)?;

                let key_index = self.indexes.key.read().unwrap();
                let key_index_file_path = format!("{}/{}", data_dir, KEY_INDEX_FILE);
                let serialized = serde_json::to_vec(&*key_index)?;
                let compressed = compress_prepend_size(&serialized);
                fs::write(key_index_file_path, compressed)?;

                let updated_at_index = self.indexes.updated_at.read().unwrap();
                let updated_at_index_file_path = format!("{}/{}", data_dir, UPDATED_AT_INDEX_FILE);
                let serialized = serde_json::to_vec(&*updated_at_index)?;
                let compressed = compress_prepend_size(&serialized);
                fs::write(updated_at_index_file_path, compressed)?;

                stats.update_saved_writes();
            }
        }

        Ok(())
    }

    pub fn select_all(&self, direction: Direction, sort: Sort) -> Result<Vec<ValueResponse>> {
        let mut stats = self.stats.write().unwrap();
        stats.inc_requests();

        let values = self.values.read().unwrap();
        let mut result = vec![];
        let mut ids = vec![];

        match sort {
            Sort::CreatedAt => {
                let created_at_index = self.indexes.created_at.read().unwrap();

                match direction {
                    Direction::Asc => {
                        for (_key, value) in created_at_index.iter() {
                            ids.append(&mut vec![*value]);
                        }
                    }
                    Direction::Desc => {
                        for (_key, value) in created_at_index.iter().rev() {
                            ids.append(&mut vec![*value]);
                        }
                    }
                }
            }
            Sort::Key => {
                let key_index = self.indexes.key.read().unwrap();

                match direction {
                    Direction::Asc => {
                        for (_key, value) in key_index.iter() {
                            ids.append(&mut vec![*value]);
                        }
                    }
                    Direction::Desc => {
                        for (_key, value) in key_index.iter().rev() {
                            ids.append(&mut vec![*value]);
                        }
                    }
                }
            }
            Sort::UpdatedAt => {
                let updated_at_index = self.indexes.updated_at.read().unwrap();

                match direction {
                    Direction::Asc => {
                        for (_key, value) in updated_at_index.iter() {
                            ids.append(&mut vec![*value]);
                        }
                    }
                    Direction::Desc => {
                        for (_key, value) in updated_at_index.iter().rev() {
                            ids.append(&mut vec![*value]);
                        }
                    }
                }
            }
        }

        for id in ids {
            let value = values.get(&id).cloned().unwrap();
            result.append(&mut vec![value.into()]);
            stats.inc_reads();
        }

        Ok(result)
    }

    pub fn try_delete(&self, key: &str) -> Result<Option<ValueResponse>> {
        let mut stats = self.stats.write().unwrap();
        stats.inc_requests();

        let mut key_index = self.indexes.key.write().unwrap();
        let id = *key_index.get(key).unwrap();

        let mut values = self.values.write().unwrap();
        let result = values.remove(&id);

        match result {
            None => Ok(None),
            Some(result) => {
                stats.inc_writes();

                let mut created_at_index = self.indexes.created_at.write().unwrap();
                created_at_index.remove(&result.created_at.timestamp_nanos());

                key_index.remove(&result.key);

                let mut updated_at_index = self.indexes.updated_at.write().unwrap();
                updated_at_index.remove(&result.updated_at.timestamp_nanos());

                Ok(Some(result.into()))
            }
        }
    }

    pub fn try_insert(&self, value_post: ValuePost) -> Result<Option<ValueResponse>> {
        let mut stats = self.stats.write().unwrap();
        stats.inc_requests();

        let mut values = self.values.write().unwrap();
        let id = Uuid::new_v4();
        let value_record = ValueRecord::new(
            id,
            &value_post.key,
            &value_post.value,
            Utc::now(),
            Utc::now(),
        );
        values.insert(id, value_record);
        let result = values.get(&id).cloned();

        match result {
            None => Ok(None),
            Some(result) => {
                stats.inc_writes();

                let mut created_at_index = self.indexes.created_at.write().unwrap();
                created_at_index.insert(result.created_at.timestamp_nanos(), id);

                let mut key_index = self.indexes.key.write().unwrap();
                key_index.insert(value_post.key, id);

                let mut updated_at_index = self.indexes.updated_at.write().unwrap();
                updated_at_index.insert(result.updated_at.timestamp_nanos(), id);

                Ok(Some(result.into()))
            }
        }
    }

    pub fn try_select(&self, key: &str) -> Result<Option<ValueResponse>> {
        let mut stats = self.stats.write().unwrap();
        stats.inc_requests();

        let key_index = self.indexes.key.read().unwrap();
        let id = key_index.get(key);

        match id {
            None => Ok(None),
            Some(id) => {
                let values = self.values.read().unwrap();
                let result = values.get(id).cloned();

                match result {
                    None => Ok(None),
                    Some(result) => {
                        stats.inc_reads();

                        Ok(Some(result.into()))
                    }
                }
            }
        }
    }

    pub fn try_upsert(&self, value_put: ValuePut) -> Result<Option<ValueResponse>> {
        let mut stats = self.stats.write().unwrap();
        stats.inc_requests();

        let mut key_index = self.indexes.key.write().unwrap();
        let id = *key_index.get(&value_put.key).unwrap();

        let mut values = self.values.write().unwrap();
        let original_value = values.get(&id).ok_or(Error::NotFound)?.clone();
        let value_record = ValueRecord::new(
            id,
            &value_put.key,
            &value_put.value,
            original_value.created_at,
            Utc::now(),
        );
        values.insert(id, value_record);
        let result = values.get(&id).cloned();

        match result {
            None => Ok(None),
            Some(result) => {
                stats.inc_writes();

                key_index.remove(&original_value.key);
                key_index.insert(value_put.key, id);

                let mut updated_at_index = self.indexes.updated_at.write().unwrap();
                updated_at_index.remove(&original_value.updated_at.timestamp_nanos());
                updated_at_index.insert(result.updated_at.timestamp_nanos(), id);

                Ok(Some(result.into()))
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Asc,
    Desc,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sort {
    CreatedAt,
    Key,
    UpdatedAt,
}
