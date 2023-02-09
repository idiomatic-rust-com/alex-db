use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub data_dir: Option<String>,
    pub enable_security_api_keys: bool,
    pub save_triggered_after_ms: i64,
    pub save_triggered_by_threshold: u16,
    pub sleep_time_between_gc_ms: u64,
    pub sleep_time_between_saves_ms: u64,
}

impl Config {
    pub fn new(
        data_dir: Option<String>,
        enable_security_api_keys: bool,
        save_triggered_after_ms: i64,
        save_triggered_by_threshold: u16,
        sleep_time_between_gc_ms: u64,
        sleep_time_between_saves_ms: u64,
    ) -> Self {
        Self {
            data_dir,
            enable_security_api_keys,
            save_triggered_after_ms,
            save_triggered_by_threshold,
            sleep_time_between_gc_ms,
            sleep_time_between_saves_ms,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new(None, true, 60000, 8, 1000, 10000)
    }
}
