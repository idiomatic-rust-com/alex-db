use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ValuePost {
    pub key: String,
    value: String,
}

impl From<ValuePost> for ValueRecord {
    fn from(value_post: ValuePost) -> Self {
        ValueRecord {
            id: Uuid::new_v4(),
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

impl From<ValuePut> for ValueRecord {
    fn from(value_put: ValuePut) -> Self {
        ValueRecord {
            id: Uuid::new_v4(),
            key: value_put.key,
            value: value_put.value,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValueRecord {
    pub id: Uuid,
    key: String,
    value: String,
    pub created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ValueResponse {
    pub key: String,
    pub value: String,
}

impl From<ValueRecord> for ValueResponse {
    fn from(db_record: ValueRecord) -> Self {
        ValueResponse {
            key: db_record.key,
            value: db_record.value,
        }
    }
}
