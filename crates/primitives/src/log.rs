use crate::{Address, Bytes, H256};
use rlp::{RlpDecodable, RlpEncodable};
use serde::{de::Error, Deserializer, Deserialize};
use sip_codecs::{main_codec, Compact};

/// Ethereum Log
#[main_codec]
#[derive(Clone, Debug, PartialEq, Eq, RlpDecodable, RlpEncodable, Default)]
pub struct Log {
    #[serde(default, rename = "transactionHash")]
    pub transaction_hash: H256,
    
    #[serde(default, rename = "transactionindex", deserialize_with="hex_to_u64")]
    pub transaction_index: u64,
    #[serde(default, rename = "blockhash")]
    pub block_hash: H256,
    #[serde(default, rename = "blockNumber", deserialize_with="hex_to_u64")]
    pub block_number: u64,
    #[serde(default, rename = "logIndex", deserialize_with="hex_to_u64")]
    pub log_index: u64,
    /// Contract that emitted this log.
    pub address: Address,
    /// Topics of the log. The number of logs depend on what `LOG` opcode is used.
    pub topics: Vec<H256>,
    /// Arbitrary length data.
    pub data: Bytes,
}

fn hex_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    // do better hex decoding than this
    let u64 = u64::from_str_radix(&s[2..], 16).map_err(D::Error::custom);
    Ok(u64.unwrap())
}

#[main_codec]
#[derive(Clone, Debug, PartialEq, Eq, RlpDecodable, RlpEncodable, Default)]
pub struct StoredLog {
    pub transaction_hash: H256,

    pub transaction_index: u64,

    pub block_hash: H256,

    pub block_number: u64,

    pub log_index: u64,
    /// Contract that emitted this log.
    pub address: Address,
    /// Topics of the log. The number of logs depend on what `LOG` opcode is used.
    pub topics: Vec<H256>,
    /// reference to the decoded log data id (0 is empty)
    pub decoded_data: u64,
    /// Arbitrary length data.
    pub data: Bytes,
}

impl From<Log> for StoredLog {
    fn from(value: Log) -> Self {
        StoredLog {
            transaction_hash: value.transaction_hash,
            transaction_index: value.transaction_index,
            block_hash: value.block_hash,
            block_number: value.block_number,
            log_index: value.log_index,
            address: value.address,
            topics: value.topics,
            decoded_data: 0,
            data: value.data,
        }
    }
}

#[main_codec]
#[derive(Clone, Debug, PartialEq, Eq, RlpDecodable, RlpEncodable, Default)]
pub struct StoredDecodedData {
    pub data: Vec<DecodedData>,
}

#[main_codec]
#[derive(Clone, Debug, PartialEq, Eq, RlpDecodable, RlpEncodable, Default)]
pub struct DecodedData {
    pub name: Vec<u8>,
    pub value: Vec<u8>,
    pub kind: u64,
    pub indexed: bool,
    pub signature: H256,
}
