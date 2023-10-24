use super::access_list::AccessList;
use crate::{Address, Bytes, ChainId, Signature, TransactionKind, TxHash, TxType};
use rlp::{length_of_length, Decodable, DecodeError, Encodable, Header};
use sip_codecs::{main_codec, Compact};
use std::mem;

use serde::{de::Error, Deserialize, Deserializer};

/// A transaction with a priority fee ([EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)).
#[main_codec]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TxEip1559 {
    pub hash: TxHash,
    /// Added as EIP-pub 155: Simple replay attack protection
    #[serde(default, rename = "chainId", deserialize_with = "hex_to_u64")]
    pub chain_id: u64,
    /// A scalar value equal to the number of transactions sent by the sender; formally Tn.
    #[serde(default, deserialize_with = "hex_to_u64")]
    pub nonce: u64,

    pub from: Address,

    // TODO: add blockHash
    #[serde(default, rename = "gas", deserialize_with = "hex_to_u64")]
    pub gas_limit: u64,

    #[serde(default, rename = "maxFeePerGas", deserialize_with = "hex_to_u128")]
    pub max_fee_per_gas: u128,

    #[serde(
        default,
        rename = "maxPriorityFeePerGas",
        deserialize_with = "hex_to_u128"
    )]
    pub max_priority_fee_per_gas: u128,

    #[serde(default, deserialize_with = "default_if_empty")]
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

fn hex_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    // do better hex decoding than this
    let u64 = u64::from_str_radix(&s[2..], 16).map_err(D::Error::custom);
    Ok(u64.unwrap())
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

fn hex_to_u128<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    // do better hex decoding than this
    let u128 = u128::from_str_radix(&s[2..], 16).map_err(D::Error::custom);
    Ok(u128.unwrap())
}

fn default_if_empty<'de, D, T>(de: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de> + Default,
{
    Option::<T>::deserialize(de).map(|x| x.unwrap_or_else(|| T::default()))
}

impl TxEip1559 {
    /// Returns the effective gas price for the given `base_fee`.
    pub fn effective_gas_price(&self, base_fee: Option<u64>) -> u128 {
        match base_fee {
            None => self.max_fee_per_gas,
            Some(base_fee) => {
                // if the tip is greater than the max priority fee per gas, set it to the max
                // priority fee per gas + base fee
                let tip = self.max_fee_per_gas.saturating_sub(base_fee as u128);
                if tip > self.max_priority_fee_per_gas {
                    self.max_priority_fee_per_gas + base_fee as u128
                } else {
                    // otherwise return the max fee per gas
                    self.max_fee_per_gas
                }
            }
        }
    }

    pub(crate) fn decode_inner(buf: &mut &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            hash: Decodable::decode(buf)?,
            chain_id: Decodable::decode(buf)?,
            nonce: Decodable::decode(buf)?,
            from: Decodable::decode(buf)?,
            max_priority_fee_per_gas: Decodable::decode(buf)?,
            max_fee_per_gas: Decodable::decode(buf)?,
            gas_limit: Decodable::decode(buf)?,
            to: Decodable::decode(buf)?,
            value: Decodable::decode(buf)?,
            input: Bytes(Decodable::decode(buf)?),
            access_list: Decodable::decode(buf)?,
            block_number: Decodable::decode(buf)?,
            transaction_index: Decodable::decode(buf)?,
        })
    }

    /// Encodes only the transaction's fields into the desired buffer, without a RLP header.
    pub(crate) fn fields_len(&self) -> usize {
        let mut len = 0;
        len += self.hash.length();
        len += self.chain_id.length();
        len += self.nonce.length();
        len += self.from.length();
        len += self.max_priority_fee_per_gas.length();
        len += self.max_fee_per_gas.length();
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
        self.max_priority_fee_per_gas.encode(out);
        self.max_fee_per_gas.encode(out);
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
        TxType::EIP1559
    }

    /// Calculates a heuristic for the in-memory size of the [TxEip1559] transaction.
    #[inline]
    pub fn size(&self) -> usize {
        mem::size_of::<TxHash>() + // hash
        mem::size_of::<ChainId>() + // chain_id
        mem::size_of::<u64>() + // nonce
        mem::size_of::<Address>() + // from
        mem::size_of::<u64>() + // gas_limit
        mem::size_of::<u128>() + // max_fee_per_gas
        mem::size_of::<u128>() + // max_priority_fee_per_gas
        self.to.size() + // to
        mem::size_of::<u128>() + // value
        self.access_list.size() + // access_list
        self.input.len() +// input
        mem::size_of::<u64>() + // block_number
        mem::size_of::<u64>() // transaction_index
    }
}

// #[cfg(test)]
// mod tests {
//     use super::TxEip1559;
//     use crate::{
//         transaction::TransactionKind,
//         AccessList, Transaction, H256,
//     };
//     use hex_literal::hex;

//     #[test]
//     fn recover_signer_eip1559() {
//         let hash: H256 =
//             hex!("0ec0b6a2df4d87424e5f6ad2a654e27aaeb7dac20ae9e8385cc09087ad532ee0").into();

//         let tx = Transaction::Eip1559( TxEip1559 {
//             hash,
//             block_number: 0x1,
//             transaction_index: 0x0,
//             chain_id: 1,
//             nonce: 0x42,
//             gas_limit: 44386,
//             to: TransactionKind::Call( hex!("6069a6c32cf691f5982febae4faf8a6f3ab2f0f6").into()),
//             value: 0,
//             input:  hex!("a22cb4650000000000000000000000005eee75727d804a2b13038928d36f8b188945a57a0000000000000000000000000000000000000000000000000000000000000000").into(),
//             max_fee_per_gas: 0x4a817c800,
//             max_priority_fee_per_gas: 0x3b9aca00,
//             access_list: AccessList::default(),
//         });
//         assert_eq!(tx.hash(), hash);
//     }
// }
