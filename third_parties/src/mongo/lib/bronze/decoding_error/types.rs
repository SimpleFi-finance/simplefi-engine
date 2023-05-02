use serde::{Serialize, Deserialize};

#[derive(Debug,Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecodingError {
    timestamp: i64,
    contract_address: String,
    error: String, // invalid_data, missing_abi, missing_event
    log: String, // tx_hash, log_index, tx_index
}