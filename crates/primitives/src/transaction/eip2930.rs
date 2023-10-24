use super::access_list::AccessList;
use crate::{Address, Bytes, ChainId, Signature, TransactionKind, TxHash, TxType};
use rlp::{length_of_length, Decodable, DecodeError, Encodable, Header};
use serde::{de::Error, Deserialize, Deserializer};
use serde_json::Value;
use sip_codecs::{main_codec, Compact};
use std::mem;

/// Transaction with an [`AccessList`] ([EIP-2930](https://eips.ethereum.org/EIPS/eip-2930)).
#[main_codec]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TxEip2930 {
    pub hash: TxHash,

    #[serde(default, rename = "chainId", deserialize_with = "hex_to_u64")]
    pub chain_id: ChainId,

    #[serde(default, deserialize_with = "hex_to_u64")]
    pub nonce: u64,

    pub from: Address,

    #[serde(default, rename = "gasPrice", deserialize_with = "hex_to_u128")]
    pub gas_price: u128,

    #[serde(default, rename = "gas", deserialize_with = "hex_to_u64")]
    pub gas_limit: u64,

    #[serde(deserialize_with = "default_if_empty")]
    pub to: TransactionKind,

    #[serde(default, rename = "v", deserialize_with = "hex_to_u128")]
    pub value: u128,

    #[serde(default, rename = "accessList")]
    pub access_list: AccessList,

    #[serde(default, rename = "blockNumber", deserialize_with = "hex_to_u64")]
    pub block_number: u64,

    #[serde(default, rename = "transactionIndex", deserialize_with = "hex_to_u64")]
    pub transaction_index: u64,

    pub input: Bytes,
}

impl From<Value> for TxEip2930 {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

impl From<String> for TxEip2930 {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap()
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

impl TxEip2930 {
    /// Calculates a heuristic for the in-memory size of the [TxEip2930] transaction.
    #[inline]
    pub fn size(&self) -> usize {
        mem::size_of::<TxHash>() + // hash
        mem::size_of::<ChainId>() + // chain_id
        mem::size_of::<u64>() + // nonce
        mem::size_of::<Address>() + // from
        mem::size_of::<u128>() + // gas_price
        mem::size_of::<u64>() + // gas_limit
        self.to.size() + // to
        mem::size_of::<u128>() + // value
        self.access_list.size() + // access_list
        self.input.len() +// input
        mem::size_of::<u64>() + // block_number
        mem::size_of::<u64>() // transaction_index
    }

    pub(crate) fn decode_inner(buf: &mut &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            hash: Decodable::decode(buf)?,
            chain_id: Decodable::decode(buf)?,
            nonce: Decodable::decode(buf)?,
            from: Decodable::decode(buf)?,
            gas_price: Decodable::decode(buf)?,
            gas_limit: Decodable::decode(buf)?,
            to: Decodable::decode(buf)?,
            value: Decodable::decode(buf)?,
            input: Bytes(Decodable::decode(buf)?),
            access_list: Decodable::decode(buf)?,
            block_number: Decodable::decode(buf)?,
            transaction_index: Decodable::decode(buf)?,
        })
    }

    /// Outputs the length of the transaction's fields, without a RLP header.
    pub(crate) fn fields_len(&self) -> usize {
        let mut len = 0;
        len += self.hash.length();
        len += self.chain_id.length();
        len += self.nonce.length();
        len += self.from.length();
        len += self.gas_price.length();
        len += self.gas_limit.length();
        len += self.to.length();
        len += self.value.length();
        len += self.input.0.length();
        len += self.access_list.length();
        len += self.block_number.length();
        len += self.transaction_index.length();

        len
    }

    /// Encodes only the transaction's fields into the desired buffer, without a RLP header.
    pub(crate) fn encode_fields(&self, out: &mut dyn bytes::BufMut) {
        self.hash.encode(out);
        self.chain_id.encode(out);
        self.nonce.encode(out);
        self.from.encode(out);
        self.gas_price.encode(out);
        self.gas_limit.encode(out);
        self.to.encode(out);
        self.value.encode(out);
        self.input.0.encode(out);
        self.access_list.encode(out);
        self.block_number.encode(out);
        self.transaction_index.encode(out);
    }

    /// Inner encoding function that is used for both rlp [`Encodable`] trait and for calculating
    /// hash that for eip2718 does not require rlp header
    pub(crate) fn encode_with_signature(
        &self,
        signature: &Signature,
        out: &mut dyn bytes::BufMut,
        with_header: bool,
    ) {
        let payload_length = self.fields_len() + signature.payload_len();
        if with_header {
            Header {
                list: false,
                payload_length: 1 + length_of_length(payload_length) + payload_length,
            }
            .encode(out);
        }
        out.put_u8(self.tx_type() as u8);
        let header = Header {
            list: true,
            payload_length,
        };
        header.encode(out);
        self.encode_fields(out);
        signature.encode(out);
    }

    /// Output the length of the RLP signed transaction encoding. This encodes with a RLP header.
    pub(crate) fn payload_len_with_signature(&self, signature: &Signature) -> usize {
        let payload_length = self.fields_len() + signature.payload_len();
        // 'transaction type byte length' + 'header length' + 'payload length'
        let len = 1 + length_of_length(payload_length) + payload_length;
        length_of_length(len) + len
    }

    /// Get transaction type
    pub(crate) fn tx_type(&self) -> TxType {
        TxType::EIP2930
    }
}

// #[cfg(test)]
// mod tests {
//     use super::TxEip2930;
//     use crate::{
//         transaction::{signature::Signature, TransactionKind},
//         Address, Bytes, Transaction, U256,
//     };
//     use bytes::BytesMut;

//     #[test]
//     fn test_decode_create() {
//         // tests that a contract creation tx encodes and decodes properly
//         let request = Transaction::Eip2930(TxEip2930 {
//             hash: Default::default(),
//             chain_id: 1u64,
//             nonce: 0,
//             gas_price: 1,
//             gas_limit: 2,
//             to: TransactionKind::Create,
//             value: 3,
//             input: Bytes::from(vec![1, 2]),
//             access_list: Default::default(),
//         });
//         let signature = Signature { odd_y_parity: true, r: U256::default(), s: U256::default() };

//         let mut encoded = BytesMut::new();
//     }

//     #[test]
//     fn test_decode_call() {
//         let request = Transaction::Eip2930(TxEip2930 {
//             hash: Default::default(),
//             chain_id: 1u64,
//             nonce: 0,
//             gas_price: 1,
//             gas_limit: 2,
//             to: TransactionKind::Call(Address::default()),
//             value: 3,
//             input: Bytes::from(vec![1, 2]),
//             access_list: Default::default(),
//         });

//         let signature = Signature { odd_y_parity: true, r: U256::default(), s: U256::default() };

//         let mut encoded = BytesMut::new();
//     }
// }
