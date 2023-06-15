use chains_drivers::types::base::{EntityBlockNumber, EntityTimestamp, EntityContractAddress};
use serde::{Serialize, Deserialize};

#[derive(Debug,PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct Tx {
    pub timestamp: i64,
    pub year: i16,
    pub month: i8,
    pub day: i8,

    pub block_number: i64,

    pub hash: Option<String>,

    pub transaction_index: i32,

    pub nonce: Option<String>,

    pub block_hash: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub value: Option<String>,

    pub gas_price: i64,

    pub gas: i64,
    pub input: Option<String>,

    pub v: i64,
    pub r: Option<String>,
    pub s: Option<String>,
}


impl EntityBlockNumber for Tx {
    fn block_number(&self) -> i64 {
        self.block_number
    }
}

impl EntityTimestamp for Tx {
    fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

impl EntityContractAddress for Tx {
    fn contract_address(&self) -> String {
        self.from.clone().unwrap_or_default()
    }
}