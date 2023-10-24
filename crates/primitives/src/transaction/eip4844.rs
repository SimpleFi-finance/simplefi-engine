use super::access_list::AccessList;
use crate::{
    constants::eip4844::DATA_GAS_PER_BLOB,
    kzg::{
        self, Blob, Bytes48, KzgCommitment, KzgProof, KzgSettings, BYTES_PER_BLOB,
        BYTES_PER_COMMITMENT, BYTES_PER_PROOF,
    },
    kzg_to_versioned_hash, Address, Bytes, ChainId, Signature, TransactionKind, TxHash, TxType,
    H256,
};
use rlp::{length_of_length, Decodable, DecodeError, Encodable, Header};
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use sip_codecs::{main_codec, Compact};
use std::{mem, ops::Deref};

use serde_json::Value;

/// [EIP-4844 Blob Transaction](https://eips.ethereum.org/EIPS/eip-4844#blob-transaction)
///
/// A transaction with blob hashes and max blob fee
#[main_codec]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TxEip4844 {
    pub hash: TxHash,
    /// Added as EIP-pub 155: Simple replay attack protection

    #[serde(default, rename = "chainId", deserialize_with = "hex_to_u64")]
    pub chain_id: ChainId,
    /// A scalar value equal to the number of transactions sent by the sender; formally Tn.

    #[serde(default, deserialize_with = "hex_to_u64")]
    pub nonce: u64,

    pub from: Address,

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

    #[serde(deserialize_with = "default_if_empty")]
    pub to: TransactionKind,

    #[serde(default, rename = "v", deserialize_with = "hex_to_u128")]
    pub value: u128,

    #[serde(default, rename = "accessList")]
    pub access_list: AccessList,

    #[serde(default, rename = "blobVersionedHashes")]
    pub blob_versioned_hashes: Vec<H256>,

    #[serde(default, rename = "maxFeePerBlobGas", deserialize_with = "hex_to_u128")]
    pub max_fee_per_blob_gas: u128,

    #[serde(default, rename = "blockNumber", deserialize_with = "hex_to_u64")]
    pub block_number: u64,

    #[serde(default, rename = "transactionIndex", deserialize_with = "hex_to_u64")]
    pub transaction_index: u64,

    pub input: Bytes,
}

fn default_if_empty<'de, D, T>(de: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de> + Default,
{
    Option::<T>::deserialize(de).map(|x| x.unwrap_or_else(|| T::default()))
}

impl From<Value> for TxEip4844 {
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}

impl From<String> for TxEip4844 {
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

impl TxEip4844 {
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

    /// Verifies that the given blob data, commitments, and proofs are all valid for this
    /// transaction.
    ///
    /// Takes as input the [KzgSettings], which should contain the the parameters derived from the
    /// KZG trusted setup.
    ///
    /// This ensures that the blob transaction payload has the same number of blob data elements,
    /// commitments, and proofs. Each blob data element is verified against its commitment and
    /// proof.
    ///
    /// Returns [BlobTransactionValidationError::InvalidProof] if any blob KZG proof in the response
    /// fails to verify, or if the versioned hashes in the transaction do not match the actual
    /// commitment versioned hashes.
    pub fn validate_blob(
        &self,
        sidecar: &BlobTransactionSidecar,
        proof_settings: &KzgSettings,
    ) -> Result<(), BlobTransactionValidationError> {
        // Ensure the versioned hashes and commitments have the same length
        if self.blob_versioned_hashes.len() != sidecar.commitments.len() {
            return Err(kzg::Error::MismatchLength(format!(
                "There are {} versioned commitment hashes and {} commitments",
                self.blob_versioned_hashes.len(),
                sidecar.commitments.len()
            ))
            .into());
        }

        // zip and iterate, calculating versioned hashes
        for (versioned_hash, commitment) in self
            .blob_versioned_hashes
            .iter()
            .zip(sidecar.commitments.iter())
        {
            // convert to KzgCommitment
            let commitment = KzgCommitment::from(*commitment.deref());

            let calculated_versioned_hash = kzg_to_versioned_hash(commitment);
            if *versioned_hash != calculated_versioned_hash {
                return Err(BlobTransactionValidationError::InvalidProof);
            }
        }

        // Verify as a batch
        let res = KzgProof::verify_blob_kzg_proof_batch(
            sidecar.blobs.as_slice(),
            sidecar.commitments.as_slice(),
            sidecar.proofs.as_slice(),
            proof_settings,
        )
        .map_err(BlobTransactionValidationError::KZGError)?;

        if res {
            Ok(())
        } else {
            Err(BlobTransactionValidationError::InvalidProof)
        }
    }

    /// Returns the total gas for all blobs in this transaction.
    #[inline]
    pub fn blob_gas(&self) -> u64 {
        // SAFETY: we don't expect u64::MAX / DATA_GAS_PER_BLOB hashes in a single transaction
        self.blob_versioned_hashes.len() as u64 * DATA_GAS_PER_BLOB
    }

    pub fn decode_inner(buf: &mut &[u8]) -> Result<Self, DecodeError> {
        Ok(Self {
            hash: Decodable::decode(buf)?,
            chain_id: Decodable::decode(buf)?,
            nonce: Decodable::decode(buf)?,
            from: Decodable::decode(buf)?,
            gas_limit: Decodable::decode(buf)?,
            max_fee_per_gas: Decodable::decode(buf)?,
            max_priority_fee_per_gas: Decodable::decode(buf)?,
            to: Decodable::decode(buf)?,
            value: Decodable::decode(buf)?,
            access_list: Decodable::decode(buf)?,
            blob_versioned_hashes: Decodable::decode(buf)?,
            max_fee_per_blob_gas: Decodable::decode(buf)?,
            block_number: Decodable::decode(buf)?,
            transaction_index: Decodable::decode(buf)?,
            input: Bytes(Decodable::decode(buf)?),
        })
    }

