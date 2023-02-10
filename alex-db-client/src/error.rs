use serde::Deserialize;
use std::fmt;

#[derive(Debug)]
pub enum ClientError<'a> {
    Database(alex_db_lib::error::Error),
    Json(serde_json::Error),
    NoActiveConnection,
    Repl(reedline_repl_rs::Error),
    Request(reqwest::Error),
    String(&'a str),
}

impl fmt::Display for ClientError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError::Database(e) => write!(f, "Database error: {e}."),
            ClientError::Json(e) => write!(f, "JSON error: {e}."),
            ClientError::NoActiveConnection => write!(f, "No active connection."),
            ClientError::Repl(e) => write!(f, "REPL error: {e}."),
            ClientError::Request(e) => write!(f, "Request error: {e}."),
            ClientError::String(s) => write!(f, "{s}."),
        }
    }
}

impl From<serde_json::Error> for ClientError<'_> {
    fn from(e: serde_json::Error) -> Self {
        ClientError::Json(e)
    }
}

impl From<alex_db_lib::error::Error> for ClientError<'_> {
    fn from(e: alex_db_lib::error::Error) -> Self {
        ClientError::Database(e)
    }
}

impl From<reedline_repl_rs::Error> for ClientError<'_> {
    fn from(e: reedline_repl_rs::Error) -> Self {
        ClientError::Repl(e)
    }
}

impl From<reqwest::Error> for ClientError<'_> {
    fn from(e: reqwest::Error) -> Self {
        ClientError::Request(e)
    }
}

impl std::error::Error for ClientError<'_> {}

#[derive(Debug, Deserialize)]
pub struct ServerError {
    pub error: String,
}
