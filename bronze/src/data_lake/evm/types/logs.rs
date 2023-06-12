use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct LogsSeries {
    pub timestamp_data: Vec<i64>,
    pub year_data: Vec<i16>,
    pub month_data: Vec<i8>,
    pub day_data: Vec<i8>,
    pub block_number_data: Vec<i64>,
    pub address_data: Vec<String>,
    pub transaction_index_data: Vec<i64>,
    pub log_index_data: Vec<i64>,
    pub transaction_hash_data: Vec<String>,
    pub topics_data: Vec<String>,
    pub data_data: Vec<String>,
    pub block_hash_data: Vec<String>,
    pub removed_data: Vec<bool>,
    pub log_type_data: Vec<String>,
    pub transaction_log_index_data: Vec<i64>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct BlockSeries {
    pub timestamp: Vec<i64>, 
    pub year: Vec<i16>, 
    pub month: Vec<i8>, 
    pub day: Vec<i8>, 
    pub number: Vec<i64>, 
    pub hash: Vec<String>, 
    pub parent_hash: Vec<String>, 
    pub uncles_hash: Vec<String>, 
    pub miner: Vec<String>, 
    pub state_root: Vec<String>, 
    pub transactions_root: Vec<String>, 
    pub receipts_root: Vec<String>, 
    pub gas_used: Vec<String>, 
    pub gas_limit: Vec<String>, 
    pub extra_data: Vec<String>, 
    pub logs_bloom: Vec<String>, 
    pub difficulty: Vec<String>, 
    pub total_difficulty: Vec<String>, 
    pub seal_fields: Vec<String>, 
    pub uncles: Vec<String>, 
    pub size: Vec<String>, 
    pub mix_hash: Vec<String>, 
    pub nonce: Vec<String>, 
    pub base_fee_per_gas: Vec<String>, 
}