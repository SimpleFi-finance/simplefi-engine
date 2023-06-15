use serde::{Serialize, Deserialize};
use serde_json::Value;

pub trait RawToValue {
    fn raw_to_value(&self, timestamp: i64) -> Value;
}

pub trait EntityBlockNumber {
    fn block_number(&self) -> i64;
}

pub trait EntityContractAddress {
    fn contract_address(&self) -> String;
}

pub trait EntityTimestamp {
    fn timestamp(&self) -> i64;
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