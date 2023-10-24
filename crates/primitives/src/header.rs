use crate::{
    BlockNumber, Bytes, H160, H256,
    H64, U256,
};
use bytes::{Buf, BufMut};

use hex_literal::hex;
use sip_codecs::{derive_arbitrary, main_codec, Compact};
use rlp::{length_of_length, Decodable, Encodable, EMPTY_LIST_CODE, EMPTY_STRING_CODE};
use serde::{Deserialize, Serialize, Deserializer, de::Error,};
use std::{mem, borrow::Cow};

pub const EMPTY_ROOT: H256 =
    H256(hex!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"));

    /// Keccak-256 hash of the RLP of an empty list, KEC("\xc0").
pub const EMPTY_LIST_HASH: H256 =
    H256(hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"));

/// Describes the current head block.
///
/// The head block is the highest fully synced block.
///
/// Note: This is a slimmed down version of [Header], primarily for communicating the highest block
/// with the P2P network and the RPC.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize
)]
pub struct Head {
    /// The number of the head block.
    pub number: BlockNumber,
    /// The hash of the head block.
    pub hash: H256,
    /// The difficulty of the head block.
    pub difficulty: U256,
    /// The total difficulty at the head block.
    pub total_difficulty: U256,
    /// The timestamp of the head block.
    pub timestamp: u64,
}

/// Block header
#[main_codec]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub struct Header {
    // TODO: add totalDifficulty, size, uncles, LogsBloom
    /// hash field
    pub hash: H256,
    /// The Keccak 256-bit hash of the parent
    /// block’s header, in its entirety; formally Hp.
    #[serde(rename = "parentHash")]
    pub parent_hash: H256,
    /// The Keccak 256-bit hash of the ommers list portion of this block; formally Ho.
    #[serde(rename = "sha3Uncles")]
    pub ommers_hash: H256,
    /// The 160-bit address to which all fees collected from the successful mining of this block
    /// be transferred; formally Hc.
    #[serde(rename = "miner")]
    pub beneficiary: H160,
    /// The Keccak 256-bit hash of the root node of the trie structure populated with each
    /// transaction in the transactions list portion of the block; formally Ht.
    /// 
    #[serde(rename = "transactionsRoot")]
    pub transactions_root: H256,
    /// The Keccak 256-bit hash of the withdrawals list portion of this block.
    /// <https://eips.ethereum.org/EIPS/eip-4895>
    #[serde(rename = "withdrawalRoot")]
    pub withdrawals_root: Option<H256>,

    #[serde(rename = "receiptsRoot")]
    pub receipts_root: H256,

    #[serde(rename = "stateRoot")]
    pub state_root: H256,
    /// A scalar value corresponding to the difficulty level of this block. This can be calculated
    /// from the previous block’s difficulty level and the timestamp; formally Hd.
    #[serde(deserialize_with = "hex_to_u256")]
    pub difficulty: U256,
    /// A scalar value equal to the number of ancestor blocks. The genesis block has a number of
    /// zero; formally Hi.
    #[serde(deserialize_with = "hex_to_u64")]
    pub number: BlockNumber,
    /// A scalar value equal to the current limit of gas expenditure per block; formally Hl.
    #[serde(rename = "gasLimit", deserialize_with = "hex_to_u64")]
    pub gas_limit: u64,
    /// A scalar value equal to the total gas used in transactions in this block; formally Hg.
    /// 
    #[serde(rename = "gasUsed", deserialize_with = "hex_to_u64")]
    pub gas_used: u64,
    /// A scalar value equal to the reasonable output of Unix’s time() at this block’s inception;
    /// formally Hs.
    #[serde(deserialize_with = "hex_to_u64")]
    pub timestamp: u64,
    /// A 256-bit hash which, combined with the
    /// nonce, proves that a sufficient amount of computation has been carried out on this block;
    /// formally Hm.
    #[serde(rename = "mixHash")]
    pub mix_hash: H256,
    /// A 64-bit value which, combined with the mixhash, proves that a sufficient amount of
    /// computation has been carried out on this block; formally Hn.
    #[serde(deserialize_with = "hex_to_u64")]
    pub nonce: u64,
    /// A scalar representing EIP1559 base fee which can move up or down each block according
    /// to a formula which is a function of gas used in parent block and gas target
    /// (block gas limit divided by elasticity multiplier) of parent block.
    /// The algorithm results in the base fee per gas increasing when blocks are
    /// above the gas target, and decreasing when blocks are below the gas target. The base fee per
    /// gas is burned.
    /// 
    #[serde(rename = "baseFeePerGas", deserialize_with="deserialize_option_u64", default)]
    pub base_fee_per_gas: Option<u64>,
    /// The total amount of blob gas consumed by the transactions within the block, added in
    /// EIP-4844.
    /// 
    #[serde(rename = "blobGasUsed", deserialize_with="deserialize_option_u64", default)]
    pub blob_gas_used: Option<u64>,
    /// A running total of blob gas consumed in excess of the target, prior to the block. Blocks
    /// with above-target blob gas consumption increase this value, blocks with below-target blob
    /// gas consumption decrease it (bounded at 0). This was added in EIP-4844.
    /// 
    #[serde(rename = "excessBlobGas", deserialize_with="deserialize_option_u64", default)]
    pub excess_blob_gas: Option<u64>,
    /// An arbitrary byte array containing data relevant to this block. This must be 32 bytes or
    /// fewer; formally Hx.
    /// 
    #[serde(rename = "extraData")]
    pub extra_data: Bytes,
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

