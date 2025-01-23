use crate::error::SolyticsResult;
use crate::models::swap::SwapEvent;
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

    pub fn store_swap(&mut self, swap: &SwapEvent) -> SolyticsResult<()> {
        let key = format!("{}:{}:{}", swap.lb_pair, swap.slot, swap.signature);
        info!("Storing swap event with key: {}", key);

        let value = bincode::serialize(&swap)?;
        self.db.put(key.as_bytes(), value)?;
        Ok(())
    }

    // pub fn get_batch_for_migration_range(
    //     &self,
    //     start_slot: u64,
    //     end_slot: u64,
    // ) -> SolyticsResult<(WriteBatch, Vec<SwapEvent>)> {
    //     let mut batch = WriteBatch::default();
    //     let mut events = Vec::new();

    //     let iter = self.db.iterator(rocksdb::IteratorMode::Start);
    //     for item in iter {
    //         let (key, value) = item?;
    //         let key_str = String::from_utf8(key.to_vec())?;
    //         let slot = match key_str.split(':').nth(1) {
    //             Some(slot_str) => slot_str.parse::<u64>()?,
    //             None => continue,
    //         };

    //         if slot > start_slot && slot <= end_slot {
    //             let event: SwapEvent = bincode::deserialize(&value)?;
    //             events.push(event);
    //             batch.delete(key);
    //         }
    //     }

    //     Ok((batch, events))
    // }
}
