use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DbRecord {
    pub key: String,
    value: String,
}