fn deserialize_option_u64<'de, D: Deserializer<'de>>(de: D) -> Result<Option<u64>, D::Error> {
    // first, deserialize into a temporary value so we can do the empty string
    // test.
    let intermediate = <Option<Cow<'de, str>>>::deserialize(de)?;
    // now, try to parse the string as a URL, using pattern matching to handle
    // the empty string case
    match intermediate.as_deref() {
        None | Some("") => Ok(None),
        Some(non_empty_string) => u64::from_str_radix(&non_empty_string[2..], 16)
            .map(Some)
            .map_err(D::Error::custom),
    }
}

fn hex_to_u256<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    // do better hex decoding than this
    let u256 = U256::from_str_radix(&s[2..], 16).map_err(D::Error::custom);
    Ok(u256.unwrap())
}

impl Default for Header {
    fn default() -> Self {
        Header {
            hash: Default::default(),
            parent_hash: Default::default(),
            ommers_hash: EMPTY_LIST_HASH,
            beneficiary: Default::default(),
            transactions_root: EMPTY_ROOT,
            difficulty: Default::default(),
            number: 0,
            gas_limit: 0,
            gas_used: 0,
            timestamp: 0,
            extra_data: Default::default(),
            mix_hash: Default::default(),
            nonce: 0,
            base_fee_per_gas: None,
            withdrawals_root: None,
            blob_gas_used: None,
            excess_blob_gas: None,
            receipts_root: EMPTY_ROOT,
            state_root: EMPTY_ROOT,
        }
    }
}

impl Header {
    pub fn hash(&self) -> H256 {
        self.hash
    }

    /// Checks if the header is empty - has no transactions and no ommers
    pub fn is_empty(&self) -> bool {
        let txs_and_ommers_empty = self.transaction_root_is_empty() && self.ommers_hash_is_empty();
        if let Some(withdrawals_root) = self.withdrawals_root {
            txs_and_ommers_empty && withdrawals_root == EMPTY_ROOT
        } else {
            txs_and_ommers_empty
        }
    }

    /// Check if the transaction root equals to empty root.
    pub fn transaction_root_is_empty(&self) -> bool {
        self.transactions_root == EMPTY_ROOT
    }

    /// Check if the ommers hash equals to empty hash list.
    pub fn ommers_hash_is_empty(&self) -> bool {
        self.ommers_hash == EMPTY_LIST_HASH
    }

    /// Calculate a heuristic for the in-memory size of the [Header].
    #[inline]
    pub fn size(&self) -> usize {
        mem::size_of::<H256>() + // parent hash
        mem::size_of::<H256>() + // ommers hash
        mem::size_of::<H160>() + // beneficiary
        mem::size_of::<H256>() + // transactions root
        mem::size_of::<Option<H256>>() + // withdrawals root
        mem::size_of::<H256>() + // receipts root
        mem::size_of::<H256>() + // state root
        // mem::size_of::<H256>() + // logs bloom
        mem::size_of::<U256>() + // difficulty
        mem::size_of::<BlockNumber>() + // number
        mem::size_of::<u64>() + // gas limit
        mem::size_of::<u64>() + // gas used
        mem::size_of::<u64>() + // timestamp
        mem::size_of::<H256>() + // mix hash
        mem::size_of::<u64>() + // nonce
        mem::size_of::<Option<u64>>() + // base fee per gas
        mem::size_of::<Option<u64>>() + // blob gas used
        mem::size_of::<Option<u64>>() + // excess blob gas
        self.extra_data.len() // extra data
    }

