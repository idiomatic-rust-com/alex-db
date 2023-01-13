use crate::{Args, Result};
use tracing::info;

#[derive(Clone, Debug)]
pub struct Config {
    pub data_dir: Option<String>,
    pub port: u16,
    pub saved_writes_sleep: u64,
    pub saved_writes_threshold: u16,
    pub security_api_keys: bool,
}

impl Config {
    pub fn new(
        data_dir: Option<String>,
        port: u16,
        saved_writes_sleep: u64,
        saved_writes_threshold: u16,
        security_api_keys: bool,
    ) -> Self {
        Self {
            data_dir,
            port,
            saved_writes_sleep,
            saved_writes_threshold,
            security_api_keys,
        }
    }
}

pub fn load(args: Args) -> Result<Config> {
    let mut data_dir = None;
    let mut port = 8080;
    let mut saved_writes_sleep = 10000;
    let mut saved_writes_threshold = 8;
    let mut security_api_keys = true;

    match std::env::var("ALEX_DB_DATA_DIR") {
        Err(_) => {}
        Ok(val) => data_dir = Some(val),
    }

    match args.data_dir {
        None => {}
        Some(val) => data_dir = Some(val),
    }

    match std::env::var("ALEX_DB_PORT") {
        Err(_) => {}
        Ok(val) => {
            port = val.parse::<u16>()?;
        }
    }

    match args.port {
        None => {}
        Some(val) => port = val,
    }

    match std::env::var("ALEX_DB_SAVED_WRITES_SLEEP") {
        Err(_) => {}
        Ok(val) => {
            saved_writes_sleep = val.parse::<u64>()?;
        }
    }

    match args.saved_writes_sleep {
        None => {}
        Some(val) => saved_writes_sleep = val,
    }

    match std::env::var("ALEX_DB_SAVED_WRITES_THRESHOLD") {
        Err(_) => {}
        Ok(val) => {
            saved_writes_threshold = val.parse::<u16>()?;
        }
    }

    match args.saved_writes_threshold {
        None => {}
        Some(val) => saved_writes_threshold = val,
    }

    match std::env::var("ALEX_DB_SECURITY_API_KEYS") {
        Err(_) => {}
        Ok(val) => {
            security_api_keys = val.parse::<bool>()?;
        }
    }

    match args.security_api_keys {
        None => {}
        Some(val) => security_api_keys = val,
    }

    info!("data_dir = {:?}", data_dir);
    info!("port = {}", port);
    info!("saved_writes_sleep = {}", saved_writes_sleep);
    info!("saved_writes_threshold = {}", saved_writes_threshold);
    info!("security_api_keys = {}", security_api_keys);

    Ok(Config::new(
        data_dir,
        port,
        saved_writes_sleep,
        saved_writes_threshold,
        security_api_keys,
    ))
}
