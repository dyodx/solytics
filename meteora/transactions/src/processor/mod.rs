use crate::models::swap::SwapEvent;
use crate::storage::Storage;
use async_trait::async_trait;
use carbon_core::{
    error::CarbonResult,
    instruction::{DecodedInstruction, InstructionMetadata, NestedInstruction},
    metrics::MetricsCollection,
    processor::Processor,
};
use carbon_meteora_dlmm_decoder::instructions::MeteoraDlmmInstruction;
use chrono::Utc;
use log::info;
use std::sync::Arc;

pub struct MeteoraInstructionProcessor {
    storage: Storage,
}

impl MeteoraInstructionProcessor {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl Processor for MeteoraInstructionProcessor {
    type InputType = (
        InstructionMetadata,
        DecodedInstruction<MeteoraDlmmInstruction>,
        Vec<NestedInstruction>,
    );

    async fn process(
        &mut self,
        data: Self::InputType,
        _metrics: Arc<MetricsCollection>,
    ) -> CarbonResult<()> {
        info!("Processing Meteora instruction!!!!!!!!!!!!!!!!!");
        let (instruction_metadata, decoded_instruction, _nested_instructions) = data;

        if let MeteoraDlmmInstruction::Swap(swap) = decoded_instruction.data {
            let event = SwapEvent {
                signature: instruction_metadata
                    .transaction_metadata
                    .signature
                    .to_string(),
                slot: instruction_metadata.transaction_metadata.slot,
                timestamp: Utc::now(),
                lb_pair: swap.lb_pair.to_string(),
                from_wallet: swap.from.to_string(),
                amount_in: swap.amount_in,
                amount_out: swap.amount_out,
                start_bin_id: swap.start_bin_id,
                end_bin_id: swap.end_bin_id,
                swap_for_y: swap.swap_for_y,
                fee: swap.fee,
                protocol_fee: swap.protocol_fee,
                fee_bps: swap.fee_bps,
                host_fee: swap.host_fee,
            };

            self.storage
                .store_swap(event)
                .await
                .map_err(|e| carbon_core::error::Error::Custom(format!("Storage error: {}", e)))?;
        }

        Ok(())
    }
}