    fn header_payload_length(&self) -> usize {
        let mut length = 0;
        length += self.hash.length();
        length += self.parent_hash.length();
        length += self.ommers_hash.length();
        length += self.beneficiary.length();
        length += self.transactions_root.length();
        length += self.receipts_root.length();
        length += self.state_root.length();
        // length += self.logs_bloom.length();
        length += self.difficulty.length();
        length += U256::from(self.number).length();
        length += U256::from(self.gas_limit).length();
        length += U256::from(self.gas_used).length();
        length += self.timestamp.length();
        length += self.extra_data.length();
        length += self.mix_hash.length();
        length += H64::from_low_u64_be(self.nonce).length();

        if let Some(base_fee) = self.base_fee_per_gas {
            length += U256::from(base_fee).length();
        } else if self.withdrawals_root.is_some() ||
            self.blob_gas_used.is_some() ||
            self.excess_blob_gas.is_some()
        {
            length += 1; // EMPTY STRING CODE
        }

        if let Some(root) = self.withdrawals_root {
            length += root.length();
        } else if self.blob_gas_used.is_some() || self.excess_blob_gas.is_some() {
            length += 1; // EMPTY STRING CODE
        }

        if let Some(blob_gas_used) = self.blob_gas_used {
            length += U256::from(blob_gas_used).length();
        } else if self.excess_blob_gas.is_some() {
            length += 1; // EMPTY STRING CODE
        }

        // Encode excess blob gas length. If new fields are added, the above pattern will need to
        // be repeated and placeholder length added. Otherwise, it's impossible to tell _which_
        // fields are missing. This is mainly relevant for contrived cases where a header is
        // created at random, for example:
        //  * A header is created with a withdrawals root, but no base fee. Shanghai blocks are
        //    post-London, so this is technically not valid. However, a tool like proptest would
        //    generate a block like this.
        if let Some(excess_blob_gas) = self.excess_blob_gas {
            length += U256::from(excess_blob_gas).length();
        }

        length
    }
}

impl Encodable for Header {
    fn encode(&self, out: &mut dyn BufMut) {
        let list_header =
            rlp::Header { list: true, payload_length: self.header_payload_length() };
        list_header.encode(out);
        self.hash.encode(out);
        self.parent_hash.encode(out);
        self.ommers_hash.encode(out);
        self.beneficiary.encode(out);
        self.transactions_root.encode(out);
        self.receipts_root.encode(out);
        self.state_root.encode(out);
        // self.logs_bloom.encode(out);
        self.difficulty.encode(out);
        U256::from(self.number).encode(out);
        U256::from(self.gas_limit).encode(out);
        U256::from(self.gas_used).encode(out);
        self.timestamp.encode(out);
        self.mix_hash.encode(out);
        H64::from_low_u64_be(self.nonce).encode(out);
        self.extra_data.encode(out);

        // Encode base fee. Put empty string if base fee is missing,
        // but withdrawals root is present.
        if let Some(ref base_fee) = self.base_fee_per_gas {
            U256::from(*base_fee).encode(out);
        } else if self.withdrawals_root.is_some() ||
            self.blob_gas_used.is_some() ||
            self.excess_blob_gas.is_some()
        {
            out.put_u8(EMPTY_STRING_CODE);
        }

        // Encode withdrawals root. Put empty string if withdrawals root is missing,
        // but blob gas used is present.
        if let Some(ref root) = self.withdrawals_root {
            root.encode(out);
        } else if self.blob_gas_used.is_some() || self.excess_blob_gas.is_some() {
            out.put_u8(EMPTY_STRING_CODE);
        }

        // Encode blob gas used. Put empty string if blob gas used is missing,
        // but excess blob gas is present.
        if let Some(ref blob_gas_used) = self.blob_gas_used {
            U256::from(*blob_gas_used).encode(out);
        } else if self.excess_blob_gas.is_some() {
            out.put_u8(EMPTY_LIST_CODE);
        }

        // Encode excess blob gas. If new fields are added, the above pattern will need to be
        // repeated and placeholders added. Otherwise, it's impossible to tell _which_ fields
        // are missing. This is mainly relevant for contrived cases where a header is created
        // at random, for example:
        //  * A header is created with a withdrawals root, but no base fee. Shanghai blocks are
        //    post-London, so this is technically not valid. However, a tool like proptest would
        //    generate a block like this.
        if let Some(ref excess_blob_gas) = self.excess_blob_gas {
            U256::from(*excess_blob_gas).encode(out);
        }
    }

