use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct StatRecord {
    pub reads: u128,
    pub requests: u128,
    pub saved_at: Option<DateTime<Utc>>,
    pub saved_writes: u128,
    pub writes: u128,
}

impl StatRecord {
    pub fn can_save(&self, save_triggered_after_ms: i64, save_triggered_by_threshold: u16) -> bool {
        let mut can_save = false;

        match self.saved_at {
            None => can_save = true,
            Some(saved_at) => {
                if saved_at + Duration::milliseconds(save_triggered_after_ms) < Utc::now() {
                    can_save = true;
                }
            }
        }

        if self.writes >= self.saved_writes + u128::from(save_triggered_by_threshold) {
            can_save = true;
        }

        can_save
    }

    pub fn inc_reads(&mut self) -> u128 {
        self.reads += 1;

        self.reads
    }

    pub fn inc_requests(&mut self) -> u128 {
        self.requests += 1;

        self.requests
    }

    pub fn inc_writes(&mut self) -> u128 {
        self.writes += 1;

        self.writes
    }

    pub fn update_saved_writes(&mut self) -> u128 {
        self.saved_at = Some(Utc::now());
        self.saved_writes = self.writes;

        self.saved_writes
    }
}
