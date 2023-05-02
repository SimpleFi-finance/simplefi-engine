use serde::{Serialize, Deserialize};

#[derive(Debug,Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecodingError {
    pub timestamp: i64,
    pub contract_address: String,
    pub error: String, // invalid_data, missing_abi, missing_event
    pub log: String, // tx_hash, log_index, tx_index
}