use crate::{
    BlockHash, BlockNumber, Header, H256, U64, TransactionSigned
};

use fixed_hash::rustc_hex::FromHexError;
use rlp::{Encodable, Decodable, DecodeError};
use serde::{Serialize, Deserialize, Deserializer, Serializer};
use sip_codecs::derive_arbitrary;

use std::{fmt, num::ParseIntError, str::FromStr};

#[derive(
    Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize,
)]
pub struct Block {
    /// Block header.
    pub header: Header,
    /// Transactions in this block.
    pub body: Vec<TransactionSigned>,
}

/// Either a block hash _or_ a block number
#[derive_arbitrary(rlp)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BlockHashOrNumber {
    /// A block hash
    Hash(H256),
    /// A block number
    Number(u64),
}

impl From<H256> for BlockHashOrNumber {
    fn from(value: H256) -> Self {
        BlockHashOrNumber::Hash(value)
    }
}

impl From<u64> for BlockHashOrNumber {
    fn from(value: u64) -> Self {
        BlockHashOrNumber::Number(value)
    }
}

/// Allows for RLP encoding of either a block hash or block number
impl Encodable for BlockHashOrNumber {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        match self {
            Self::Hash(block_hash) => block_hash.encode(out),
            Self::Number(block_number) => block_number.encode(out),
        }
    }
    fn length(&self) -> usize {
        match self {
            Self::Hash(block_hash) => block_hash.length(),
            Self::Number(block_number) => block_number.length(),
        }
    }
}

/// Allows for RLP decoding of a block hash or block number
impl Decodable for BlockHashOrNumber {
    fn decode(buf: &mut &[u8]) -> Result<Self, DecodeError> {
        let header: u8 = *buf.first().ok_or(DecodeError::InputTooShort)?;
        // if the byte string is exactly 32 bytes, decode it into a Hash
        // 0xa0 = 0x80 (start of string) + 0x20 (32, length of string)
        if header == 0xa0 {
            // strip the first byte, parsing the rest of the string.
            // If the rest of the string fails to decode into 32 bytes, we'll bubble up the
            // decoding error.
            let hash = H256::decode(buf)?;
            Ok(Self::Hash(hash))
        } else {
            // a block number when encoded as bytes ranges from 0 to any number of bytes - we're
            // going to accept numbers which fit in less than 64 bytes.
            // Any data larger than this which is not caught by the Hash decoding should error and
            // is considered an invalid block number.
            Ok(Self::Number(u64::decode(buf)?))
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Failed to parse `{input}` as integer: {pares_int_error} or as hex: {hex_error}")]
pub struct ParseBlockHashOrNumberError {
    input: String,
    pares_int_error: ParseIntError,
    hex_error: FromHexError,
}

impl FromStr for BlockHashOrNumber {
    type Err = ParseBlockHashOrNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match u64::from_str(s) {
            Ok(val) => Ok(val.into()),
            Err(pares_int_error) => match H256::from_str(s) {
                Ok(val) => Ok(val.into()),
                Err(hex_error) => Err(ParseBlockHashOrNumberError {
                    input: s.to_string(),
                    pares_int_error,
                    hex_error,
                }),
            },
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseBlockNumberError {
    /// Failed to parse hex value
    #[error(transparent)]
    ParseIntErr(#[from] ParseIntError),
    /// Block numbers should be 0x-prefixed
    #[error(transparent)]
    MissingPrefix(#[from] HexStringMissingPrefixError),
}

#[derive(Debug, Default, thiserror::Error)]
#[non_exhaustive]
#[error("hex string without 0x prefix")]
pub struct HexStringMissingPrefixError;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub enum BlockNumberOrTag {
    /// Latest block
    #[default]
    Latest,
    /// Finalized block accepted as canonical
    Finalized,
    /// Safe head block
    Safe,
    /// Earliest block (genesis)
    Earliest,
    /// Pending block (not yet part of the blockchain)
    Pending,
    /// Block by number from canon chain
    Number(u64),
}

impl BlockNumberOrTag {
    /// Returns the numeric block number if explicitly set
    pub fn as_number(&self) -> Option<u64> {
        match *self {
            BlockNumberOrTag::Number(num) => Some(num),
            _ => None,
        }
    }

    /// Returns `true` if a numeric block number is set
    pub fn is_number(&self) -> bool {
        matches!(self, BlockNumberOrTag::Number(_))
    }

    /// Returns `true` if it's "latest"
    pub fn is_latest(&self) -> bool {
        matches!(self, BlockNumberOrTag::Latest)
    }

    /// Returns `true` if it's "finalized"
    pub fn is_finalized(&self) -> bool {
        matches!(self, BlockNumberOrTag::Finalized)
    }

    /// Returns `true` if it's "safe"
    pub fn is_safe(&self) -> bool {
        matches!(self, BlockNumberOrTag::Safe)
    }

    /// Returns `true` if it's "pending"
    pub fn is_pending(&self) -> bool {
        matches!(self, BlockNumberOrTag::Pending)
    }

    /// Returns `true` if it's "earliest"
    pub fn is_earliest(&self) -> bool {
        matches!(self, BlockNumberOrTag::Earliest)
    }
}

impl From<u64> for BlockNumberOrTag {
    fn from(num: u64) -> Self {
        BlockNumberOrTag::Number(num)
    }
}

impl From<U64> for BlockNumberOrTag {
    fn from(num: U64) -> Self {
        num.as_u64().into()
    }
}

impl Serialize for BlockNumberOrTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            BlockNumberOrTag::Number(ref x) => serializer.serialize_str(&format!("0x{x:x}")),
            BlockNumberOrTag::Latest => serializer.serialize_str("latest"),
            BlockNumberOrTag::Finalized => serializer.serialize_str("finalized"),
            BlockNumberOrTag::Safe => serializer.serialize_str("safe"),
            BlockNumberOrTag::Earliest => serializer.serialize_str("earliest"),
            BlockNumberOrTag::Pending => serializer.serialize_str("pending"),
        }
    }
}

impl<'de> Deserialize<'de> for BlockNumberOrTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl FromStr for BlockNumberOrTag {
    type Err = ParseBlockNumberError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let block = match s {
            "latest" => Self::Latest,
            "finalized" => Self::Finalized,
            "safe" => Self::Safe,
            "earliest" => Self::Earliest,
            "pending" => Self::Pending,
            _number => {
                if let Some(hex_val) = s.strip_prefix("0x") {
                    let number = u64::from_str_radix(hex_val, 16);
                    BlockNumberOrTag::Number(number?)
                } else {
                    return Err(HexStringMissingPrefixError::default().into())
                }
            }
        };
        Ok(block)
    }
}

impl fmt::Display for BlockNumberOrTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlockNumberOrTag::Number(ref x) => format!("0x{x:x}").fmt(f),
            BlockNumberOrTag::Latest => f.write_str("latest"),
            BlockNumberOrTag::Finalized => f.write_str("finalized"),
            BlockNumberOrTag::Safe => f.write_str("safe"),
            BlockNumberOrTag::Earliest => f.write_str("earliest"),
            BlockNumberOrTag::Pending => f.write_str("pending"),
        }
    }
}


