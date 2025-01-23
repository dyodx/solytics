// mod clickhouse;
mod rocks;

use crate::error::SolyticsResult;
use crate::models::swap::SwapEvent;

pub struct Storage {
    rocks: rocks::RocksStorage,
    // clickhouse: clickhouse::ClickHouseStorage,
    // last_migration_slot: u64,
    // migration_threshold: u64,
}

impl Storage {
    pub fn new(
        rocks_db_path: &str,
        // clickhouse_url: &str,
        // clickhouse_db: &str,
    ) -> SolyticsResult<Self> {
        // let migration_threshold: u64 = env::var("MIGRATION_THRESHOLD")
        //     .map_err(|_| SolyticsError::Storage("MIGRATION_THRESHOLD not set".into()))?
        //     .parse()
        //     .map_err(|_| SolyticsError::Storage("MIGRATION_THRESHOLD must be a number".into()))?;

        Ok(Self {
            rocks: rocks::RocksStorage::new(rocks_db_path)?,
            // clickhouse: clickhouse::ClickHouseStorage::new(clickhouse_url, clickhouse_db),
            // last_migration_slot: 0,
            // migration_threshold,
        })
    }

    pub async fn store_swap(&mut self, swap: SwapEvent) -> SolyticsResult<()> {
        self.rocks.store_swap(&swap)?;
        // self.maybe_migrate_to_clickhouse(swap.slot).await?;
        Ok(())
    }

    // async fn maybe_migrate_to_clickhouse(&mut self, current_slot: u64) -> SolyticsResult<()> {
    //     if current_slot - self.last_migration_slot > self.migration_threshold {
    //         let (batch, events) = self
    //             .rocks
    //             .get_batch_for_migration_range(self.last_migration_slot, current_slot)?;

    //         if !events.is_empty() {
    //             self.clickhouse.migrate_events(events).await?;
    //             self.rocks.db.write(batch)?;
    //         }

    //         self.last_migration_slot = current_slot;
    //         info!("Migrated events to ClickHouse up to slot {}", current_slot);
    //     }

    //     Ok(())
    // }
}
