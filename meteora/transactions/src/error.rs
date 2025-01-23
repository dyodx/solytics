use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolyticsError {
    // #[error("Storage error: {0}")]
    // Storage(String),
    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),
    #[error("RocksDB error: {0}")]
    RocksDB(#[from] rocksdb::Error),
    // #[error("ClickHouse error: {0}")]
    // ClickHouse(#[from] clickhouse::error::Error),
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("Parse int error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

pub type SolyticsResult<T> = std::result::Result<T, SolyticsError>;
