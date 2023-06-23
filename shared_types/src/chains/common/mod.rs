use serde_json::Value;
pub mod chain;
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