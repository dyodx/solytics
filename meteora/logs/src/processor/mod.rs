use crate::models::log::LogEvent;
use crate::storage::Storage;
use async_trait::async_trait;
use carbon_core::{
    datasource::LogsUpdate, error::CarbonResult, metrics::MetricsCollection, processor::Processor,
};
use chrono::Utc;
use log::info;
use std::sync::Arc;

pub struct MeteoraLogsProcessor {
    storage: Storage,
}

impl MeteoraLogsProcessor {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl Processor for MeteoraLogsProcessor {
    type InputType = LogsUpdate;

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        info!("Processing Meteora logs!!!!!!!!!!!!!!!!!");

        let event = LogEvent {
            signature: data.signature.to_string(),
            slot: data.slot,
            timestamp: Utc::now(),
            logs: data.logs,
        };

        self.storage
            .store_log(event)
            .await
            .map_err(|e| carbon_core::error::Error::Custom(format!("Storage error: {}", e)))?;

        Ok(())
    }
}
