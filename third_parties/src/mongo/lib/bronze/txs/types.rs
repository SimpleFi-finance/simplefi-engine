use serde::{Serialize, Deserialize};

#[derive(Debug,PartialEq, Clone, Serialize, Deserialize)]
pub struct Tx {
    pub timestamp: i64,
    pub year: i16,
    pub month: i8,
    pub day: i8,
    pub block_number: i64,
    pub hash: String,
    pub transaction_index: i32,
    pub nonce: String,
    pub block_hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas_price: String,
    pub gas: String,
    pub input: String,
    pub v: i64,
    pub r: String,
    pub s: String,
}