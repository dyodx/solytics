// use crate::error::SolyticsResult;
// use crate::models::swap::SwapEvent;
// use chrono::{DateTime, Utc};
// use clickhouse::{Client, Row};
// use log::info;
// use serde::{Deserialize, Serialize};

// pub struct ClickHouseStorage {
//     client: Client,
// }

// impl ClickHouseStorage {
//     pub fn new(url: &str, database: &str) -> Self {
//         let client = Client::default().with_url(url).with_database(database);
//         Self { client }
//     }

//     pub async fn migrate_events(&self, events: Vec<SwapEvent>) -> SolyticsResult<()> {
//         if events.is_empty() {
//             return Ok(());
//         }

//         info!("Starting migration of {} events", events.len());

//         let mut inserter = self
//             .client
//             .inserter("meteora_swaps")?
//             .with_max_rows(1)
//             .with_max_bytes(10_000_000);

//         for (index, event) in events.iter().enumerate() {
//             info!("Writing event {}/{}", index + 1, events.len());

//             let row = ClickhouseSwapEvent {
//                 signature: event.signature.clone(),
//                 slot: event.slot,
//                 timestamp: event.timestamp,
//                 lb_pair: event.lb_pair.clone(),
//                 from_wallet: event.from_wallet.clone(),
//                 amount_in: event.amount_in,
//                 amount_out: event.amount_out,
//                 start_bin_id: event.start_bin_id,
//                 end_bin_id: event.end_bin_id,
//                 swap_for_y: if event.swap_for_y { 1u8 } else { 0u8 },
//                 fee: event.fee,
//                 protocol_fee: event.protocol_fee,
//                 fee_bps: 0u64, // Hardcoded for now
//                 host_fee: event.host_fee,
//             };

//             info!("Attempting to write row: {:?}", row);

//             inserter.write(&row)?;
//         }

//         info!("Committing batch");
//         let stats = inserter.end().await?;
//         info!(
//             "Migration completed: {} rows inserted in {} transactions",
//             stats.rows, stats.transactions
//         );
//         Ok(())
//     }
// }

// #[derive(Debug, Serialize, Deserialize, Row)]
// struct ClickhouseSwapEvent {
//     signature: String,
//     slot: u64,
//     timestamp: DateTime<Utc>,
//     lb_pair: String,
//     from_wallet: String,
//     amount_in: u64,
//     amount_out: u64,
//     start_bin_id: i32,
//     end_bin_id: i32,
//     swap_for_y: u8,
//     fee: u64,
//     protocol_fee: u64,
//     fee_bps: u64,
//     host_fee: u64,
// }
