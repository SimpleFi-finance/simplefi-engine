use serde::{de::Error, Serialize, Deserialize, Deserializer};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecodedLog {
    pub timestamp: i64,
    pub year: i16,
    pub month: i8,
    pub day: i8,
    pub block_number: i64,
    pub block_hash: String,
    pub transaction_hash: String,
    pub transaction_index: i64,
    pub address: String,
    pub log_index: i64,
    pub transaction_log_index: i64,
    pub topics: Vec<String>,
    pub decoded_data: Vec<String>,
    pub removed: bool,
    pub log_type: Option<String>,
}

