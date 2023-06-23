use chrono::{Datelike, NaiveDateTime};
use serde::{de::Error, Serialize, Deserialize, Deserializer};
use serde_json::{json, Value};

use crate::chains::common::{RawToValue, EntityBlockNumber, EntityContractAddress};

#[derive(Debug,PartialEq, Clone, Serialize, Deserialize, Default)]
pub struct DecodedData {
    pub name: String,
    pub value: String,
    pub kind: String,
    pub indexed: bool,
    pub hash_signature: String,
    pub signature: String,
}

#[derive(Debug,Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Log {
    #[serde(default, rename = "blockNumber", deserialize_with="hex_to_i64")]
    pub block_number: i64,
    #[serde(default, rename = "blockHash")]
    pub block_hash: String,
    #[serde(default, rename = "transactionHash")]
    pub transaction_hash: Option<String>,
    #[serde(default, rename = "transactionIndex", deserialize_with="hex_to_i64")]
    pub transaction_index: i64,

    pub address: Option<String>,

    pub data: Option<String>,

    pub topics: Vec<String>,
    #[serde(default, rename = "logIndex", deserialize_with="hex_to_i64")]
    pub log_index: i64,
    #[serde(default, rename = "transactionLogIndex", deserialize_with="hex_to_i64")]
    pub transaction_log_index: i64,
    pub removed: bool,
    #[serde(default, rename = "logType")]
    pub log_type: Option<String>,
}

impl RawToValue for Log {
    fn raw_to_value(&self, timestamp: i64) -> Value {

        let date = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();

        json!({
            "timestamp": date.timestamp_micros(),
            "year": date.year() as i16,
            "month": date.month() as i8,
            "day": date.day() as i8,
            "block_number": self.block_number,
            "block_hash": self.block_hash.clone(),
            "transaction_hash": self.transaction_hash.clone(),
            "transaction_index": self.transaction_index,
            "address": self.address.clone(),
            "data": self.data.clone(),
            "decoded_data": Option::<Vec<DecodedData>>::None,
            "topics": self.topics.clone(),
            "log_index": self.log_index,
            "transaction_log_index": self.transaction_log_index,
            "removed": self.removed,
            "log_type": self.log_type.clone(),
        })
    }
}


impl EntityBlockNumber for Log {
    fn block_number(&self) -> i64 {
        self.block_number
    }
}

impl EntityContractAddress for Log {
    fn contract_address(&self) -> String {
        self.address.clone().unwrap()
    }
}

fn hex_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    // do better hex decoding than this
    let u64 = u64::from_str_radix(&s[2..], 16).map_err(D::Error::custom);
    Ok(u64.unwrap() as i64)
}