    fn length(&self) -> usize {
        let mut length = 0;
        length += self.header_payload_length();
        length += length_of_length(length);
        length
    }
}

impl Decodable for Header {
    fn decode(buf: &mut &[u8]) -> Result<Self, rlp::DecodeError> {
        let rlp_head = rlp::Header::decode(buf)?;

        if !rlp_head.list {
            return Err(rlp::DecodeError::UnexpectedString)
        }
        let started_len = buf.len();

        let mut this = Self {
            hash: Decodable::decode(buf)?,
            parent_hash: Decodable::decode(buf)?,
            ommers_hash: Decodable::decode(buf)?,
            beneficiary: Decodable::decode(buf)?,
            transactions_root: Decodable::decode(buf)?,
            receipts_root: Decodable::decode(buf)?,
            state_root: Decodable::decode(buf)?,
            // logs_bloom: Decodable::decode(buf)?,
            difficulty: Decodable::decode(buf)?,
            number: U256::decode(buf)?.to::<u64>(),
            gas_limit: U256::decode(buf)?.to::<u64>(),
            gas_used: U256::decode(buf)?.to::<u64>(),
            timestamp: Decodable::decode(buf)?,
            mix_hash: Decodable::decode(buf)?,
            nonce: H64::decode(buf)?.to_low_u64_be(),
            extra_data: Decodable::decode(buf)?,
            base_fee_per_gas: None,
            withdrawals_root: None,
            blob_gas_used: None,
            excess_blob_gas: None,
        };

        if started_len - buf.len() < rlp_head.payload_length {
            if buf.first().map(|b| *b == EMPTY_STRING_CODE).unwrap_or_default() {
                buf.advance(1)
            } else {
                this.base_fee_per_gas = Some(U256::decode(buf)?.to::<u64>());
            }
        }

        // Withdrawals root for post-shanghai headers
        if started_len - buf.len() < rlp_head.payload_length {
            if buf.first().map(|b| *b == EMPTY_STRING_CODE).unwrap_or_default() {
                buf.advance(1)
            } else {
                this.withdrawals_root = Some(Decodable::decode(buf)?);
            }
        }

        // Blob gas used and excess blob gas for post-cancun headers
        if started_len - buf.len() < rlp_head.payload_length {
            if buf.first().map(|b| *b == EMPTY_LIST_CODE).unwrap_or_default() {
                buf.advance(1)
            } else {
                this.blob_gas_used = Some(U256::decode(buf)?.to::<u64>());
            }
        }

        // Decode excess blob gas. If new fields are added, the above pattern will need to be
        // repeated and placeholders decoded. Otherwise, it's impossible to tell _which_ fields are
        // missing. This is mainly relevant for contrived cases where a header is created at
        // random, for example:
        //  * A header is created with a withdrawals root, but no base fee. Shanghai blocks are
        //    post-London, so this is technically not valid. However, a tool like proptest would
        //    generate a block like this.
        if started_len - buf.len() < rlp_head.payload_length {
            this.excess_blob_gas = Some(U256::decode(buf)?.to::<u64>());
        }

        let consumed = started_len - buf.len();
        if consumed != rlp_head.payload_length {
            return Err(rlp::DecodeError::ListLengthMismatch {
                expected: rlp_head.payload_length,
                got: consumed,
            })
        }
        Ok(this)
    }
}

#[derive_arbitrary(rlp)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, Serialize, Deserialize)]
pub enum HeadersDirection {
    /// Falling block number.
    Falling,
    /// Rising block number.
    #[default]
    Rising,
}

