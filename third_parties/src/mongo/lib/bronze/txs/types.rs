use serde::{de::Error, Serialize, Deserialize, Deserializer};

#[derive(Debug,PartialEq, Clone, Serialize, Deserialize)]
pub struct Tx {
    pub timestamp: Option<i64>,
    pub year: Option<i16>,
    pub month: Option<i8>,
    pub day: Option<i8>,

    #[serde(default, rename = "blockNumber", deserialize_with="hex_to_i64")]
    pub block_number: i64,

    pub hash: Option<String>,

    #[serde(default, deserialize_with="hex_to_i32")]
    pub transaction_index: i32,

    pub nonce: Option<String>,

    #[serde(default, rename = "blockHash")]
    pub block_hash: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub value: Option<String>,

    #[serde(default, rename = "gasPrice", deserialize_with="hex_to_i64")]
    pub gas_price: i64,

    #[serde(default, deserialize_with="hex_to_i64")]
    pub gas: i64,
    pub input: Option<String>,

    #[serde(default, deserialize_with="hex_to_i64")]
    pub v: i64,
    pub r: Option<String>,
    pub s: Option<String>,
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

fn hex_to_i32<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    // do better hex decoding than this
    let u32 = u32::from_str_radix(&s[2..], 16).map_err(D::Error::custom);
    Ok(u32.unwrap() as i32)
}