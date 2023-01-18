use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

pub mod db;
pub mod error;
pub mod stat_record;
pub mod value_record;