impl HeadersDirection {
    /// Returns true for rising block numbers
    pub fn is_rising(&self) -> bool {
        matches!(self, HeadersDirection::Rising)
    }

    /// Returns true for falling block numbers
    pub fn is_falling(&self) -> bool {
        matches!(self, HeadersDirection::Falling)
    }

    /// Converts the bool into a direction.
    ///
    /// Returns:
    ///
    /// [`HeadersDirection::Rising`] block numbers for `reverse == 0 == false`
    /// [`HeadersDirection::Falling`] block numbers for `reverse == 1 == true`
    pub fn new(reverse: bool) -> Self {
        if reverse {
            HeadersDirection::Falling
        } else {
            HeadersDirection::Rising
        }
    }
}

impl Encodable for HeadersDirection {
    fn encode(&self, out: &mut dyn BufMut) {
        bool::from(*self).encode(out)
    }

    fn length(&self) -> usize {
        bool::from(*self).length()
    }
}

impl Decodable for HeadersDirection {
    fn decode(buf: &mut &[u8]) -> Result<Self, rlp::DecodeError> {
        let value: bool = Decodable::decode(buf)?;
        Ok(value.into())
    }
}

impl From<bool> for HeadersDirection {
    fn from(reverse: bool) -> Self {
        Self::new(reverse)
    }
}

impl From<HeadersDirection> for bool {
    fn from(value: HeadersDirection) -> Self {
        match value {
            HeadersDirection::Rising => false,
            HeadersDirection::Falling => true,
        }
    }
}

// mod ethers_compat {
//     use super::*;
//     use ethers_core::types::{Block, H256 as EthersH256};

//     impl From<&Block<EthersH256>> for Header {
//         fn from(block: &Block<EthersH256>) -> Self {
//             Header {
//                 parent_hash: block.parent_hash.0.into(),
//                 number: block.number.unwrap().as_u64(),
//                 gas_limit: block.gas_limit.as_u64(),
//                 difficulty: block.difficulty.into(),
//                 nonce: block.nonce.unwrap().to_low_u64_be(),
//                 extra_data: block.extra_data.0.clone().into(),
//                 state_root: block.state_root.0.into(),
//                 transactions_root: block.transactions_root.0.into(),
//                 receipts_root: block.receipts_root.0.into(),
//                 timestamp: block.timestamp.as_u64(),
//                 mix_hash: block.mix_hash.unwrap().0.into(),
//                 beneficiary: block.author.unwrap().0.into(),
//                 base_fee_per_gas: block.base_fee_per_gas.map(|fee| fee.as_u64()),
//                 ommers_hash: block.uncles_hash.0.into(),
//                 gas_used: block.gas_used.as_u64(),
//                 withdrawals_root: None,
//                 logs_bloom: block.logs_bloom.unwrap_or_default().0.into(),
//                 blob_gas_used: None,
//                 excess_blob_gas: None,
//             }
//         }
//     }