    /// Outputs the length of the transaction's fields, without a RLP header.
    pub(crate) fn fields_len(&self) -> usize {
        let mut len = 0;
        len += self.hash.length();
        len += self.chain_id.length();
        len += self.nonce.length();
        len += self.from.length();
        len += self.gas_limit.length();
        len += self.max_fee_per_gas.length();
        len += self.max_priority_fee_per_gas.length();
        len += self.to.length();
        len += self.value.length();
        len += self.access_list.length();
        len += self.blob_versioned_hashes.length();
        len += self.max_fee_per_blob_gas.length();
        len += self.block_number.length();
        len += self.transaction_index.length();
        len += self.input.0.length();
        len
    }

    /// Encodes only the transaction's fields into the desired buffer, without a RLP header.
    pub(crate) fn encode_fields(&self, out: &mut dyn bytes::BufMut) {
        self.hash.encode(out);
        self.chain_id.encode(out);
        self.nonce.encode(out);
        self.from.encode(out);
        self.gas_limit.encode(out);
        self.max_fee_per_gas.encode(out);
        self.max_priority_fee_per_gas.encode(out);
        self.to.encode(out);
        self.value.encode(out);
        self.access_list.encode(out);
        self.blob_versioned_hashes.encode(out);
        self.max_fee_per_blob_gas.encode(out);
        self.block_number.encode(out);
        self.transaction_index.encode(out);
        self.input.0.encode(out);
    }

    /// Calculates a heuristic for the in-memory size of the [TxEip4844] transaction.
    #[inline]
    pub fn size(&self) -> usize {
        mem::size_of::<TxHash>() + // hash
        mem::size_of::<u64>() + // chain_id
        mem::size_of::<u64>() + // nonce
        mem::size_of::<Address>() + // from
        mem::size_of::<u64>() + // gas_limit
        mem::size_of::<u128>() + // max_fee_per_gas
        mem::size_of::<u128>() + // max_priority_fee_per_gas
        self.to.size() + // to
        mem::size_of::<u128>() + // value
        self.access_list.size() + // access_list
        self.input.len() +  // input
        self.blob_versioned_hashes.capacity() * mem::size_of::<H256>() + // blob hashes size
        mem::size_of::<u128>() + // max_fee_per_data_gas
        mem::size_of::<u64>() + // block_number
        mem::size_of::<u64>() // transaction_index
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
        TxType::EIP4844
    }
}

/// An error that can occur when validating a [BlobTransaction].
#[derive(Debug, thiserror::Error)]
pub enum BlobTransactionValidationError {
    /// Proof validation failed.
    #[error("invalid kzg proof")]
    InvalidProof,
    /// An error returned by the [kzg] library
    #[error("kzg error: {0:?}")]
    KZGError(kzg::Error),
    /// The inner transaction is not a blob transaction
    #[error("unable to verify proof for non blob transaction: {0}")]
    NotBlobTransaction(u8),
}

impl From<kzg::Error> for BlobTransactionValidationError {
    fn from(value: kzg::Error) -> Self {
        Self::KZGError(value)
    }
}

/// A response to `GetPooledTransactions` that includes blob data, their commitments, and their
/// corresponding proofs.
///
/// This is defined in [EIP-4844](https://eips.ethereum.org/EIPS/eip-4844#networking) as an element
/// of a `PooledTransactions` response.
///
/// NOTE: This contains a [TransactionSigned], which could be a non-4844 transaction type, even
/// though that would not make sense. This type is meant to be constructed using decoding methods,
/// which should always construct the [TransactionSigned] with an EIP-4844 transaction.
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BlobTransaction {
    /// The transaction hash.
    pub hash: TxHash,
    /// The transaction payload.
    pub transaction: TxEip4844,
    /// The transaction signature.
    pub signature: Signature,
    /// The transaction's blob sidecar.
    pub sidecar: BlobTransactionSidecar,
}

impl BlobTransaction {
    /// Verifies that the transaction's blob data, commitments, and proofs are all valid.
    ///
    /// See also [TxEip4844::validate_blob]
    pub fn validate(
        &self,
        proof_settings: &KzgSettings,
    ) -> Result<(), BlobTransactionValidationError> {
        self.transaction
            .validate_blob(&self.sidecar, proof_settings)
    }
}

/// This represents a set of blobs, and its corresponding commitments and proofs.
#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BlobTransactionSidecar {
    /// The blob data.
    pub blobs: Vec<Blob>,
    /// The blob commitments.
    pub commitments: Vec<Bytes48>,
    /// The blob proofs.
    pub proofs: Vec<Bytes48>,
}

impl BlobTransactionSidecar {
    /// Calculates a size heuristic for the in-memory size of the [BlobTransactionSidecar].
    #[inline]
    pub fn size(&self) -> usize {
        self.blobs.len() * BYTES_PER_BLOB + // blobs
        self.commitments.len() * BYTES_PER_COMMITMENT + // commitments
        self.proofs.len() * BYTES_PER_PROOF // proofs
    }
}
