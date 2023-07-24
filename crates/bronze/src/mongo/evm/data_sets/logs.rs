use crate::data_lake::evm::data_sets::logs::LogsSeries;
use chains_types::common::{EntityBlockNumber, EntityContractAddress, EntityTimestamp};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct DecodedData {
    pub name: String,
    pub value: String,
    pub kind: String,
    pub indexed: bool,
    pub hash_signature: String,
    pub signature: String,
}

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
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

impl EntityBlockNumber for Log {
    fn block_number(&self) -> i64 {
        self.block_number
    }
}

impl EntityTimestamp for Log {
    fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

impl EntityContractAddress for Log {
    fn contract_address(&self) -> String {
        self.address.clone().unwrap_or_default()
    }
}

pub fn log_to_logs_series(logs: Vec<Log>) -> LogsSeries {
    let mut timestamp_data: Vec<i64> = Vec::new();
    let mut year_data: Vec<i16> = Vec::new();
    let mut month_data: Vec<i8> = Vec::new();
    let mut day_data: Vec<i8> = Vec::new();
    let mut block_number_data: Vec<i64> = Vec::new();
    let mut address_data: Vec<String> = Vec::new();
    let mut transaction_index_data: Vec<i64> = Vec::new();
    let mut log_index_data: Vec<i64> = Vec::new();
    let mut transaction_hash_data: Vec<String> = Vec::new();
    let mut topics_data: Vec<String> = Vec::new();
    let mut data_data: Vec<String> = Vec::new();
    let mut block_hash_data: Vec<String> = Vec::new();
    let mut removed_data: Vec<bool> = Vec::new();
    let mut log_type_data: Vec<String> = Vec::new();
    let mut transaction_log_index_data: Vec<i64> = Vec::new();

    for log in logs {
        timestamp_data.push(log.timestamp);
        year_data.push(log.year);
        month_data.push(log.month);
        day_data.push(log.day);
        block_number_data.push(log.block_number);
        address_data.push(log.address.unwrap());
        transaction_index_data.push(log.transaction_index);
        log_index_data.push(log.log_index);
        transaction_hash_data.push(log.transaction_hash.unwrap());
        let topics = serde_json::to_string(&log.topics).unwrap();
        topics_data.push(topics);
        data_data.push(log.data.unwrap());
        block_hash_data.push(log.block_hash);
        removed_data.push(log.removed);
        log_type_data.push(log.log_type.unwrap());
        transaction_log_index_data.push(log.transaction_log_index);
    }

    LogsSeries {
        timestamp_data,
        year_data,
        month_data,
        day_data,
        block_number_data,
        address_data,
        transaction_index_data,
        log_index_data,
        transaction_hash_data,
        topics_data,
        data_data,
        block_hash_data,
        removed_data,
        log_type_data,
        transaction_log_index_data,
    }
}
