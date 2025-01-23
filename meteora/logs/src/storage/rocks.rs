use crate::{error::SolyticsResult, models::log::LogEvent};
use log::info;
use rocksdb::{Options, DB};

pub struct RocksStorage {
    pub(crate) db: DB,
}

impl RocksStorage {
    pub fn new(path: &str) -> SolyticsResult<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let db = DB::open(&opts, path)?;

        Ok(Self { db })
    }

    pub fn store_log(&mut self, event: &LogEvent) -> SolyticsResult<()> {
        let key = format!("{}:{}", event.slot, event.signature);
        info!("Storing log event with key: {}", key);

        let value = bincode::serialize(&event)?;
        self.db.put(key.as_bytes(), value)?;
        Ok(())
    }
}
