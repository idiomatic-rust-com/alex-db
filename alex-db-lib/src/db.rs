use crate::{
    config::Config,
    error::Error,
    stat_record::StatRecord,
    user_record::UserRecord,
    value_index::ValueIndex,
    value_record::{
        Value, ValueAppend, ValueDecrement, ValueIncrement, ValuePopBack, ValuePopFront, ValuePost,
        ValuePrepend, ValuePut, ValueRecord, ValueResponse,
    },
    Result,
};
use chrono::{Duration, Utc};
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path, str::FromStr, sync::RwLock};
use uuid::Uuid;

pub const API_KEYS_FILE: &str = "api_keys.sec";
pub const VALUES_CREATED_AT_INDEX_FILE: &str = "values_created_at.idx";
pub const VALUES_DATABASE_FILE: &str = "values.db";
pub const VALUES_DELETE_AT_INDEX_FILE: &str = "values_delete_at.idx";
pub const VALUES_KEY_INDEX_FILE: &str = "values_key.idx";
pub const VALUES_UPDATED_AT_INDEX_FILE: &str = "values_updated_at.idx";

#[derive(Debug, Deserialize, Serialize)]
pub struct Db {
    api_keys: RwLock<Vec<Uuid>>,
    pub config: Config,
    pub stats: RwLock<StatRecord>,
    pub users: RwLock<Vec<UserRecord>>,
    pub values: RwLock<HashMap<Uuid, ValueRecord>>,
    pub values_indexes: ValueIndex,
}

impl Db {
    /// Creates new DB.
    ///
    /// # Examples
    ///
    /// ```
    /// use alex_db_lib::{config::Config, db::Db};
    ///
    /// let config = Config::default();
    /// let db = Db::new(config);
    ///
    /// assert_eq!(0, db.values.read().unwrap().len());
    /// ```
    pub fn new(config: Config) -> Self {
        Self {
            api_keys: RwLock::new(vec![]),
            config,
            stats: RwLock::new(StatRecord::default()),
            users: RwLock::new(vec![]),
            values: RwLock::new(HashMap::new()),
            values_indexes: ValueIndex::default(),
        }
    }

    pub fn api_key_exists(&self, api_key: Uuid) -> Result<bool> {
        let api_keys = self.api_keys.read().map_err(|_| Error::Lock)?;

        let result = api_keys.contains(&api_key);

        Ok(result)
    }

    pub fn api_key_init(&self) -> Result<Option<Uuid>> {
        let mut api_keys = self.api_keys.write().map_err(|_| Error::Lock)?;

        if api_keys.is_empty() {
            let api_key = Uuid::new_v4();
            api_keys.append(&mut vec![api_key]);

            return Ok(Some(api_key));
        }

        Ok(None)
    }

    pub fn gc(&self) -> Result<()> {
        let values_delete_at_index = self
            .values_indexes
            .delete_at
            .read()
            .map_err(|_| Error::Lock)?;
        let now = Utc::now();
        let mut ids = vec![];

        for (key, value) in values_delete_at_index.iter() {
            if now.timestamp_nanos() > *key {
                ids.append(&mut vec![*value]);
            }
        }

        drop(values_delete_at_index);

        for id in ids {
            self.try_delete_by_id(id)?;
        }

        Ok(())
    }

    pub fn get_stats(&self) -> Result<StatRecord> {
        let stats = self.stats.read().map_err(|_| Error::Lock)?.to_owned();

        Ok(stats)
    }

