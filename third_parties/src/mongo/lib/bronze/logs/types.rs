use serde::{Serialize, Deserialize};

#[derive(Debug,Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Log {
    pub timestamp: i64,
    pub year: i16,
    pub month: i8,
    pub day: i8,
    pub block_number: i64,
    pub block_hash: String,
    pub transaction_hash: Option<String>,
    pub transaction_index: i64,

    pub address: Option<String>,

    pub data: Option<String>,

    pub decoded_data: Option<Vec<DecodedData>>,
    pub topics: Vec<String>,
    pub log_index: i64,
    pub transaction_log_index: i64,
    pub removed: bool,
    pub log_type: Option<String>,
}

impl Log {
    pub fn new(
        timestamp: i64,
        year: i16,
        month: i8,
        day: i8,
        block_number: i64,
        block_hash: String,
        transaction_hash: Option<String>,
        transaction_index: i64,

        address: Option<String>,

        data: Option<String>,

        decoded_data: Option<Vec<DecodedData>>,
        topics: Vec<String>,
        log_index: i64,
        transaction_log_index: i64,
        removed: bool,
        log_type: Option<String>,
    ) -> Self {
        Self {
            timestamp,
            year,
            month,
            day,
            block_number,
            block_hash,
            transaction_hash,
            transaction_index,
            address,
            data,
            decoded_data,
            topics,
            log_index,
            transaction_log_index,
            removed,
            log_type,
        }
    }
}

#[derive(Debug,PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct DecodedData {
    pub name: String,
    pub value: String,
    pub kind: String,
    pub indexed: bool,
    pub hash_signature: String,
    pub signature: String,
}