/// Block number and hash.
#[derive(Clone, Copy, Hash, Default, PartialEq, Eq)]
pub struct BlockNumHash {
    /// Block number
    pub number: BlockNumber,
    /// Block hash
    pub hash: BlockHash,
}

/// Block number and hash of the forked block.
pub type ForkBlock = BlockNumHash;

impl std::fmt::Debug for BlockNumHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.number).field(&self.hash).finish()
    }
}

impl BlockNumHash {
    /// Creates a new `BlockNumHash` from a block number and hash.
    pub fn new(number: BlockNumber, hash: BlockHash) -> Self {
        Self { number, hash }
    }

    /// Consumes `Self` and returns [`BlockNumber`], [`BlockHash`]
    pub fn into_components(self) -> (BlockNumber, BlockHash) {
        (self.number, self.hash)
    }

    /// Returns whether or not the block matches the given [BlockHashOrNumber].
    pub fn matches_block_or_num(&self, block: &BlockHashOrNumber) -> bool {
        match block {
            BlockHashOrNumber::Hash(hash) => self.hash == *hash,
            BlockHashOrNumber::Number(number) => self.number == *number,
        }
    }
}

impl From<(BlockNumber, BlockHash)> for BlockNumHash {
    fn from(val: (BlockNumber, BlockHash)) -> Self {
        BlockNumHash { number: val.0, hash: val.1 }
    }
}

impl From<(BlockHash, BlockNumber)> for BlockNumHash {
    fn from(val: (BlockHash, BlockNumber)) -> Self {
        BlockNumHash { hash: val.0, number: val.1 }
    }
}