    pub fn restore(&mut self) -> Result<()> {
        if let Some(data_dir) = &self.config.data_dir {
            let api_keys_file_path = format!("{data_dir}/{API_KEYS_FILE}");
            if Path::new(&api_keys_file_path).exists() {
                let compressed = fs::read(api_keys_file_path)?;
                let uncompressed = decompress_size_prepended(&compressed)?;
                let serialized = String::from_utf8(uncompressed)?;
                self.api_keys = serde_json::from_str(&serialized)?;
            }

            let values_created_at_index_file_path =
                format!("{data_dir}/{VALUES_CREATED_AT_INDEX_FILE}");
            if Path::new(&values_created_at_index_file_path).exists() {
                let compressed = fs::read(values_created_at_index_file_path)?;
                let uncompressed = decompress_size_prepended(&compressed)?;
                let serialized = String::from_utf8(uncompressed)?;
                self.values_indexes.created_at = serde_json::from_str(&serialized)?;
            }

            let values_delete_at_index_file_path =
                format!("{data_dir}/{VALUES_DELETE_AT_INDEX_FILE}");
            if Path::new(&values_delete_at_index_file_path).exists() {
                let compressed = fs::read(values_delete_at_index_file_path)?;
                let uncompressed = decompress_size_prepended(&compressed)?;
                let serialized = String::from_utf8(uncompressed)?;
                self.values_indexes.delete_at = serde_json::from_str(&serialized)?;
            }

            let values_key_index_file_path = format!("{data_dir}/{VALUES_KEY_INDEX_FILE}");
            if Path::new(&values_key_index_file_path).exists() {
                let compressed = fs::read(values_key_index_file_path)?;
                let uncompressed = decompress_size_prepended(&compressed)?;
                let serialized = String::from_utf8(uncompressed)?;
                self.values_indexes.key = serde_json::from_str(&serialized)?;
            }

            let values_updated_at_index_file_path =
                format!("{data_dir}/{VALUES_UPDATED_AT_INDEX_FILE}");
            if Path::new(&values_updated_at_index_file_path).exists() {
                let compressed = fs::read(values_updated_at_index_file_path)?;
                let uncompressed = decompress_size_prepended(&compressed)?;
                let serialized = String::from_utf8(uncompressed)?;
                self.values_indexes.updated_at = serde_json::from_str(&serialized)?;
            }

            let values_file_path = format!("{data_dir}/{VALUES_DATABASE_FILE}");
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
        if let Some(data_dir) = &self.config.data_dir {
            let mut stats = self.stats.write().map_err(|_| Error::Lock)?;

            if stats.can_save(
                self.config.save_triggered_after_ms,
                self.config.save_triggered_by_threshold,
            ) {
                let api_keys = self.api_keys.read().map_err(|_| Error::Lock)?.to_owned();
                let api_keys_file_path = format!("{data_dir}/{API_KEYS_FILE}");
                let serialized = serde_json::to_vec(&*api_keys)?;
                let compressed = compress_prepend_size(&serialized);
                fs::write(api_keys_file_path, compressed)?;

                let values_created_at_index = self
                    .values_indexes
                    .created_at
                    .read()
                    .map_err(|_| Error::Lock)?;
                let values_created_at_index_file_path =
                    format!("{data_dir}/{VALUES_CREATED_AT_INDEX_FILE}");
                let serialized = serde_json::to_vec(&*values_created_at_index)?;
                let compressed = compress_prepend_size(&serialized);
                fs::write(values_created_at_index_file_path, compressed)?;

                let values_delete_at_index = self
                    .values_indexes
                    .delete_at
                    .read()
                    .map_err(|_| Error::Lock)?;
                let values_delete_at_index_file_path =
                    format!("{data_dir}/{VALUES_DELETE_AT_INDEX_FILE}");
                let serialized = serde_json::to_vec(&*values_delete_at_index)?;
                let compressed = compress_prepend_size(&serialized);
                fs::write(values_delete_at_index_file_path, compressed)?;

                let values_key_index = self.values_indexes.key.read().map_err(|_| Error::Lock)?;
                let values_key_index_file_path = format!("{data_dir}/{VALUES_KEY_INDEX_FILE}");
                let serialized = serde_json::to_vec(&*values_key_index)?;
                let compressed = compress_prepend_size(&serialized);
                fs::write(values_key_index_file_path, compressed)?;

                let values_updated_at_index = self
                    .values_indexes
                    .updated_at
                    .read()
                    .map_err(|_| Error::Lock)?;
                let values_updated_at_index_file_path =
                    format!("{data_dir}/{VALUES_UPDATED_AT_INDEX_FILE}");
                let serialized = serde_json::to_vec(&*values_updated_at_index)?;
                let compressed = compress_prepend_size(&serialized);
                fs::write(values_updated_at_index_file_path, compressed)?;

                let values = self.values.read().map_err(|_| Error::Lock)?;
                let values_file_path = format!("{data_dir}/{VALUES_DATABASE_FILE}");
                let serialized = serde_json::to_vec(&*values)?;
                let compressed = compress_prepend_size(&serialized);
                fs::write(values_file_path, compressed)?;

                stats.update_saved_writes();
            }
        }

        Ok(())
    }

    /// Returns a list of records from the database.
    ///
    /// # Examples
    ///
    /// ```
    /// use alex_db_lib::{config::Config, db::{Db, Direction, Sort}, value_record::{Value, ValuePost}};
    ///
    /// let config = Config::default();
    /// let mut db = Db::new(config);
    ///
    /// assert_eq!(0, db.stats.read().unwrap().reads);
    ///
    /// let value_responses = db.list(Direction::Asc, None, None, Sort::CreatedAt).unwrap();
    ///
    /// assert_eq!(0, value_responses.len());
    /// assert_eq!(0, db.stats.read().unwrap().reads);
    ///
    /// let key = "test_key".to_string();
    /// let value = Value::Boolean(true);
    /// let value_post = ValuePost { key: key.clone(), ttl: None, value: value.clone() };
    /// db.try_create(value_post);
    /// let value_responses = db.list(Direction::Asc, None, None, Sort::CreatedAt).unwrap();
    ///
    /// assert_eq!(1, value_responses.len());
    /// assert_eq!(1, db.stats.read().unwrap().reads);
    /// ```
    pub fn list(
        &self,
        direction: Direction,
        limit: Option<usize>,
        page: Option<usize>,
        sort: Sort,
    ) -> Result<Vec<ValueResponse>> {
        let mut stats = self.stats.write().map_err(|_| Error::Lock)?;
        stats.inc_requests();

        let values = self.values.read().map_err(|_| Error::Lock)?;
        let mut result = vec![];
        let mut ids = vec![];

        match sort {
            Sort::CreatedAt => {
                let values_created_at_index = self
                    .values_indexes
                    .created_at
                    .read()
                    .map_err(|_| Error::Lock)?;

                match direction {
                    Direction::Asc => {
                        for (_key, value) in values_created_at_index.iter() {
                            ids.append(&mut vec![*value]);
                        }
                    }
                    Direction::Desc => {
                        for (_key, value) in values_created_at_index.iter().rev() {
                            ids.append(&mut vec![*value]);
                        }
                    }
                }
            }
            Sort::DeleteAt => {
                let values_delete_at_index = self
                    .values_indexes
                    .delete_at
                    .read()
                    .map_err(|_| Error::Lock)?;

                match direction {
                    Direction::Asc => {
                        for (_key, value) in values_delete_at_index.iter() {
                            ids.append(&mut vec![*value]);
                        }
                    }
                    Direction::Desc => {
                        for (_key, value) in values_delete_at_index.iter().rev() {
                            ids.append(&mut vec![*value]);
                        }
                    }
                }
            }
            Sort::Key => {
                let values_key_index = self.values_indexes.key.read().map_err(|_| Error::Lock)?;

                match direction {
                    Direction::Asc => {
                        for (_key, value) in values_key_index.iter() {
                            ids.append(&mut vec![*value]);
                        }
                    }
                    Direction::Desc => {
                        for (_key, value) in values_key_index.iter().rev() {
                            ids.append(&mut vec![*value]);
                        }
                    }
                }
            }
            Sort::UpdatedAt => {
                let values_updated_at_index = self
                    .values_indexes
                    .updated_at
                    .read()
                    .map_err(|_| Error::Lock)?;

                match direction {
                    Direction::Asc => {
                        for (_key, value) in values_updated_at_index.iter() {
                            ids.append(&mut vec![*value]);
                        }
                    }
                    Direction::Desc => {
                        for (_key, value) in values_updated_at_index.iter().rev() {
                            ids.append(&mut vec![*value]);
                        }
                    }
                }
            }
        }

        let limit = limit.unwrap_or(100);
        let page = page.unwrap_or(1);

        let skip = (page - 1) * limit;

        ids = ids
            .into_iter()
            .skip(skip)
            .take(limit)
            .collect::<Vec<Uuid>>();

        for id in ids {
            let value = values.get(&id).ok_or(Error::NotFound)?.clone();
            result.append(&mut vec![value.into()]);
            stats.inc_reads();
        }

        Ok(result)
    }

    /// Tries to append a value to an existing record in the database using the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use alex_db_lib::{config::Config, db::Db, value_record::{Value, ValueAppend, ValuePost}};
    /// use std::collections::VecDeque;
    ///
    /// let config = Config::default();
    /// let mut db = Db::new(config);
    ///
    /// assert_eq!(0, db.stats.read().unwrap().writes);
    ///
    /// let key = "test_key".to_string();
    /// let value1 = Value::String("test_value".to_string());
    /// let value1_array = Value::Array(VecDeque::from([value1.clone()]));
    /// let value_post = ValuePost { key: key.clone(), ttl: None, value: value1_array.clone() };
    /// let value_response = db.try_create(value_post).unwrap().unwrap();
    ///
    /// assert_eq!(value_response.key, key);
    /// assert_eq!(value_response.value, value1_array);
    /// assert_eq!(1, db.stats.read().unwrap().writes);
    ///
    /// let value2 = Value::Integer(100);
    /// let value2_array = Value::Array(VecDeque::from([value2.clone()]));
    /// let value_append = ValueAppend { append: value2_array.clone() };
    /// let value_response = db.try_append(&key, value_append.clone()).unwrap().unwrap();
    ///
    /// assert_eq!(value_response.key, key);
    /// assert_eq!(value_response.value, Value::Array(VecDeque::from([value1, value2])));
    /// assert_eq!(2, db.stats.read().unwrap().writes);
    ///
    /// let value_response = db.try_append("wrong_key", value_append);
    ///
    /// assert!(value_response.is_err());
    /// ```
    pub fn try_append(
        &self,
        key: &str,
        value_append: ValueAppend,
    ) -> Result<Option<ValueResponse>> {
        let mut stats = self.stats.write().map_err(|_| Error::Lock)?;
        stats.inc_requests();

        let values_key_index = self.values_indexes.key.write().map_err(|_| Error::Lock)?;
        let id = *values_key_index.get(key).ok_or(Error::NotFound)?;

        let mut values = self.values.write().map_err(|_| Error::Lock)?;
        let original_value = values.get(&id).ok_or(Error::NotFound)?.clone();

        let value = match (original_value.value, value_append.append) {
            (Value::Array(original_value_vec), Value::Array(mut value_append_vec)) => {
                let mut new_value = original_value_vec;
                new_value.append(&mut value_append_vec);

                Value::Array(new_value)
            }
            _ => return Ok(None),
        };

        let now = Utc::now();
        let value_record = ValueRecord::new(
            id,
            &original_value.key,
            &value,
            original_value.created_at,
            original_value.delete_at,
            now,
        );
        values.insert(id, value_record);
        let result = values.get(&id).cloned();

        match result {
            None => Ok(None),
            Some(result) => {
                stats.inc_writes();

                let mut values_updated_at_index = self
                    .values_indexes
                    .updated_at
                    .write()
                    .map_err(|_| Error::Lock)?;
                values_updated_at_index.remove(&original_value.updated_at.timestamp_nanos());
                values_updated_at_index.insert(result.updated_at.timestamp_nanos(), id);

                Ok(Some(result.into()))
            }
        }
    }

    /// Tries to create a new record containing a value in the database.
    ///
    /// # Examples
    ///
    /// ```
    /// use alex_db_lib::{config::Config, db::Db, value_record::{Value, ValuePost}};
    ///
    /// let config = Config::default();
    /// let mut db = Db::new(config);
    ///
    /// assert_eq!(0, db.values_indexes.created_at.read().unwrap().len());
    /// assert_eq!(0, db.values_indexes.delete_at.read().unwrap().len());
    /// assert_eq!(0, db.values_indexes.key.read().unwrap().len());
    /// assert_eq!(0, db.values_indexes.updated_at.read().unwrap().len());
    /// assert_eq!(0, db.stats.read().unwrap().writes);
    /// assert_eq!(0, db.values.read().unwrap().len());
    ///
    /// let key = "test_key1".to_string();
    /// let value = Value::String("test_value".to_string());
    /// let value_post = ValuePost { key: key.clone(), ttl: None, value: value.clone() };
    /// let value_response = db.try_create(value_post).unwrap().unwrap();
    ///
    /// assert_eq!(value_response.key, key);
    /// assert_eq!(value_response.value, value);
    /// assert_eq!(1, db.values_indexes.created_at.read().unwrap().len());
    /// assert_eq!(0, db.values_indexes.delete_at.read().unwrap().len());
    /// assert_eq!(1, db.values_indexes.key.read().unwrap().len());
    /// assert_eq!(1, db.values_indexes.updated_at.read().unwrap().len());
    /// assert_eq!(1, db.stats.read().unwrap().writes);
    /// assert_eq!(1, db.values.read().unwrap().len());
    ///
    /// let key = "test_key2".to_string();
    /// let value = Value::Integer(10);
    /// let value_post = ValuePost { key: key.clone(), ttl: Some(100), value: value.clone() };
    /// let value_response = db.try_create(value_post.clone()).unwrap().unwrap();
    ///
    /// assert_eq!(value_response.key, key);
    /// assert_eq!(value_response.value, value);
    /// assert_eq!(2, db.values_indexes.created_at.read().unwrap().len());
    /// assert_eq!(1, db.values_indexes.delete_at.read().unwrap().len());
    /// assert_eq!(2, db.values_indexes.key.read().unwrap().len());
    /// assert_eq!(2, db.values_indexes.updated_at.read().unwrap().len());
    /// assert_eq!(2, db.stats.read().unwrap().writes);
    /// assert_eq!(2, db.values.read().unwrap().len());
    ///
    /// let value_response = db.try_create(value_post);
    ///
    /// assert!(value_response.is_err());
    /// ```
    pub fn try_create(&self, value_post: ValuePost) -> Result<Option<ValueResponse>> {
        let mut stats = self.stats.write().map_err(|_| Error::Lock)?;
        stats.inc_requests();

        let mut values_key_index = self.values_indexes.key.write().map_err(|_| Error::Lock)?;
        let existing_id = values_key_index.get(&value_post.key);

        if existing_id.is_some() {
            return Err(Box::new(Error::KeyExists));
        }

        let mut values = self.values.write().map_err(|_| Error::Lock)?;
        let id = Uuid::new_v4();
        let now = Utc::now();
        let delete_at = value_post.ttl.map(|ttl| now + Duration::seconds(ttl));
        let value_record =
            ValueRecord::new(id, &value_post.key, &value_post.value, now, delete_at, now);
        values.insert(id, value_record);
        let result = values.get(&id).cloned();

        match result {
            None => Ok(None),
            Some(result) => {
                stats.inc_writes();

                let mut values_created_at_index = self
                    .values_indexes
                    .created_at
                    .write()
                    .map_err(|_| Error::Lock)?;
                values_created_at_index.insert(result.created_at.timestamp_nanos(), id);

                if let Some(delete_at) = delete_at {
                    let mut values_delete_at_index = self
                        .values_indexes
                        .delete_at
                        .write()
                        .map_err(|_| Error::Lock)?;
                    values_delete_at_index.insert(delete_at.timestamp_nanos(), id);
                }

                values_key_index.insert(value_post.key, id);

                let mut values_updated_at_index = self
                    .values_indexes
                    .updated_at
                    .write()
                    .map_err(|_| Error::Lock)?;
                values_updated_at_index.insert(result.updated_at.timestamp_nanos(), id);

                Ok(Some(result.into()))
            }
        }
    }

    /// Tries to decrement a value of an existing record in the database using the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use alex_db_lib::{config::Config, db::Db, value_record::{Value, ValueDecrement, ValuePost}};
    /// use std::collections::VecDeque;
    ///
    /// let config = Config::default();
    /// let mut db = Db::new(config);
    ///
    /// assert_eq!(0, db.stats.read().unwrap().writes);
    ///
    /// let key = "test_key".to_string();
    /// let value = Value::Integer(5000);
    /// let value_post = ValuePost { key: key.clone(), ttl: None, value: value.clone() };
    /// let value_response = db.try_create(value_post).unwrap().unwrap();
    ///
    /// assert_eq!(value_response.key, key);
    /// assert_eq!(value_response.value, value);
    /// assert_eq!(1, db.stats.read().unwrap().writes);
    ///
    /// let value_decrement = ValueDecrement { decrement: None };
    /// let value_response = db.try_decrement(&key, value_decrement).unwrap().unwrap();
    ///
    /// assert_eq!(value_response.key, key);
    /// assert_eq!(value_response.value, Value::Integer(4999));
    /// assert_eq!(2, db.stats.read().unwrap().writes);
    ///
    /// let value_decrement = ValueDecrement { decrement: Some(10) };
    /// let value_response = db.try_decrement(&key, value_decrement.clone()).unwrap().unwrap();
    ///
    /// assert_eq!(value_response.key, key);
    /// assert_eq!(value_response.value, Value::Integer(4989));
    /// assert_eq!(3, db.stats.read().unwrap().writes);
    ///
    /// let value_response = db.try_decrement("wrong_key", value_decrement);
    ///
    /// assert!(value_response.is_err());
    /// ```
    pub fn try_decrement(
        &self,
        key: &str,
        value_decrement: ValueDecrement,
    ) -> Result<Option<ValueResponse>> {
        let mut stats = self.stats.write().map_err(|_| Error::Lock)?;
        stats.inc_requests();

        let values_key_index = self.values_indexes.key.write().map_err(|_| Error::Lock)?;
        let id = *values_key_index.get(key).ok_or(Error::NotFound)?;

        let mut values = self.values.write().map_err(|_| Error::Lock)?;
        let original_value = values.get(&id).ok_or(Error::NotFound)?.clone();

        let value = match original_value.value {
            Value::Integer(original_value_integer) => match value_decrement.decrement {
                None => Value::Integer(original_value_integer.saturating_sub(1)),
                Some(decrement) => {
                    if let Some(abs) = decrement.checked_abs() {
                        Value::Integer(original_value_integer.saturating_sub(abs))
                    } else {
                        Value::Integer(original_value_integer)
                    }
                }
            },
            _ => return Ok(None),
        };

        let now = Utc::now();
        let value_record = ValueRecord::new(
            id,
            &original_value.key,
            &value,
            original_value.created_at,
            original_value.delete_at,
            now,
        );
        values.insert(id, value_record);
        let result = values.get(&id).cloned();

        match result {
            None => Ok(None),
            Some(result) => {
                stats.inc_writes();

                let mut values_updated_at_index = self
                    .values_indexes
                    .updated_at
                    .write()
                    .map_err(|_| Error::Lock)?;
                values_updated_at_index.remove(&original_value.updated_at.timestamp_nanos());
                values_updated_at_index.insert(result.updated_at.timestamp_nanos(), id);

                Ok(Some(result.into()))
            }
        }
    }

    /// Tries to delete an existing record from the database using the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use alex_db_lib::{config::Config, db::Db, value_record::{Value, ValuePost}};
    /// use std::collections::VecDeque;
    ///
    /// let config = Config::default();
    /// let mut db = Db::new(config);
    ///
    /// assert_eq!(0, db.stats.read().unwrap().writes);
    ///
    /// let key = "test_key".to_string();
    /// let value = Value::Boolean(false);
    /// let value_post = ValuePost { key: key.clone(), ttl: None, value: value.clone() };
    /// let value_response = db.try_create(value_post).unwrap().unwrap();
    ///
    /// assert_eq!(value_response.key, key);
    /// assert_eq!(value_response.value, value);
    /// assert_eq!(1, db.stats.read().unwrap().writes);
    ///
    /// let value_response = db.try_delete(&key).unwrap().unwrap();
    ///
    /// assert_eq!(value_response.key, key);
    /// assert_eq!(value_response.value, value);
    /// assert_eq!(2, db.stats.read().unwrap().writes);
    ///
    /// let value_response = db.try_read(&key).unwrap();
    ///
    /// assert!(value_response.is_none());
    ///
    /// let value_response = db.try_delete("wrong_key");
    ///
    /// assert!(value_response.is_err());
    /// ```
    pub fn try_delete(&self, key: &str) -> Result<Option<ValueResponse>> {
        let values_key_index = self.values_indexes.key.read().map_err(|_| Error::Lock)?;
        let id = *values_key_index.get(key).ok_or(Error::NotFound)?;
        drop(values_key_index);

        self.try_delete_by_id(id)
    }

    fn try_delete_by_id(&self, id: Uuid) -> Result<Option<ValueResponse>> {
        let mut stats = self.stats.write().map_err(|_| Error::Lock)?;
        stats.inc_requests();

        let mut values = self.values.write().map_err(|_| Error::Lock)?;
        let result = values.remove(&id);

        match result {
            None => Ok(None),
            Some(result) => {
                stats.inc_writes();

                let mut values_created_at_index = self
                    .values_indexes
                    .created_at
                    .write()
                    .map_err(|_| Error::Lock)?;
                values_created_at_index.remove(&result.created_at.timestamp_nanos());

                if let Some(delete_at) = result.delete_at {
                    let mut values_delete_at_index = self
                        .values_indexes
                        .delete_at
                        .write()
                        .map_err(|_| Error::Lock)?;
                    values_delete_at_index.remove(&delete_at.timestamp_nanos());
                }

                let mut values_key_index =
                    self.values_indexes.key.write().map_err(|_| Error::Lock)?;
                values_key_index.remove(&result.key);

                let mut values_updated_at_index = self
                    .values_indexes
                    .updated_at
                    .write()
                    .map_err(|_| Error::Lock)?;
                values_updated_at_index.remove(&result.updated_at.timestamp_nanos());

                Ok(Some(result.into()))
            }
        }
    }

    /// Tries to increment a value of an existing record in the database using the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use alex_db_lib::{config::Config, db::Db, value_record::{Value, ValuePost, ValueIncrement}};
    /// use std::collections::VecDeque;
    ///
    /// let config = Config::default();
    /// let mut db = Db::new(config);
    ///
    /// assert_eq!(0, db.stats.read().unwrap().writes);
    ///
    /// let key = "test_key".to_string();
    /// let value = Value::Integer(1000);
    /// let value_post = ValuePost { key: key.clone(), ttl: None, value: value.clone() };
    /// let value_response = db.try_create(value_post).unwrap().unwrap();
    ///
    /// assert_eq!(value_response.key, key);
    /// assert_eq!(value_response.value, value);
    /// assert_eq!(1, db.stats.read().unwrap().writes);
    ///
    /// let value_increment = ValueIncrement { increment: None };
    /// let value_response = db.try_increment(&key, value_increment).unwrap().unwrap();
    ///
    /// assert_eq!(value_response.key, key);
    /// assert_eq!(value_response.value, Value::Integer(1001));
    /// assert_eq!(2, db.stats.read().unwrap().writes);
    ///
    /// let value_increment = ValueIncrement { increment: Some(10) };
    /// let value_response = db.try_increment(&key, value_increment.clone()).unwrap().unwrap();
    ///
    /// assert_eq!(value_response.key, key);
    /// assert_eq!(value_response.value, Value::Integer(1011));
    /// assert_eq!(3, db.stats.read().unwrap().writes);
    ///
    /// let value_response = db.try_increment("wrong_key", value_increment);
    ///
    /// assert!(value_response.is_err());
    /// ```
    pub fn try_increment(
        &self,
        key: &str,
        value_increment: ValueIncrement,
    ) -> Result<Option<ValueResponse>> {
        let mut stats = self.stats.write().map_err(|_| Error::Lock)?;
        stats.inc_requests();

        let values_key_index = self.values_indexes.key.write().map_err(|_| Error::Lock)?;
        let id = *values_key_index.get(key).ok_or(Error::NotFound)?;

        let mut values = self.values.write().map_err(|_| Error::Lock)?;
        let original_value = values.get(&id).ok_or(Error::NotFound)?.clone();

        let value = match original_value.value {
            Value::Integer(original_value_integer) => match value_increment.increment {
                None => Value::Integer(original_value_integer.saturating_add(1)),
                Some(increment) => {
                    if let Some(abs) = increment.checked_abs() {
                        Value::Integer(original_value_integer.saturating_add(abs))
                    } else {
                        Value::Integer(original_value_integer)
                    }
                }
            },
            _ => return Ok(None),
        };

        let now = Utc::now();
        let value_record = ValueRecord::new(
            id,
            &original_value.key,
            &value,
            original_value.created_at,
            original_value.delete_at,
            now,
        );
        values.insert(id, value_record);
        let result = values.get(&id).cloned();

        match result {
            None => Ok(None),
            Some(result) => {
                stats.inc_writes();

                let mut values_updated_at_index = self
                    .values_indexes
                    .updated_at
                    .write()
                    .map_err(|_| Error::Lock)?;
                values_updated_at_index.remove(&original_value.updated_at.timestamp_nanos());
                values_updated_at_index.insert(result.updated_at.timestamp_nanos(), id);

                Ok(Some(result.into()))
            }
        }
    }

    /// Tries to pop a value from the back of an existing record in the database using the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use alex_db_lib::{config::Config, db::Db, value_record::{Value, ValuePopBack, ValuePost}};
    /// use std::collections::VecDeque;
    ///
    /// let config = Config::default();
    /// let mut db = Db::new(config);
    ///
    /// assert_eq!(0, db.stats.read().unwrap().writes);
    ///
    /// let key = "test_key".to_string();
    /// let value1 = Value::String("test_value1".to_string());
    /// let value2 = Value::String("test_value2".to_string());
    /// let value3 = Value::Integer(100);
    /// let value4 = Value::Integer(1000);
    /// let value_array = Value::Array(VecDeque::from([value1.clone(), value2.clone(), value3.clone(), value4.clone()]));
    /// let value_post = ValuePost { key: key.clone(), ttl: None, value: value_array.clone() };
    /// let value_response = db.try_create(value_post).unwrap().unwrap();
    ///
    /// assert_eq!(value_response.key, key);
    /// assert_eq!(value_response.value, value_array);
    /// assert_eq!(1, db.stats.read().unwrap().writes);
    ///
    /// let value_pop_back = ValuePopBack { pop_back: None };
    /// let value_response = db.try_pop_back(&key, value_pop_back).unwrap().unwrap();
    ///
    /// assert_eq!(value_response, vec![value4]);
    /// assert_eq!(2, db.stats.read().unwrap().writes);
    ///
    /// let value_pop_back = ValuePopBack { pop_back: Some(2) };
    /// let value_response = db.try_pop_back(&key, value_pop_back.clone()).unwrap().unwrap();
    ///
    /// assert_eq!(value_response, vec![value3, value2]);
    /// assert_eq!(3, db.stats.read().unwrap().writes);
    ///
    /// let value_response = db.try_pop_back("wrong_key", value_pop_back);
    ///
    /// assert!(value_response.is_err());
    /// ```
    pub fn try_pop_back(
        &self,
        key: &str,
        value_pop_back: ValuePopBack,
    ) -> Result<Option<Vec<Value>>> {
        let mut stats = self.stats.write().map_err(|_| Error::Lock)?;
        stats.inc_requests();

        let values_key_index = self.values_indexes.key.write().map_err(|_| Error::Lock)?;
        let id = *values_key_index.get(key).ok_or(Error::NotFound)?;

        let mut values = self.values.write().map_err(|_| Error::Lock)?;
        let original_value = values.get(&id).ok_or(Error::NotFound)?.clone();

        let mut return_values = vec![];
        let value = match original_value.value {
            Value::Array(original_value_vec) => match value_pop_back.pop_back {
                None => {
                    let mut new_value = original_value_vec;
                    let pop_value = new_value.pop_back();
                    if let Some(pop_value) = pop_value {
                        return_values.append(&mut vec![pop_value]);
                    }

                    Value::Array(new_value)
                }
                Some(mut pop_back) => {
                    if pop_back > original_value_vec.len() {
                        pop_back = original_value_vec.len();
                    }

                    let mut new_value = original_value_vec;

                    for _i in 1..=pop_back {
                        let pop_value = new_value.pop_back();
                        if let Some(pop_value) = pop_value {
                            return_values.append(&mut vec![pop_value]);
                        }
                    }

                    Value::Array(new_value)
                }
            },
            _ => return Ok(None),
        };

        let now = Utc::now();
        let value_record = ValueRecord::new(
            id,
            &original_value.key,
            &value,
            original_value.created_at,
            original_value.delete_at,
            now,
        );
        values.insert(id, value_record);
        let result = values.get(&id).cloned();

        match result {
            None => Ok(None),
            Some(result) => {
                stats.inc_writes();

                let mut values_updated_at_index = self
                    .values_indexes
                    .updated_at
                    .write()
                    .map_err(|_| Error::Lock)?;
                values_updated_at_index.remove(&original_value.updated_at.timestamp_nanos());
                values_updated_at_index.insert(result.updated_at.timestamp_nanos(), id);

                Ok(Some(return_values))
            }
        }
    }

    /// Tries to pop a value from the front of an existing record in the database using the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use alex_db_lib::{config::Config, db::Db, value_record::{Value, ValuePopFront, ValuePost}};
    /// use std::collections::VecDeque;
    ///
    /// let config = Config::default();
    /// let mut db = Db::new(config);
    ///
    /// assert_eq!(0, db.stats.read().unwrap().writes);
    ///
    /// let key = "test_key".to_string();
    /// let value1 = Value::String("test_value1".to_string());
    /// let value2 = Value::String("test_value2".to_string());
    /// let value3 = Value::Integer(100);
    /// let value4 = Value::Integer(1000);
    /// let value_array = Value::Array(VecDeque::from([value1.clone(), value2.clone(), value3.clone(), value4.clone()]));
    /// let value_post = ValuePost { key: key.clone(), ttl: None, value: value_array.clone() };
    /// let value_response = db.try_create(value_post).unwrap().unwrap();
    ///
    /// assert_eq!(value_response.key, key);
    /// assert_eq!(value_response.value, value_array);
    /// assert_eq!(1, db.stats.read().unwrap().writes);
    ///
    /// let value_pop_front = ValuePopFront { pop_front: None };
    /// let value_response = db.try_pop_front(&key, value_pop_front).unwrap().unwrap();
    ///
    /// assert_eq!(value_response, vec![value1]);
    /// assert_eq!(2, db.stats.read().unwrap().writes);
    ///
    /// let value_pop_front = ValuePopFront { pop_front: Some(2) };
    /// let value_response = db.try_pop_front(&key, value_pop_front.clone()).unwrap().unwrap();
    ///
    /// assert_eq!(value_response, vec![value2, value3]);
    /// assert_eq!(3, db.stats.read().unwrap().writes);
    ///
    /// let value_response = db.try_pop_front("wrong_key", value_pop_front);
    ///
    /// assert!(value_response.is_err());
    /// ```
    pub fn try_pop_front(
        &self,
        key: &str,
        value_pop_front: ValuePopFront,
    ) -> Result<Option<Vec<Value>>> {
        let mut stats = self.stats.write().map_err(|_| Error::Lock)?;
        stats.inc_requests();

        let values_key_index = self.values_indexes.key.write().map_err(|_| Error::Lock)?;
        let id = *values_key_index.get(key).ok_or(Error::NotFound)?;

        let mut values = self.values.write().map_err(|_| Error::Lock)?;
        let original_value = values.get(&id).ok_or(Error::NotFound)?.clone();

        let mut return_values = vec![];
        let value = match original_value.value {
            Value::Array(original_value_vec) => match value_pop_front.pop_front {
                None => {
                    let mut new_value = original_value_vec;
                    let pop_value = new_value.pop_front();
                    if let Some(pop_value) = pop_value {
                        return_values.append(&mut vec![pop_value]);
                    }

                    Value::Array(new_value)
                }
                Some(mut pop_front) => {
                    if pop_front > original_value_vec.len() {
                        pop_front = original_value_vec.len();
                    }
                    let mut new_value = original_value_vec;

                    for _i in 1..=pop_front {
                        let pop_value = new_value.pop_front();
                        if let Some(pop_value) = pop_value {
                            return_values.append(&mut vec![pop_value]);
                        }
                    }

                    Value::Array(new_value)
                }
            },
            _ => return Ok(None),
        };

        let now = Utc::now();
        let value_record = ValueRecord::new(
            id,
            &original_value.key,
            &value,
            original_value.created_at,
            original_value.delete_at,
            now,
        );
        values.insert(id, value_record);
        let result = values.get(&id).cloned();

        match result {
            None => Ok(None),
            Some(result) => {
                stats.inc_writes();

                let mut values_updated_at_index = self
                    .values_indexes
                    .updated_at
                    .write()
                    .map_err(|_| Error::Lock)?;
                values_updated_at_index.remove(&original_value.updated_at.timestamp_nanos());
                values_updated_at_index.insert(result.updated_at.timestamp_nanos(), id);

                Ok(Some(return_values))
            }
        }
    }

    /// Tries to prepend a value to an existing record in the database using the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use alex_db_lib::{config::Config, db::Db, value_record::{Value, ValuePost, ValuePrepend}};
    /// use std::collections::VecDeque;
    ///
    /// let config = Config::default();
    /// let mut db = Db::new(config);
    ///
    /// assert_eq!(0, db.stats.read().unwrap().writes);
    ///
    /// let key = "test_key".to_string();
    /// let value1 = Value::String("test_value".to_string());
    /// let value1_array = Value::Array(VecDeque::from([value1.clone()]));
    /// let value_post = ValuePost { key: key.clone(), ttl: None, value: value1_array.clone() };
    /// let value_response = db.try_create(value_post).unwrap().unwrap();
    ///
    /// assert_eq!(value_response.key, key);
    /// assert_eq!(value_response.value, value1_array);
    /// assert_eq!(1, db.stats.read().unwrap().writes);
    ///
    /// let value2 = Value::Integer(100);
    /// let value2_array = Value::Array(VecDeque::from([value2.clone()]));
    /// let value_prepend = ValuePrepend { prepend: value2_array.clone() };
    /// let value_response = db.try_prepend(&key, value_prepend.clone()).unwrap().unwrap();
    ///
    /// assert_eq!(value_response.key, key);
    /// assert_eq!(value_response.value, Value::Array(VecDeque::from([value2, value1])));
    /// assert_eq!(2, db.stats.read().unwrap().writes);
    ///
    /// let value_response = db.try_prepend("wrong_key", value_prepend);
    ///
    /// assert!(value_response.is_err());
    /// ```
    pub fn try_prepend(
        &self,
        key: &str,
        value_prepend: ValuePrepend,
    ) -> Result<Option<ValueResponse>> {
        let mut stats = self.stats.write().map_err(|_| Error::Lock)?;
        stats.inc_requests();

        let values_key_index = self.values_indexes.key.write().map_err(|_| Error::Lock)?;
        let id = *values_key_index.get(key).ok_or(Error::NotFound)?;

        let mut values = self.values.write().map_err(|_| Error::Lock)?;
        let original_value = values.get(&id).ok_or(Error::NotFound)?.clone();

        let value = match (original_value.value, value_prepend.prepend) {
            (Value::Array(original_value_vec), Value::Array(value_prepend_vec)) => {
                let mut new_value = original_value_vec;

                for value_prepend_item in value_prepend_vec {
                    new_value.push_front(value_prepend_item);
                }

                Value::Array(new_value)
            }
            _ => return Ok(None),
        };

        let now = Utc::now();
        let value_record = ValueRecord::new(
            id,
            &original_value.key,
            &value,
            original_value.created_at,
            original_value.delete_at,
            now,
        );
        values.insert(id, value_record);
        let result = values.get(&id).cloned();

        match result {
            None => Ok(None),
            Some(result) => {
                stats.inc_writes();

                let mut values_updated_at_index = self
                    .values_indexes
                    .updated_at
                    .write()
                    .map_err(|_| Error::Lock)?;
                values_updated_at_index.remove(&original_value.updated_at.timestamp_nanos());
                values_updated_at_index.insert(result.updated_at.timestamp_nanos(), id);

                Ok(Some(result.into()))
            }
        }
    }

    /// Tries to read a record from the database using the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use alex_db_lib::{config::Config, db::Db, value_record::{Value, ValuePost}};
    ///
    /// let config = Config::default();
    /// let mut db = Db::new(config);
    ///
    /// assert_eq!(0, db.stats.read().unwrap().reads);
    ///
    /// let key = "test_key".to_string();
    /// let value = Value::Integer(10);
    /// let value_post = ValuePost { key: key.clone(), ttl: None, value: value.clone() };
    /// db.try_create(value_post);
    /// let value_response = db.try_read(&key).unwrap().unwrap();
    ///
    /// assert_eq!(value_response.key, key);
    /// assert_eq!(value_response.value, value);
    /// assert_eq!(1, db.stats.read().unwrap().reads);
    /// ```
    pub fn try_read(&self, key: &str) -> Result<Option<ValueResponse>> {
        let mut stats = self.stats.write().map_err(|_| Error::Lock)?;
        stats.inc_requests();

        let values_key_index = self.values_indexes.key.read().map_err(|_| Error::Lock)?;
        let id = values_key_index.get(key);

        match id {
            None => Ok(None),
            Some(id) => {
                let values = self.values.read().map_err(|_| Error::Lock)?;
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

    /// Tries to update a record in the database using the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use alex_db_lib::{config::Config, db::Db, value_record::{Value, ValuePost, ValuePut}};
    ///
    /// let config = Config::default();
    /// let mut db = Db::new(config);
    ///
    /// assert_eq!(0, db.stats.read().unwrap().writes);
    ///
    /// let key = "test_key".to_string();
    /// let value = Value::String("test_value".to_string());
    /// let value_post = ValuePost { key: key.clone(), ttl: None, value: value.clone() };
    /// let value_response = db.try_create(value_post).unwrap().unwrap();
    ///
    /// assert_eq!(value_response.key, key);
    /// assert_eq!(value_response.value, value);
    /// assert_eq!(1, db.stats.read().unwrap().writes);
    ///
    /// let value = Value::Integer(100);
    /// let value_put = ValuePut { ttl: None, value: value.clone() };
    /// let value_response = db.try_update(&key, value_put.clone()).unwrap().unwrap();
    ///
    /// assert_eq!(value_response.key, key);
    /// assert_eq!(value_response.value, value);
    /// assert_eq!(2, db.stats.read().unwrap().writes);
    ///
    /// let value_response = db.try_update("wrong_key", value_put);
    ///
    /// assert!(value_response.is_err());
    /// ```
    pub fn try_update(&self, key: &str, value_put: ValuePut) -> Result<Option<ValueResponse>> {
        let mut stats = self.stats.write().map_err(|_| Error::Lock)?;
        stats.inc_requests();

        let values_key_index = self.values_indexes.key.read().map_err(|_| Error::Lock)?;
        let id = *values_key_index.get(key).ok_or(Error::NotFound)?;

        let mut values = self.values.write().map_err(|_| Error::Lock)?;
        let original_value = values.get(&id).ok_or(Error::NotFound)?.clone();

        let now = Utc::now();
        let delete_at = value_put.ttl.map(|ttl| now + Duration::seconds(ttl));
        let value_record = ValueRecord::new(
            id,
            &original_value.key,
            &value_put.value,
            original_value.created_at,
            delete_at,
            now,
        );
        values.insert(id, value_record);
        let result = values.get(&id).cloned();

        match result {
            None => Ok(None),
            Some(result) => {
                stats.inc_writes();

                let mut values_delete_at_index = self
                    .values_indexes
                    .delete_at
                    .write()
                    .map_err(|_| Error::Lock)?;
                if let Some(original_value_delete_at) = original_value.delete_at {
                    values_delete_at_index.remove(&original_value_delete_at.timestamp_nanos());
                }
                if let Some(delete_at) = delete_at {
                    values_delete_at_index.insert(delete_at.timestamp_nanos(), id);
                }

                let mut values_updated_at_index = self
                    .values_indexes
                    .updated_at
                    .write()
                    .map_err(|_| Error::Lock)?;
                values_updated_at_index.remove(&original_value.updated_at.timestamp_nanos());
                values_updated_at_index.insert(result.updated_at.timestamp_nanos(), id);

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

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "asc" => Ok(Direction::Asc),
            "desc" => Ok(Direction::Desc),
            _ => Ok(Direction::Asc),
        }
    }
}

impl From<Direction> for String {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Asc => "asc".to_string(),
            Direction::Desc => "desc".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sort {
    CreatedAt,
    DeleteAt,
    Key,
    UpdatedAt,
}

impl FromStr for Sort {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "created_at" => Ok(Sort::CreatedAt),
            "delete_at" => Ok(Sort::DeleteAt),
            "key" => Ok(Sort::Key),
            "updated_at" => Ok(Sort::UpdatedAt),
            _ => Ok(Sort::Key),
        }
    }
}

impl From<Sort> for String {
    fn from(sort: Sort) -> Self {
        match sort {
            Sort::CreatedAt => "created_at".to_string(),
            Sort::DeleteAt => "delete_at".to_string(),
            Sort::Key => "key".to_string(),
            Sort::UpdatedAt => "updated_at".to_string(),
        }
    }
}
