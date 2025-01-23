mod error;
mod models;
mod processor;
mod storage;
use carbon_core::error::{CarbonResult, Error as CarbonError};
use carbon_log_metrics::LogMetrics;
use carbon_rpc_logs_subscribe_datasource::{Filters, RpcLogsSubscribe};
use processor::MeteoraLogsProcessor;
use solana_client::rpc_config::RpcTransactionLogsConfig;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey, pubkey::Pubkey};
use std::{env, sync::Arc};
use storage::Storage;

pub const METEORA_PROGRAM_ID: Pubkey = pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo");

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let logs_subscribe = RpcLogsSubscribe::new(
        std::env::var("RPC_WS_URL").unwrap(),
        Filters::new(
            vec![METEORA_PROGRAM_ID],
            RpcTransactionLogsConfig {
                commitment: Some(CommitmentConfig::confirmed()),
            },
        ),
    );

    let storage = Storage::new(env::var("ROCKS_DB_PATH").unwrap().as_str())
        .map_err(|e| CarbonError::Custom(format!("Storage initialization error: {}", e)))?;

    let processor = MeteoraLogsProcessor::new(storage);

    carbon_core::pipeline::Pipeline::builder()
        .datasource(logs_subscribe)
        .metrics(Arc::new(LogMetrics::new()))
        .logs(processor)
        .build()?
        .run()
        .await?;

    Ok(())
}
