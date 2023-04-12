use serde::{Serialize, Deserialize};


#[derive(Debug,PartialEq, Clone, Serialize, Deserialize)]
pub struct Block {
    pub timestamp: i64,
    pub year: i16,
    pub month: i8,
    pub day: i8,
    pub number: i64,
    pub hash: String,
    pub parent_hash: String,
    pub uncles_hash: String,
    pub author: String,
    pub state_root: String,
    pub transactions_root: String,
    pub receipts_root: String,
    pub gas_used: String,
    pub gas_limit: String,
    pub extra_data: String,
    pub logs_bloom: String,
    pub difficulty: String,
    pub total_difficulty: String,
    pub seal_fields: Vec<String>,
    pub uncles: Vec<String>,
    pub transactions: Vec<String>,
    pub size: String,
    pub mix_hash: String,
    pub nonce: String,
    pub base_fee_per_gas: String,
}