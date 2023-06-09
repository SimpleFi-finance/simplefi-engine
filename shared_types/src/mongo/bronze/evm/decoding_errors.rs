use serde::{Serialize, Deserialize};

#[derive(Debug,Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecodingError {
    pub timestamp: i64,
    pub contract_address: String,
    pub error: ErrorType, // invalid_data, missing_abi, missing_event
    pub log: String, // tx_hash, log_index, tx_index
}

#[derive(Debug,Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ErrorType {
    InvalidData,
    UnsupportedDataType,
    #[default]
    EventNotFound,
}