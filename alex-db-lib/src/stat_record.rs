use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct StatRecord {
    pub reads: u128,
    pub requests: u128,
    pub saved_writes: u128,
    pub writes: u128,
}

impl StatRecord {
    pub fn can_save(&self, saved_writes_threshold: u16) -> bool {
        self.writes >= self.saved_writes + u128::from(saved_writes_threshold)
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
        self.saved_writes = self.writes;

        self.saved_writes
    }
}
