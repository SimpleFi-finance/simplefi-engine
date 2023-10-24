use crate::{Bytes, ChainId, Signature, TransactionKind, TxType, TxHash, Address};
use sip_codecs::{main_codec, Compact};
use rlp::{length_of_length, Encodable, Header, RlpDecodable, RlpEncodable};
use serde::{Deserialize, Deserializer,de::Error};
use serde_json::Value;
use std::{mem, borrow::Cow};
/// Legacy transaction.
#[main_codec]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, RlpDecodable, RlpEncodable)]
pub struct TxLegacy {
    pub hash: TxHash,

    #[serde(default, rename = "chainId", deserialize_with="deserialize_option_u64")]
    pub chain_id: ChainId,

    // TODO: add blockHash

    pub from: Address, 

    // pub block_hash: BlockHash,

    #[serde(default, deserialize_with="hex_to_u64")]
    pub nonce: u64,    
    
    #[serde(default, rename = "gasPrice", deserialize_with="hex_to_u128")]
    pub gas_price: u128,

    #[serde(default, rename = "gas", deserialize_with="hex_to_u64")]
    pub gas_limit: u64,

    #[serde(deserialize_with = "default_if_empty")]
    pub to: TransactionKind,
    
    #[serde(default, rename = "v", deserialize_with="hex_to_u128")]
    pub value: u128,
    
    #[serde(default, rename = "blockNumber", deserialize_with="hex_to_u64")]
    pub block_number: u64,

    #[serde(default, rename = "transactionIndex", deserialize_with="hex_to_u64")]
    pub transaction_index: u64,

    pub input: Bytes,
}

impl From<Value> for TxLegacy {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

impl From<String> for TxLegacy {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap()
    }
}

fn deserialize_option_u64<'de, D: Deserializer<'de>>(de: D) -> Result<u64, D::Error> {
    // first, deserialize into a temporary value so we can do the empty string
    // test.
    let intermediate = <Option<Cow<'de, str>>>::deserialize(de)?;

    // now, try to parse the string as a URL, using pattern matching to handle
    // the empty string case
    match intermediate.as_deref() {
        None | Some("") => Ok(0),
        Some(non_empty_string) => u64::from_str_radix(&non_empty_string[2..], 16)
            .map_err(D::Error::custom),
    }
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

fn hex_to_u128<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    // do better hex decoding than this
    let u128 = u128::from_str_radix(&s[2..], 16).map_err(D::Error::custom);
    Ok(u128.unwrap())
}

// fn hex_to_u256<'de, D>(deserializer: D) -> Result<U256, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let s: String = Deserialize::deserialize(deserializer)?;
//     // do better hex decoding than this
//     let u256 = U256::from_str_radix(&s[2..], 16).map_err(D::Error::custom);
//     Ok(u256.unwrap())
// }


fn default_if_empty<'de, D, T>(de: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de> + Default,
{
    Option::<T>::deserialize(de).map(|x| x.unwrap_or_else(|| T::default()))
}


impl TxLegacy {
    /// Calculates a heuristic for the in-memory size of the [TxLegacy] transaction.
    #[inline]
    pub fn size(&self) -> usize {
        mem::size_of::<TxHash>() + // hash
        mem::size_of::<ChainId>() + // chain_id
        mem::size_of::<Address>() + // from
        mem::size_of::<u64>() + // nonce
        mem::size_of::<u128>() + // gas_price
        mem::size_of::<u64>() + // gas_limit
        self.to.size() + // to
        mem::size_of::<u128>() + // value
        self.input.len() + // input
        mem::size_of::<u64>() + // block_number
        mem::size_of::<u64>() // transaction_index
    }

    /// Outputs the length of the transaction's fields, without a RLP header or length of the
    /// eip155 fields.
    pub(crate) fn fields_len(&self) -> usize {
        let mut len = 0;
        len += self.hash.length();
        len += self.chain_id.length();
        len += self.from.length();
        len += self.nonce.length();
        len += self.gas_price.length();
        len += self.gas_limit.length();
        len += self.to.length();
        len += self.value.length();
        len += self.block_number.length();
        len += self.transaction_index.length();
        len += self.input.0.length();
        len
    }

    /// Encodes only the transaction's fields into the desired buffer, without a RLP header or
    /// eip155 fields.
    pub(crate) fn encode_fields(&self, out: &mut dyn bytes::BufMut) {
        self.hash.encode(out);
        self.chain_id.encode(out);
        self.from.encode(out);
        self.nonce.encode(out);
        self.gas_price.encode(out);
        self.gas_limit.encode(out);
        self.to.encode(out);
        self.value.encode(out);
        self.block_number.encode(out);
        self.transaction_index.encode(out);
        self.input.0.encode(out);
    }

    /// Inner encoding function that is used for both rlp [`Encodable`] trait and for calculating
    /// hash.
    pub(crate) fn encode_with_signature(&self, signature: &Signature, out: &mut dyn bytes::BufMut) {
        let payload_length =
            self.fields_len() + signature.payload_len_with_eip155_chain_id(self.chain_id);
        let header = Header { list: true, payload_length };
        header.encode(out);
        self.encode_fields(out);
        signature.encode_with_eip155_chain_id(out, self.chain_id);
    }

    /// Output the length of the RLP signed transaction encoding.
    pub(crate) fn payload_len_with_signature(&self, signature: &Signature) -> usize {
        let payload_length =
            self.fields_len() + signature.payload_len_with_eip155_chain_id(self.chain_id);
        // 'header length' + 'payload length'
        length_of_length(payload_length) + payload_length
    }

    /// Get transaction type
    pub(crate) fn tx_type(&self) -> TxType {
        TxType::Legacy
    }
}