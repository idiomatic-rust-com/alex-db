use crate::{Args, Result};

#[derive(Clone, Debug)]
pub struct Config {
    pub data_dir: Option<String>,
    pub port: u16,
}

impl Config {
    pub fn new(data_dir: Option<String>, port: u16) -> Self {
        Self { data_dir, port }
    }
}

pub fn load(args: Args) -> Result<Config> {
    let mut data_dir = None;
    let mut port = 8080;

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

    Ok(Config::new(data_dir, port))
}
