use crate::{Args, Result};

#[derive(Clone, Debug)]
pub struct Config {
    pub data_dir: Option<String>,
    pub port: u16,
    pub saved_writes_threshold: u16,
}

impl Config {
    pub fn new(data_dir: Option<String>, port: u16, saved_writes_threshold: u16) -> Self {
        Self {
            data_dir,
            port,
            saved_writes_threshold,
        }
    }
}

pub fn load(args: Args) -> Result<Config> {
    let mut data_dir = None;
    let mut port = 8080;
    let mut saved_writes_threshold = 8;

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

    Ok(Config::new(data_dir, port, saved_writes_threshold))
}
