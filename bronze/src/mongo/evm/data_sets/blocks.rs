use chains_drivers::types::base::{EntityTimestamp, EntityBlockNumber};
use serde::{Serialize, Deserialize};


#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub timestamp: i64,

    pub year: i16,

    pub month: i8,

    pub day: i8,

    pub number: i64,

    pub hash: Option<String>,

    pub parent_hash: Option<String>,

    pub transactions_root: Option<String>,

    pub receipts_root: Option<String>,

    pub mix_hash: Option<String>,

    pub difficulty: i64,

    pub extra_data: Option<String>,

    pub gas_used: i64,

    pub gas_limit: i64,

    pub logs_bloom: Option<String>,

    pub miner: Option<String>,

    pub nonce: i64,

    pub uncles_hash: Option<String>,

    pub state_root: Option<String>,

    pub base_fee_per_gas: i64,

    pub withdrawals_root: Option<String>,
}


impl EntityTimestamp for Block {
    fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

impl EntityBlockNumber for Block {
    fn block_number(&self) -> i64 {
        self.number
    }
}