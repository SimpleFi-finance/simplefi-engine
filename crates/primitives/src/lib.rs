pub use ethers_core::{
    types::{BigEndianHash, H128, H64, U64},
    utils as rpc_utils,
};
pub use revm_primitives::{B160 as H160, B256 as H256, U256};

mod chain;
pub use chain::{
    Chain, ChainSpec, ChainSpecBuilder, DEV, GOERLI, MAINNET, SEPOLIA, ComputationEngine, ChainRpcProvider
};

mod stage;
pub use stage::StageId;

mod block;
pub use block::{Block, BlockHashOrNumber, BlockNumHash, BlockNumberOrTag, ForkBlock};

mod header;
pub use header::{Head, Header, HeadersDirection};


mod integer_list;
pub use integer_list::IntegerList;

/// Helpers for working with serde
pub mod serde_helper;
pub use serde_helper::JsonU256;

pub mod hex_bytes;
pub use hex_bytes::Bytes;

mod bits;
pub use bits::H512;

mod log;
pub use log::{Log, StoredDecodedData, StoredLog, DecodedData};

pub mod constants;

mod storage;
pub use storage::StorageEntry;

/// EIP-4844 + KZG helpers
pub mod kzg {
    pub use c_kzg::*;
}

mod compression;
pub use compression::*;

pub mod eip4844;
pub use eip4844::{calculate_excess_blob_gas, kzg_to_versioned_hash};

mod transaction;
pub mod volumetric;
pub mod protocol;
pub mod market;

pub use volumetric::*;
pub use market::*;
pub use protocol::*;

pub use transaction::{
    util::secp256k1::{public_key_to_address, recover_signer, sign_message},
    AccessList, AccessListItem, AccessListWithGasUsed, BlobTransaction, BlobTransactionSidecar,
    BlobTransactionValidationError,
    InvalidTransactionError,
    TransactionSigned,
    Signature, Transaction, TransactionKind, TransactionMeta,
    TxEip1559, TxEip2930,
    TxEip4844, TxLegacy, TxType, EIP1559_TX_TYPE_ID, EIP2930_TX_TYPE_ID, EIP4844_TX_TYPE_ID,
    LEGACY_TX_TYPE_ID,
};

/// A block hash.
pub type BlockHash = H256;
/// A block number
pub type BlockNumber = u64;
/// An Ethereum address.
pub type Address = H160;
/// A transaction hash is a kecack hash of an RLP encoded signed transaction.
pub type TxHash = H256;
/// The sequence number of all existing transactions.
pub type TxNumber = u64;
/// The sequence number of all existing Receipts.
pub type LogNumber = u64;
/// The index of transaction in a block.
pub type TxIndex = u64;
/// Chain identifier type (introduced in EIP-155).
pub type ChainId = u64;
/// An account storage key.
pub type StorageKey = H256;
/// An account storage value.
pub type StorageValue = U256;
/// Solidity contract functions are addressed using the first four byte of the Keccak-256 hash of
/// their signature
pub type Selector = [u8; 4];

pub type VolumeKey = u64;

pub type MarketPeriod = String;

pub type MarketAddress = H256;

pub use ruint::{
    aliases::{U128, U8},
    UintTryTo,
};

/// Returns the keccak256 hash for the given data.
#[inline]
pub fn keccak256(data: impl AsRef<[u8]>) -> H256 {
    use tiny_keccak::{Hasher, Keccak};

    let mut buf = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(data.as_ref());
    hasher.finalize(&mut buf);
    buf.into()
}
