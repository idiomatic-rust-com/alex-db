use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

lazy_static! {
    static ref VALID_KEY: Regex = Regex::new(r"^[a-zA-Z0-9._~!$&'()*+,;=:@/?-]+$").unwrap();
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(untagged)]
pub enum ArrayValue {
    Boolean(bool),
    Integer(i64),
    String(String),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(untagged)]
pub enum Value {
    Array(Vec<ArrayValue>),
    Boolean(bool),
    Integer(i64),
    String(String),
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct ValuePost {
    #[validate(regex = "VALID_KEY")]
    pub key: String,
    pub ttl: Option<i64>,
    pub value: Value,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct ValuePut {
    #[validate(regex = "VALID_KEY")]
    pub key: String,
    pub ttl: Option<i64>,
    pub value: Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValueRecord {
    pub id: Uuid,
    pub key: String,
    value: Value,
    pub created_at: DateTime<Utc>,
    pub delete_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

impl ValueRecord {
    pub fn new(
        id: Uuid,
        key: &str,
        value: &Value,
        created_at: DateTime<Utc>,
        delete_at: Option<DateTime<Utc>>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            key: key.into(),
            value: value.clone(),
            created_at,
            delete_at,
            updated_at,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ValueResponse {
    pub key: String,
    pub value: Value,
}

impl From<ValueRecord> for ValueResponse {
    fn from(db_record: ValueRecord) -> Self {
        ValueResponse {
            key: db_record.key,
            value: db_record.value,
        }
    }
}