//     impl From<&Block<EthersH256>> for SealedHeader {
//         fn from(block: &Block<EthersH256>) -> Self {
//             let header = Header::from(block);
//             match block.hash {
//                 Some(hash) => header.seal(hash.0.into()),
//                 None => header.seal_slow(),
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
        use super::{Bytes, Decodable, Encodable, Header, H256};
        use crate::{Address, U256};
        use ethers_core::utils::hex::{self};
        use std::str::FromStr;
    #[test]
    fn test_encode_block_header() {
        let expected = hex::decode("f90117a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000940000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000008208ae820d0582115c8215b3821a0aa00000000000000000000000000000000000000000000000000000000000000000880000000000000000827788").unwrap();
        let header = Header {
            difficulty: U256::from(0x8ae_u64),
            number: 0xd05_u64,
            gas_limit: 0x115c_u64,
            gas_used: 0x15b3_u64,
            timestamp: 0x1a0a_u64,
            extra_data: Bytes::from_str("7788").unwrap(),
            ommers_hash: H256::zero(),
            state_root: H256::zero(),
            transactions_root: H256::zero(),
            receipts_root: H256::zero(),
            ..Default::default()
        };

        let mut data = vec![];
        header.encode(&mut data);
        assert_eq!(hex::encode(&data), hex::encode(expected));
        assert_eq!(header.length(), data.len());
    }

    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    #[test]
    fn test_decode_block_header() {
        let data = hex::decode("f90138a08da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceeda00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000940000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000000008208ae820d0582115c8215b3821a0aa00000000000000000000000000000000000000000000000000000000000000000880000000000000000827788").unwrap();
        
        let expected = Header {
            difficulty: U256::from(0x8aeu64),
            number: 0xd05u64,
            gas_limit: 0x115cu64,
            gas_used: 0x15b3u64,
            timestamp: 0x1a0au64,
            ommers_hash: H256::zero(),
            extra_data: Bytes::from_str("7788").unwrap(),
            state_root: H256::zero(),
            transactions_root: H256::zero(),
            receipts_root: H256::zero(),
            hash: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(),
            ..Default::default()
        };

        let header = <Header as Decodable>::decode(&mut data.as_slice()).unwrap();
        assert_eq!(header, expected);

        // make sure the hash matches
        let expected_hash =
            H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed")
                .unwrap();
        assert_eq!(header.hash(), expected_hash);
    }
    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481

    // Test vector from: https://github.com/ethereum/tests/blob/f47bbef4da376a49c8fc3166f09ab8a6d182f765/BlockchainTests/ValidBlocks/bcEIP1559/baseFee.json#L15-L36
    #[test]
    fn test_eip1559_block_header_hash() {
        let expected_hash =
            H256::from_str("935112b766170efb4e8f768498370dc8dd884ca776555f7489eecd3dd7034077")
                .unwrap();

        let header = Header {
            hash: H256::from_str("935112b766170efb4e8f768498370dc8dd884ca776555f7489eecd3dd7034077").unwrap(),
            parent_hash: H256::from_str("e0a94a7a3c9617401586b1a27025d2d9671332d22d540e0af72b069170380f2a").unwrap(),
            ommers_hash: H256::from_str("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347").unwrap(),
            beneficiary: Address::from_str("ba5e000000000000000000000000000000000000").unwrap(),
            state_root: H256::from_str("ec3c94b18b8a1cff7d60f8d258ec723312932928626b4c9355eb4ab3568ec7f7").unwrap(),
            transactions_root: H256::from_str("50f738580ed699f0469702c7ccc63ed2e51bc034be9479b7bff4e68dee84accf").unwrap(),
            receipts_root: H256::from_str("29b0562f7140574dd0d50dee8a271b22e1a0a7b78fca58f7c60370d8317ba2a9").unwrap(),
            // logs_bloom: H256::from_str("29b0562f7140574dd0d50dee8a271b22e1a0a7b78fca58f7c60370d8317ba2a9").unwrap(),
            difficulty: U256::from(0x020000),
            number: 0x01_u64,
            gas_limit: 0x016345785d8a0000_u64,
            gas_used: 0x015534_u64,
            timestamp: 0x079e,
            extra_data: Bytes::from_str("42").unwrap(),
            mix_hash: H256::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
            nonce: 0,
            base_fee_per_gas: Some(0x036b_u64),
            withdrawals_root: None,
            blob_gas_used: None,
            excess_blob_gas: None,
        };
        assert_eq!(header.hash(), expected_hash);
    }
}



