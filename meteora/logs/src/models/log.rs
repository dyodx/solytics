use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEvent {
    pub signature: String,
    pub slot: u64,
    pub timestamp: DateTime<Utc>,
    pub logs: Vec<String>,
}
