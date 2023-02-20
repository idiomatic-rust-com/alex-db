#![forbid(unsafe_code)]

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

pub mod config;
pub mod db;
pub mod error;
pub mod stat_record;
pub mod user_record;
pub mod value_index;
pub mod value_record;
