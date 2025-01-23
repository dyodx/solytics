CREATE DATABASE IF NOT EXISTS meteora;

CREATE TABLE IF NOT EXISTS meteora.meteora_swaps (
    signature String,
    slot UInt64,
    timestamp DateTime,
    lb_pair String,
    from_wallet String,
    amount_in UInt64,
    amount_out UInt64,
    start_bin_id Int32,
    end_bin_id Int32,
    swap_for_y UInt8,
    fee UInt64,
    protocol_fee UInt64,
    fee_bps UInt64,
    host_fee UInt64
) ENGINE = MergeTree()
PARTITION BY toYYYYMMDD(timestamp)
ORDER BY (timestamp, slot, lb_pair)
SETTINGS index_granularity = 8192;

-- Create materialized views for common queries
CREATE MATERIALIZED VIEW IF NOT EXISTS meteora.swap_volume_daily
ENGINE = SummingMergeTree()
PARTITION BY toYYYYMMDD(day)
ORDER BY (lb_pair, toDate(day))
AS SELECT
    lb_pair,
    toDate(timestamp) as day, -- Converts timestamp to date, removing time component
    count() as swap_count,
    sum(amount_in) as total_amount_in,
    sum(amount_out) as total_amount_out,
    sum(fee) as total_fees
FROM meteora.meteora_swaps
GROUP BY lb_pair, toDate(timestamp);
