use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DbRecord {
    key: String,
    value: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ValuePost {
    pub key: String,
    value: String,
}

impl From<ValuePost> for DbRecord {
    fn from(value_post: ValuePost) -> Self {
        DbRecord {
            key: value_post.key,
            value: value_post.value,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ValuePut {
    pub key: String,
    value: String,
}

impl From<ValuePut> for DbRecord {
    fn from(value_put: ValuePut) -> Self {
        DbRecord {
            key: value_put.key,
            value: value_put.value,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ValueResponse {
    pub key: String,
    pub value: String,
}

impl From<DbRecord> for ValueResponse {
    fn from(db_record: DbRecord) -> Self {
        ValueResponse {
            key: db_record.key,
            value: db_record.value,
        }
    }
}
