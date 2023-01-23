use crate::{Args, Result};
use alex_db_lib::config::Config as DbConfig;
use tracing::info;

#[derive(Clone, Debug)]
pub struct Config {
    pub db_config: DbConfig,
    pub port: u16,
}

impl Config {
    pub fn new(db_config: DbConfig, port: u16) -> Self {
        Self { db_config, port }
    }
}

pub fn load(args: Args) -> Result<Config> {
    let mut data_dir = None;
    let mut enable_security_api_keys = true;
    let mut port = 8080;
    let mut save_triggered_after_ms = 600000;
    let mut save_triggered_by_threshold = 8;
    let mut sleep_time_between_gc_ms = 1000;
    let mut sleep_time_between_saves_ms = 10000;

    if let Ok(val) = std::env::var("ALEX_DB_DATA_DIR") {
        data_dir = Some(val)
    }

    if let Some(val) = args.data_dir {
        data_dir = Some(val)
    }

    if let Ok(val) = std::env::var("ALEX_DB_ENABLE_SECURITY_API_KEYS") {
        enable_security_api_keys = val.parse::<bool>()?
    }

    if let Some(val) = args.enable_security_api_keys {
        enable_security_api_keys = val
    }

    if let Ok(val) = std::env::var("ALEX_DB_PORT") {
        port = val.parse::<u16>()?
    }

    if let Some(val) = args.port {
        port = val
    }

    if let Ok(val) = std::env::var("ALEX_DB_SAVE_TRIGGERED_AFTER_MS") {
        save_triggered_after_ms = val.parse::<i64>()?
    }

    if let Some(val) = args.save_triggered_after_ms {
        save_triggered_after_ms = val
    }

    if let Ok(val) = std::env::var("ALEX_DB_SAVE_TRIGGERED_BY_THRESHOLD") {
        save_triggered_by_threshold = val.parse::<u16>()?
    }

    if let Some(val) = args.save_triggered_by_threshold {
        save_triggered_by_threshold = val
    }

    if let Ok(val) = std::env::var("ALEX_DB_SLEEP_TIME_BETWEEN_GC_MS") {
        sleep_time_between_gc_ms = val.parse::<u64>()?
    }

    if let Some(val) = args.sleep_time_between_gc_ms {
        sleep_time_between_gc_ms = val
    }

    if let Ok(val) = std::env::var("ALEX_DB_SLEEP_TIME_BETWEEN_SAVES_MS") {
        sleep_time_between_saves_ms = val.parse::<u64>()?
    }

    if let Some(val) = args.sleep_time_between_saves_ms {
        sleep_time_between_saves_ms = val
    }

    info!("data_dir = {:?}", data_dir);
    info!("enable_security_api_keys = {}", enable_security_api_keys);
    info!("port = {}", port);
    info!("save_triggered_after_ms = {}", save_triggered_after_ms);
    info!(
        "save_triggered_by_threshold = {}",
        save_triggered_by_threshold
    );
    info!("sleep_time_between_gc_ms = {}", sleep_time_between_gc_ms);
    info!(
        "sleep_time_between_saves_ms = {}",
        sleep_time_between_saves_ms
    );

    let db_config = DbConfig::new(
        data_dir,
        enable_security_api_keys,
        save_triggered_after_ms,
        save_triggered_by_threshold,
        sleep_time_between_gc_ms,
        sleep_time_between_saves_ms,
    );

    let config = Config::new(db_config, port);

    Ok(config)
}
