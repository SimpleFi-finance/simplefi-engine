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

#[derive(Debug,PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct DecodedData {
    pub name: String,
    pub value: String,
    pub kind: String,
    pub indexed: bool,
    pub hash_signature: String,
    pub signature: String,
}