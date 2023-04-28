use serde::{de::Error, Serialize, Deserialize, Deserializer};

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


fn hex_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    // do better hex decoding than this
    let u64 = u64::from_str_radix(&s[2..], 16).map_err(D::Error::custom);
    Ok(u64.unwrap() as i64)
}