//     // Test vector from: https://github.com/ethereum/tests/blob/970503935aeb76f59adfa3b3224aabf25e77b83d/BlockchainTests/ValidBlocks/bcExample/shanghaiExample.json#L15-L34
//     #[test]
//     fn test_decode_block_header_with_withdrawals() {
//         let data = hex::decode("f9021ca018db39e19931515b30b16b3a92c292398039e31d6c267111529c3f2ba0a26c17a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347942adc25665018aa1fe0e6bc666dac8fc2697ff9baa095efce3d6972874ca8b531b233b7a1d1ff0a56f08b20c8f1b89bef1b001194a5a071e515dd89e8a7973402c2e11646081b4e2209b2d3a1550df5095289dabcb3fba0ed9c51ea52c968e552e370a77a41dac98606e98b915092fb5f949d6452fce1c4b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008001887fffffffffffffff830125b882079e42a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b42188000000000000000009a027f166f1d7c789251299535cb176ba34116e44894476a7886fe5d73d9be5c973").unwrap();
//         let expected = Header {
//             parent_hash: H256::from_str(
//                 "18db39e19931515b30b16b3a92c292398039e31d6c267111529c3f2ba0a26c17",
//             )
//             .unwrap(),
//             beneficiary: Address::from_str("2adc25665018aa1fe0e6bc666dac8fc2697ff9ba").unwrap(),
//             state_root: H256::from_str(
//                 "95efce3d6972874ca8b531b233b7a1d1ff0a56f08b20c8f1b89bef1b001194a5",
//             )
//             .unwrap(),
//             transactions_root: H256::from_str(
//                 "71e515dd89e8a7973402c2e11646081b4e2209b2d3a1550df5095289dabcb3fb",
//             )
//             .unwrap(),
//             receipts_root: H256::from_str(
//                 "ed9c51ea52c968e552e370a77a41dac98606e98b915092fb5f949d6452fce1c4",
//             )
//             .unwrap(),
//             number: 0x01,
//             gas_limit: 0x7fffffffffffffff,
//             gas_used: 0x0125b8,
//             timestamp: 0x079e,
//             extra_data: Bytes::from_str("42").unwrap(),
//             mix_hash: H256::from_str(
//                 "56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
//             )
//             .unwrap(),
//             base_fee_per_gas: Some(0x09),
//             withdrawals_root: Some(
//                 H256::from_str("27f166f1d7c789251299535cb176ba34116e44894476a7886fe5d73d9be5c973")
//                     .unwrap(),
//             ),
//             ..Default::default()
//         };
//         let header = <Header as Decodable>::decode(&mut data.as_slice()).unwrap();
//         assert_eq!(header, expected);

//         let expected_hash =
//             H256::from_str("85fdec94c534fa0a1534720f167b899d1fc268925c71c0cbf5aaa213483f5a69")
//                 .unwrap();
//         assert_eq!(header.hash_slow(), expected_hash);
//     }

//     // Test vector from: https://github.com/ethereum/tests/blob/7e9e0940c0fcdbead8af3078ede70f969109bd85/BlockchainTests/ValidBlocks/bcExample/cancunExample.json
//     #[test]
//     fn test_decode_block_header_with_blob_fields_ef_tests() {
//         let data = hex::decode("f90221a03a9b485972e7353edd9152712492f0c58d89ef80623686b6bf947a4a6dce6cb6a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347942adc25665018aa1fe0e6bc666dac8fc2697ff9baa03c837fc158e3e93eafcaf2e658a02f5d8f99abc9f1c4c66cdea96c0ca26406aea04409cc4b699384ba5f8248d92b784713610c5ff9c1de51e9239da0dac76de9cea046cab26abf1047b5b119ecc2dda1296b071766c8b1307e1381fcecc90d513d86b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008001887fffffffffffffff8302a86582079e42a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b42188000000000000000009a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b4218302000080").unwrap();
//         let expected = Header {
//             parent_hash: H256::from_str(
//                 "3a9b485972e7353edd9152712492f0c58d89ef80623686b6bf947a4a6dce6cb6",
//             )
//             .unwrap(),
//             ommers_hash: H256::from_str(
//                 "1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
//             )
//             .unwrap(),
//             beneficiary: Address::from_str("2adc25665018aa1fe0e6bc666dac8fc2697ff9ba").unwrap(),
//             state_root: H256::from_str(
//                 "3c837fc158e3e93eafcaf2e658a02f5d8f99abc9f1c4c66cdea96c0ca26406ae",
//             )
//             .unwrap(),
//             transactions_root: H256::from_str(
//                 "4409cc4b699384ba5f8248d92b784713610c5ff9c1de51e9239da0dac76de9ce",
//             )
//             .unwrap(),
//             receipts_root: H256::from_str(
//                 "46cab26abf1047b5b119ecc2dda1296b071766c8b1307e1381fcecc90d513d86",
//             )
//             .unwrap(),
//             logs_bloom: Default::default(),
//             difficulty: U256::from(0),
//             number: 0x1,
//             gas_limit: 0x7fffffffffffffff,
//             gas_used: 0x02a865,
//             timestamp: 0x079e,
//             extra_data: Bytes::from(vec![0x42]),
//             mix_hash: H256::from_str(
//                 "56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
//             )
//             .unwrap(),
//             nonce: 0,
//             base_fee_per_gas: Some(9),
//             withdrawals_root: Some(
//                 H256::from_str("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421")
//                     .unwrap(),
//             ),
//             blob_gas_used: Some(0x020000),
//             excess_blob_gas: Some(0),
//         };

