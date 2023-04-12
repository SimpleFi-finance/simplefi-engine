use serde::{Serialize, Deserialize};


#[derive(Debug,PartialEq, Clone, Serialize, Deserialize)]
pub struct Log {
    pub timestamp: i64,
    pub year: i16,
    pub month: i8,
    pub day: i8,
    pub block_number: i64,
    pub block_hash: String,
    pub transaction_hash: String,
    pub transaction_index: i64,
    pub address: String,
    pub data: String,
    pub topics: Vec<String>,
    pub log_index: i64,
    pub transaction_log_index: i64,
    pub removed: bool,
    pub log_type: String,
}