use chrono::{Datelike, NaiveDateTime};
use serde::{de::Error, Serialize, Deserialize, Deserializer};
use serde_json::{Value, json};

use crate::types::base::RawToValue;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block<T> {
    #[serde(default, deserialize_with="hex_to_i64")]
    pub timestamp: i64,

    #[serde(default, deserialize_with="hex_to_i64")]
    pub number: i64,

    pub hash: Option<String>,

    #[serde(default, rename = "parentHash")]
    pub parent_hash: Option<String>,

    #[serde(default, rename = "transactionsRoot")]
    pub transactions_root: Option<String>,

    #[serde(default, rename = "receiptsRoot")]
    pub receipts_root: Option<String>,

    #[serde(rename = "mixHash")]
    pub mix_hash: Option<String>,

    #[serde(default, deserialize_with="hex_to_i64")]
    pub difficulty: i64,

    #[serde(default, rename = "extraData")]
    pub extra_data: Option<String>,

    #[serde(default, rename = "gasUsed", deserialize_with="hex_to_i64")]
    pub gas_used: i64,

    #[serde(default, rename = "gasLimit", deserialize_with="hex_to_i64")]
    pub gas_limit: i64,

    #[serde(rename = "logsBloom")]
    pub logs_bloom: Option<String>,

    pub miner: Option<String>,

    #[serde(default, deserialize_with="hex_to_i64")]
    pub nonce: i64,

    #[serde(default, rename = "sha3Uncles")]
    pub uncles_hash: Option<String>,

    #[serde(default, rename = "stateRoot")]
    pub state_root: Option<String>,

    #[serde(rename = "baseFeePerGas", deserialize_with="hex_to_i64")]
    pub base_fee_per_gas: i64,

    #[serde(rename = "withdrawalsRoot")]
    pub withdrawals_root: Option<String>,

    #[serde(default)]
    pub transactions: Option<Vec<T>>,
}

impl<T> RawToValue for Block<T> {
    #[allow(unused_variables)]
    fn raw_to_value(&self, timestamp: i64) -> Value {
        let date = NaiveDateTime::from_timestamp_opt(self.timestamp, 0).unwrap();
    
        json!({
            "timestamp": date.timestamp_micros(),
            "year": date.year() as i16,
            "month": date.month() as i8,
            "day": date.day() as i8,
            "number": self.number,
            "hash": self.hash.clone(),
            "parent_hash": self.parent_hash.clone(),
            "transactions_root": self.transactions_root.clone(),
            "receipts_root": self.receipts_root.clone(),
            "mix_hash": self.mix_hash.clone(),
            "difficulty": self.difficulty,
            "extra_data": self.extra_data.clone(),
            "gas_used": self.gas_used,
            "gas_limit": self.gas_limit,
            "logs_bloom": self.logs_bloom.clone(),
            "miner": self.miner.clone(),
            "nonce": self.nonce,
            "uncles_hash": self.uncles_hash.clone(),
            "state_root": self.state_root.clone(),
            "base_fee_per_gas": self.base_fee_per_gas,
            "withdrawals_root": self.withdrawals_root.clone(),
        })
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