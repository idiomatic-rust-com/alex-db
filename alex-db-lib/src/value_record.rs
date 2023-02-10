use crate::error::Error;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, str::FromStr};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

lazy_static! {
    static ref VALID_KEY: Regex = Regex::new(r"^[a-zA-Z0-9._~!$&'()*+,;=:@/?-]+$").unwrap();
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
#[serde(untagged)]
pub enum Value {
    Array(VecDeque<Value>),
    Boolean(bool),
    Integer(i64),
    String(String),
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut destination_value = match s.parse::<bool>() {
            Err(_) => None,
            Ok(value) => Some(Value::Boolean(value)),
        };

        if destination_value.is_none() {
            destination_value = match s.parse::<i64>() {
                Err(_) => None,
                Ok(value) => Some(Value::Integer(value)),
            };
        }

        if destination_value.is_none() {
            let splitted_arguments = s.split("::").into_iter().collect::<Vec<&str>>();
            if splitted_arguments.len() > 1 {
                let mut splitted_argument_values = VecDeque::new();
                for splitted_argument in splitted_arguments {
                    let splitted_argument_value = Self::from_str(splitted_argument)?;
                    splitted_argument_values.push_back(splitted_argument_value);
                }
                destination_value = Some(Value::Array(splitted_argument_values));
            }
        }

        if destination_value.is_none() {
            destination_value = Some(Value::String(s.to_string()));
        }

        let value = destination_value.ok_or(Error::ValueParse)?;

        Ok(value)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct ValueAppend {
    pub append: Value,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct ValueDecrement {
    pub decrement: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct ValueIncrement {
    pub increment: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct ValuePost {
    #[validate(regex = "VALID_KEY")]
    pub key: String,
    pub ttl: Option<i64>,
    pub value: Value,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct ValuePopBack {
    pub pop_back: Option<usize>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct ValuePopFront {
    pub pop_front: Option<usize>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct ValuePrepend {
    pub prepend: Value,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct ValuePut {
    pub ttl: Option<i64>,
    pub value: Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValueRecord {
    pub id: Uuid,
    pub key: String,
    pub value: Value,
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