//         let header = Header::decode(&mut data.as_slice()).unwrap();
//         assert_eq!(header, expected);

//         let expected_hash =
//             H256::from_str("0x10aca3ebb4cf6ddd9e945a5db19385f9c105ede7374380c50d56384c3d233785")
//                 .unwrap();
//         assert_eq!(header.hash_slow(), expected_hash);
//     }

//     #[test]
//     fn test_decode_block_header_with_blob_fields() {
//         // Block from devnet-7
//         let data = hex::decode("f90239a013a7ec98912f917b3e804654e37c9866092043c13eb8eab94eb64818e886cff5a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d4934794f97e180c050e5ab072211ad2c213eb5aee4df134a0ec229dbe85b0d3643ad0f471e6ec1a36bbc87deffbbd970762d22a53b35d068aa056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421b901000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000080830305988401c9c380808464c40d5499d883010c01846765746888676f312e32302e35856c696e7578a070ccadc40b16e2094954b1064749cc6fbac783c1712f1b271a8aac3eda2f232588000000000000000007a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421808401600000").unwrap();
//         let expected = Header {
//             parent_hash: H256::from_str(
//                 "13a7ec98912f917b3e804654e37c9866092043c13eb8eab94eb64818e886cff5",
//             )
//             .unwrap(),
//             ommers_hash: H256::from_str(
//                 "1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
//             )
//             .unwrap(),
//             beneficiary: Address::from_str("f97e180c050e5ab072211ad2c213eb5aee4df134").unwrap(),
//             state_root: H256::from_str(
//                 "ec229dbe85b0d3643ad0f471e6ec1a36bbc87deffbbd970762d22a53b35d068a",
//             )
//             .unwrap(),
//             transactions_root: H256::from_str(
//                 "56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
//             )
//             .unwrap(),
//             receipts_root: H256::from_str(
//                 "56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
//             )
//             .unwrap(),
//             logs_bloom: Default::default(),
//             difficulty: U256::from(0),
//             number: 0x30598,
//             gas_limit: 0x1c9c380,
//             gas_used: 0,
//             timestamp: 0x64c40d54,
//             extra_data: Bytes::from(
//                 hex::decode("d883010c01846765746888676f312e32302e35856c696e7578").unwrap(),
//             ),
//             mix_hash: H256::from_str(
//                 "70ccadc40b16e2094954b1064749cc6fbac783c1712f1b271a8aac3eda2f2325",
//             )
//             .unwrap(),
//             nonce: 0,
//             base_fee_per_gas: Some(7),
//             withdrawals_root: Some(
//                 H256::from_str("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421")
//                     .unwrap(),
//             ),
//             blob_gas_used: Some(0),
//             excess_blob_gas: Some(0x1600000),
//         };

//         let header = Header::decode(&mut data.as_slice()).unwrap();
//         assert_eq!(header, expected);

//         let expected_hash =
//             H256::from_str("0x539c9ea0a3ca49808799d3964b8b6607037227de26bc51073c6926963127087b")
//                 .unwrap();
//         assert_eq!(header.hash_slow(), expected_hash);
//     }

//     #[test]
//     fn sanity_direction() {
//         let reverse = true;
//         assert_eq!(HeadersDirection::Falling, reverse.into());
//         assert_eq!(reverse, bool::from(HeadersDirection::Falling));

//         let reverse = false;
//         assert_eq!(HeadersDirection::Rising, reverse.into());
//         assert_eq!(reverse, bool::from(HeadersDirection::Rising));

//         let mut buf = Vec::new();
//         let direction = HeadersDirection::Falling;
//         direction.encode(&mut buf);
//         assert_eq!(direction, HeadersDirection::decode(&mut buf.as_slice()).unwrap());

//         let mut buf = Vec::new();
//         let direction = HeadersDirection::Rising;
//         direction.encode(&mut buf);
//         assert_eq!(direction, HeadersDirection::decode(&mut buf.as_slice()).unwrap());
//     }
// }
