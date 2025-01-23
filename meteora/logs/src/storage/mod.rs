mod rocks;

use crate::error::SolyticsResult;
use crate::models::log::LogEvent;

pub struct Storage {
    rocks: rocks::RocksStorage,
}

impl Storage {
    pub fn new(rocks_db_path: &str) -> SolyticsResult<Self> {
        Ok(Self {
            rocks: rocks::RocksStorage::new(rocks_db_path)?,
        })
    }

    pub async fn store_log(&mut self, event: LogEvent) -> SolyticsResult<()> {
        self.rocks.store_log(&event)?;
        Ok(())
    }
}
