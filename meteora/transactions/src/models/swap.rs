use chrono::{DateTime, Utc};
// use clickhouse::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SwapEvent {
    pub signature: String,
    pub slot: u64,
    pub timestamp: DateTime<Utc>,
    pub lb_pair: String,
    pub from_wallet: String,
    pub amount_in: u64,
    pub amount_out: u64,
    pub start_bin_id: i32,
    pub end_bin_id: i32,
    pub swap_for_y: bool,
    pub fee: u64,
    pub protocol_fee: u64,
    pub fee_bps: u128,
    pub host_fee: u64,
}
