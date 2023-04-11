use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    timestamp: i64,
    year: i16,
    month: i8,
    day: i8,
    number: i64,
    hash: String,
    parent_hash: String,
    uncles_hash: String,
    author: String,
    state_root: String,
    transactions_root: String,
    receipts_root: String,
    gas_used: String,
    gas_limit: String,
    extra_data: String,
    logs_bloom: String,
    difficulty: String,
    total_difficulty: String,
    seal_fields: Vec<String>,
    uncles: Vec<String>,
    transactions: Vec<String>,
    size: String,
    mix_hash: String,
    nonce: String,
    base_fee_per_gas: String,
}