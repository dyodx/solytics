mod error;
mod models;
mod processor;
mod storage;
use carbon_core::error::{CarbonResult, Error as CarbonError};
use carbon_log_metrics::LogMetrics;
use carbon_meteora_dlmm_decoder::MeteoraDlmmDecoder;
use carbon_rpc_transaction_crawler_datasource::{Filters, RpcTransactionCrawler};
use log::info;
use processor::MeteoraInstructionProcessor;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey, pubkey::Pubkey};
use std::{env, sync::Arc, time::Duration};
use storage::Storage;

pub const METEORA_PROGRAM_ID: Pubkey = pubkey!("LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo");

#[tokio::main]
pub async fn main() -> CarbonResult<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    tokio::spawn(async move {
        if let Ok(()) = tokio::signal::ctrl_c().await {
            println!("Received Ctrl+C, forcing shutdown...");
            tokio::time::sleep(Duration::from_millis(100)).await;
            std::process::exit(0);
        }
    });

    let filters = Filters::new(None, None, None);

    let transaction_crawler = RpcTransactionCrawler::new(
        env::var("RPC_URL").unwrap_or_default(),
        METEORA_PROGRAM_ID,
        100,
        Duration::from_secs(5),
        filters,
        Some(CommitmentConfig::finalized()),
        5,
    );

    let storage = Storage::new(env::var("ROCKS_DB_PATH").unwrap().as_str())
        .map_err(|e| CarbonError::Custom(format!("Storage initialization error: {}", e)))?;

    let processor = MeteoraInstructionProcessor::new(storage);

    carbon_core::pipeline::Pipeline::builder()
        .datasource(transaction_crawler)
        .metrics(Arc::new(LogMetrics::new()))
        .metrics_flush_interval(3)
        .instruction(MeteoraDlmmDecoder, processor)
        .build()?
        .run()
        .await?;

    Ok(())
}
