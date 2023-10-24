use crate::{
    Address, Bytes, TxHash, BlockNumber,
};
pub use access_list::{AccessList, AccessListItem, AccessListWithGasUsed};
use bytes::{Buf, BytesMut};
use derive_more::{AsRef, Deref};
pub use error::InvalidTransactionError;
pub use meta::TransactionMeta;
use sip_codecs::{derive_arbitrary, Compact, main_codec};
use rlp::{Decodable, DecodeError, Encodable, Header, EMPTY_LIST_CODE, EMPTY_STRING_CODE};
use serde::{Deserialize, Serialize};
pub use signature::Signature;
use std::mem;
pub use tx_type::{
    TxType, EIP1559_TX_TYPE_ID, EIP2930_TX_TYPE_ID, EIP4844_TX_TYPE_ID, LEGACY_TX_TYPE_ID,
};
use serde_json::Value;
pub use eip1559::TxEip1559;
pub use eip2930::TxEip2930;
pub use eip4844::{
    BlobTransaction, BlobTransactionSidecar, BlobTransactionValidationError, TxEip4844,
};
pub use legacy::TxLegacy;

mod access_list;
mod eip1559;
mod eip2930;
mod eip4844;
mod error;
mod legacy;
mod meta;
mod signature;
mod tx_type;
pub(crate) mod util;

/// A raw transaction.
///
/// Transaction types were introduced in [EIP-2718](https://eips.ethereum.org/EIPS/eip-2718).
#[main_codec]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Transaction {
    /// Legacy transaction (type `0x0`).
    ///
    /// Traditional Ethereum transactions, containing parameters `nonce`, `gasPrice`, `gasLimit`,
    /// `to`, `value`, `data`, `v`, `r`, and `s`.
    ///
    /// These transactions do not utilize access lists nor do they incorporate EIP-1559 fee market
    /// changes.
    Legacy(TxLegacy),
    /// Transaction with an [`AccessList`] ([EIP-2930](https://eips.ethereum.org/EIPS/eip-2930)), type `0x1`.
    ///
    /// The `accessList` specifies an array of addresses and storage keys that the transaction
    /// plans to access, enabling gas savings on cross-contract calls by pre-declaring the accessed
    /// contract and storage slots.
    Eip2930(TxEip2930),
    /// A transaction with a priority fee ([EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)), type `0x2`.
    ///
    /// Unlike traditional transactions, EIP-1559 transactions use an in-protocol, dynamically
    /// changing base fee per gas, adjusted at each block to manage network congestion.
    ///
    /// - `maxPriorityFeePerGas`, specifying the maximum fee above the base fee the sender is
    ///   willing to pay
    /// - `maxFeePerGas`, setting the maximum total fee the sender is willing to pay.
    ///
    /// The base fee is burned, while the priority fee is paid to the miner who includes the
    /// transaction, incentivizing miners to include transactions with higher priority fees per
    /// gas.
    Eip1559(TxEip1559),
    /// Shard Blob Transactions ([EIP-4844](https://eips.ethereum.org/EIPS/eip-4844)), type `0x3`.
    ///
    /// Shard Blob Transactions introduce a new transaction type called a blob-carrying transaction
    /// to reduce gas costs. These transactions are similar to regular Ethereum transactions but
    /// include additional data called a blob.
    ///
    /// Blobs are larger (~125 kB) and cheaper than the current calldata, providing an immutable
    /// and read-only memory for storing transaction data.
    ///
    /// EIP-4844, also known as proto-danksharding, implements the framework and logic of
    /// danksharding, introducing new transaction formats and verification rules.
    Eip4844(TxEip4844),
}

impl From<Value> for Transaction {
    fn from(value: Value) -> Self {

        if let Some(field) = value.get("type") {

            let tx_type = u64::from_str_radix(field.as_str().unwrap().strip_prefix("0x").unwrap(), 16).unwrap();

            match tx_type {
                0 => {
                    // Legacy  
                    let tx = serde_json::from_value(value).unwrap();
    
                    Transaction::Legacy(tx)
                },
                1 => {
                    // Eip2930

                    let tx: TxEip2930 = serde_json::from_value(value).unwrap();
                    Transaction::Eip2930(tx)
                },
                2 => {
                    //EIP 1559

                    let tx: TxEip1559 = serde_json::from_value(value).unwrap();
                    Transaction::Eip1559(tx)
                },
                3 => {
                    //EIP 4844
                    let tx: TxEip4844 = serde_json::from_value(value).unwrap();
                    Transaction::Eip4844(tx)
                },
                _ => {
                    panic!("unknown tx type: {}", tx_type)
                }
            }
        } else {
            let tx = serde_json::from_value(value).unwrap();
    
            Transaction::Legacy(tx)
        }
    }
}

impl From<String> for Transaction {
    fn from(value: String) -> Self {
        let value: Value = serde_json::from_str(&value).unwrap();
        Transaction::from(value)
    }
}

// === impl Transaction ===

impl Transaction {
    pub fn hash(&self) -> TxHash {
        match self {
            Transaction::Legacy(TxLegacy { hash, .. }) => *hash,
            Transaction::Eip2930(TxEip2930 { hash, .. }) => *hash,
            Transaction::Eip1559(TxEip1559 { hash, .. }) => *hash,
            Transaction::Eip4844(TxEip4844 { hash, .. }) => *hash,
        }
    }

    pub fn block_number(&self) -> BlockNumber {
        match self {
            Transaction::Legacy(TxLegacy { block_number, .. }) => *block_number,
            Transaction::Eip2930(TxEip2930 { block_number, .. }) => *block_number,
            Transaction::Eip1559(TxEip1559 { block_number, .. }) => *block_number,
            Transaction::Eip4844(TxEip4844 { block_number, .. }) => *block_number,
        }
    }

    pub fn transaction_index(&self) -> u64 {
        match self {
            Transaction::Legacy(TxLegacy { transaction_index, .. }) => *transaction_index,
            Transaction::Eip2930(TxEip2930 { transaction_index, .. }) => *transaction_index,
            Transaction::Eip1559(TxEip1559 { transaction_index, .. }) => *transaction_index,
            Transaction::Eip4844(TxEip4844 { transaction_index, .. }) => *transaction_index,
        }
    }

    /// Get chain_id.
    pub fn chain_id(&self) -> Option<u64> {
        match self {
            Transaction::Legacy(TxLegacy { chain_id, .. }) => Some(*chain_id),
            Transaction::Eip2930(TxEip2930 { chain_id, .. }) => Some(*chain_id),
            Transaction::Eip1559(TxEip1559 { chain_id, .. }) => Some(*chain_id),
            Transaction::Eip4844(TxEip4844 { chain_id, .. }) => Some(*chain_id),
        }
    }

    /// Sets the transaction's chain id to the provided value.
    pub fn set_chain_id(&mut self, chain_id: u64) {
        match self {
            Transaction::Legacy(TxLegacy { chain_id: ref mut c, .. }) => *c = chain_id,
            Transaction::Eip2930(TxEip2930 { chain_id: ref mut c, .. }) => *c = chain_id,
            Transaction::Eip1559(TxEip1559 { chain_id: ref mut c, .. }) => *c = chain_id,
            Transaction::Eip4844(TxEip4844 { chain_id: ref mut c, .. }) => *c = chain_id,
        }
    }

    /// Gets the transaction's [`TransactionKind`], which is the address of the recipient or
    /// [`TransactionKind::Create`] if the transaction is a contract creation.
    pub fn kind(&self) -> &TransactionKind {
        match self {
            Transaction::Legacy(TxLegacy { to, .. }) |
            Transaction::Eip2930(TxEip2930 { to, .. }) |
            Transaction::Eip1559(TxEip1559 { to, .. }) |
            Transaction::Eip4844(TxEip4844 { to, .. }) => to,
        }
    }

    /// Get the transaction's nonce.
    pub fn to(&self) -> Option<Address> {
        self.kind().to()
    }

    /// Get transaction type
    pub fn tx_type(&self) -> TxType {
        match self {
            Transaction::Legacy(legacy_tx) => legacy_tx.tx_type(),
            Transaction::Eip2930(access_list_tx) => access_list_tx.tx_type(),
            Transaction::Eip1559(dynamic_fee_tx) => dynamic_fee_tx.tx_type(),
            Transaction::Eip4844(blob_tx) => blob_tx.tx_type(),
        }
    }

    /// Gets the transaction's value field.
    pub fn value(&self) -> u128 {
        *match self {
            Transaction::Legacy(TxLegacy { value, .. }) => value,
            Transaction::Eip2930(TxEip2930 { value, .. }) => value,
            Transaction::Eip1559(TxEip1559 { value, .. }) => value,
            Transaction::Eip4844(TxEip4844 { value, .. }) => value,
        }
    }

    /// Get the transaction's nonce.
    pub fn nonce(&self) -> u64 {
        match self {
            Transaction::Legacy(TxLegacy { nonce, .. }) => *nonce,
            Transaction::Eip2930(TxEip2930 { nonce, .. }) => *nonce,
            Transaction::Eip1559(TxEip1559 { nonce, .. }) => *nonce,
            Transaction::Eip4844(TxEip4844 { nonce, .. }) => *nonce,
        }
    }

    /// Get the gas limit of the transaction.
    pub fn gas_limit(&self) -> u64 {
        match self {
            Transaction::Legacy(TxLegacy { gas_limit, .. }) |
            Transaction::Eip2930(TxEip2930 { gas_limit, .. }) |
            Transaction::Eip1559(TxEip1559 { gas_limit, .. }) |
            Transaction::Eip4844(TxEip4844 { gas_limit, .. }) => *gas_limit,
        }
    }

    /// Returns true if the tx supports dynamic fees
    pub fn is_dynamic_fee(&self) -> bool {
        match self {
            Transaction::Legacy(_) | Transaction::Eip2930(_) => false,
            Transaction::Eip1559(_) | Transaction::Eip4844(_) => true,
        }
    }

    /// Max fee per gas for eip1559 transaction, for legacy transactions this is gas_price.
    ///
    /// This is also commonly referred to as the "Gas Fee Cap" (`GasFeeCap`).
    pub fn max_fee_per_gas(&self) -> u128 {
        match self {
            Transaction::Legacy(TxLegacy { gas_price, .. }) |
            Transaction::Eip2930(TxEip2930 { gas_price, .. }) => *gas_price,
            Transaction::Eip1559(TxEip1559 { max_fee_per_gas, .. }) |
            Transaction::Eip4844(TxEip4844 { max_fee_per_gas, .. }) => *max_fee_per_gas,
        }
    }

    /// Max priority fee per gas for eip1559 transaction, for legacy and eip2930 transactions this
    /// is `None`
    ///
    /// This is also commonly referred to as the "Gas Tip Cap" (`GasTipCap`).
    pub fn max_priority_fee_per_gas(&self) -> Option<u128> {
        match self {
            Transaction::Legacy(_) => None,
            Transaction::Eip2930(_) => None,
            Transaction::Eip1559(TxEip1559 { max_priority_fee_per_gas, .. }) |
            Transaction::Eip4844(TxEip4844 { max_priority_fee_per_gas, .. }) => {
                Some(*max_priority_fee_per_gas)
            }
        }
    }

    /// Max fee per blob gas for eip4844 transaction [TxEip4844].
    ///
    /// Returns `None` for non-eip4844 transactions.
    ///
    /// This is also commonly referred to as the "Blob Gas Fee Cap" (`BlobGasFeeCap`).
    pub fn max_fee_per_blob_gas(&self) -> Option<u128> {
        match self {
            Transaction::Eip4844(TxEip4844 { max_fee_per_blob_gas, .. }) => {
                Some(*max_fee_per_blob_gas)
            }
            _ => None,
        }
    }

    /// Return the max priority fee per gas if the transaction is an EIP-1559 transaction, and
    /// otherwise return the gas price.
    ///
    /// # Warning
    ///
    /// This is different than the `max_priority_fee_per_gas` method, which returns `None` for
    /// non-EIP-1559 transactions.
    pub fn priority_fee_or_price(&self) -> u128 {
        match self {
            Transaction::Legacy(TxLegacy { gas_price, .. }) |
            Transaction::Eip2930(TxEip2930 { gas_price, .. }) => *gas_price,
            Transaction::Eip1559(TxEip1559 { max_priority_fee_per_gas, .. }) |
            Transaction::Eip4844(TxEip4844 { max_priority_fee_per_gas, .. }) => {
                *max_priority_fee_per_gas
            }
        }
    }

    /// Returns the effective gas price for the given base fee.
    ///
    /// If the transaction is a legacy or EIP2930 transaction, the gas price is returned.
    pub fn effective_gas_price(&self, base_fee: Option<u64>) -> u128 {
        match self {
            Transaction::Legacy(tx) => tx.gas_price,
            Transaction::Eip2930(tx) => tx.gas_price,
            Transaction::Eip1559(dynamic_tx) => dynamic_tx.effective_gas_price(base_fee),
            Transaction::Eip4844(dynamic_tx) => dynamic_tx.effective_gas_price(base_fee),
        }
    }

    /// Determine the effective gas limit for the given transaction and base fee.
    /// If the base fee is `None`, the `max_priority_fee_per_gas`, or gas price for non-EIP1559
    /// transactions is returned.
    ///
    /// If the `max_fee_per_gas` is less than the base fee, `None` returned.
    pub fn effective_gas_tip(&self, base_fee: Option<u64>) -> Option<u128> {
        if let Some(base_fee) = base_fee {
            let max_fee_per_gas = self.max_fee_per_gas();

            if max_fee_per_gas < base_fee as u128 {
                None
            } else {
                let effective_max_fee = max_fee_per_gas - base_fee as u128;
                Some(std::cmp::min(effective_max_fee, self.priority_fee_or_price()))
            }
        } else {
            Some(self.priority_fee_or_price())
        }
    }

    /// Returns the effective miner gas tip cap (`gasTipCap`) for the given base fee:
    /// `min(maxFeePerGas - baseFee, maxPriorityFeePerGas)`
    ///
    /// Returns `None` if the basefee is higher than the [Transaction::max_fee_per_gas].
    pub fn effective_tip_per_gas(&self, base_fee: u64) -> Option<u128> {
        let base_fee = base_fee as u128;
        let max_fee_per_gas = self.max_fee_per_gas();

        if max_fee_per_gas < base_fee {
            return None
        }

        // the miner tip is the difference between the max fee and the base fee or the
        // max_priority_fee_per_gas, whatever is lower

        // SAFETY: max_fee_per_gas >= base_fee
        let fee = max_fee_per_gas - base_fee;

        if let Some(priority_fee) = self.max_priority_fee_per_gas() {
            return Some(fee.min(priority_fee))
        }

        Some(fee)
    }

    /// Get the transaction's input field.
    pub fn input(&self) -> &Bytes {
        match self {
            Transaction::Legacy(TxLegacy { input, .. }) => input,
            Transaction::Eip2930(TxEip2930 { input, .. }) => input,
            Transaction::Eip1559(TxEip1559 { input, .. }) => input,
            Transaction::Eip4844(TxEip4844 { input, .. }) => input,
        }
    }

    /// Inner encoding function that is used for both rlp [`Encodable`] trait and for calculating
    /// hash that for eip2718 does not require rlp header
    pub fn encode_with_signature(
        &self,
        signature: &Signature,
        out: &mut dyn bytes::BufMut,
        with_header: bool,
    ) {
        match self {
            Transaction::Legacy(legacy_tx) => {
                // do nothing w/ with_header
                legacy_tx.encode_with_signature(signature, out)
            }
            Transaction::Eip2930(access_list_tx) => {
                access_list_tx.encode_with_signature(signature, out, with_header)
            }
            Transaction::Eip1559(dynamic_fee_tx) => {
                dynamic_fee_tx.encode_with_signature(signature, out, with_header)
            }
            Transaction::Eip4844(blob_tx) => {
                blob_tx.encode_with_signature(signature, out, with_header)
            }
        }
    }

    /// This sets the transaction's nonce.
    pub fn set_nonce(&mut self, nonce: u64) {
        match self {
            Transaction::Legacy(tx) => tx.nonce = nonce,
            Transaction::Eip2930(tx) => tx.nonce = nonce,
            Transaction::Eip1559(tx) => tx.nonce = nonce,
            Transaction::Eip4844(tx) => tx.nonce = nonce,
        }
    }

    /// This sets the transaction's value.
    pub fn set_value(&mut self, value: u128) {
        match self {
            Transaction::Legacy(tx) => tx.value = value,
            Transaction::Eip2930(tx) => tx.value = value,
            Transaction::Eip1559(tx) => tx.value = value,
            Transaction::Eip4844(tx) => tx.value = value,
        }
    }

    /// This sets the transaction's input field.
    pub fn set_input(&mut self, input: Bytes) {
        match self {
            Transaction::Legacy(tx) => tx.input = input,
            Transaction::Eip2930(tx) => tx.input = input,
            Transaction::Eip1559(tx) => tx.input = input,
            Transaction::Eip4844(tx) => tx.input = input,
        }
    }

    /// Calculates a heuristic for the in-memory size of the [Transaction].
    #[inline]
    fn size(&self) -> usize {
        match self {
            Transaction::Legacy(tx) => tx.size(),
            Transaction::Eip2930(tx) => tx.size(),
            Transaction::Eip1559(tx) => tx.size(),
            Transaction::Eip4844(tx) => tx.size(),
        }
    }

    /// Returns true if the transaction is a legacy transaction.
    #[inline]
    pub fn is_legacy(&self) -> bool {
        matches!(self, Transaction::Legacy(_))
    }

    /// Returns true if the transaction is an EIP-2930 transaction.
    #[inline]
    pub fn is_eip2930(&self) -> bool {
        matches!(self, Transaction::Eip2930(_))
    }

    /// Returns true if the transaction is an EIP-1559 transaction.
    #[inline]
    pub fn is_eip1559(&self) -> bool {
        matches!(self, Transaction::Eip1559(_))
    }

    /// Returns true if the transaction is an EIP-4844 transaction.
    #[inline]
    pub fn is_eip4844(&self) -> bool {
        matches!(self, Transaction::Eip4844(_))
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::Legacy(TxLegacy::default())
    }
}

/// This encodes the transaction _without_ the signature, and is only suitable for creating a hash
/// intended for signing.

/// Whether or not the transaction is a contract creation.
#[derive_arbitrary(compact, rlp)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionKind {
    /// A transaction that creates a contract.
    #[default]
    Create,
    /// A transaction that calls a contract or transfer.
    Call(Address),
}

impl TransactionKind {
    /// Returns the address of the contract that will be called or will receive the transfer.
    pub fn to(self) -> Option<Address> {
        match self {
            TransactionKind::Create => None,
            TransactionKind::Call(to) => Some(to),
        }
    }

    /// Calculates a heuristic for the in-memory size of the [TransactionKind].
    #[inline]
    fn size(self) -> usize {
        mem::size_of::<Self>()
    }
}

impl Compact for TransactionKind {
    fn to_compact<B>(self, buf: &mut B) -> usize
    where
        B: bytes::BufMut + AsMut<[u8]>,
    {
        match self {
            TransactionKind::Create => 0,
            TransactionKind::Call(address) => {
                address.to_compact(buf);
                1
            }
        }
    }

    fn from_compact(buf: &[u8], identifier: usize) -> (Self, &[u8]) {
        match identifier {
            0 => (TransactionKind::Create, buf),
            1 => {
                let (addr, buf) = Address::from_compact(buf, buf.len());
                (TransactionKind::Call(addr), buf)
            }
            _ => unreachable!("Junk data in database: unknown TransactionKind variant"),
        }
    }
}

impl Encodable for TransactionKind {
    fn encode(&self, out: &mut dyn rlp::BufMut) {
        match self {
            TransactionKind::Call(to) => to.encode(out),
            TransactionKind::Create => out.put_u8(EMPTY_STRING_CODE),
        }
    }
    fn length(&self) -> usize {
        match self {
            TransactionKind::Call(to) => to.length(),
            TransactionKind::Create => 1, // EMPTY_STRING_CODE is a single byte
        }
    }
}

impl Decodable for TransactionKind {
    fn decode(buf: &mut &[u8]) -> Result<Self, DecodeError> {
        if let Some(&first) = buf.first() {
            if first == EMPTY_STRING_CODE {
                buf.advance(1);
                Ok(TransactionKind::Create)
            } else {
                let addr = <Address as Decodable>::decode(buf)?;
                Ok(TransactionKind::Call(addr))
            }
        } else {
            Err(DecodeError::InputTooShort)
        }
    }
}

/// Signed transaction.
#[main_codec]
#[derive(Debug, Clone, PartialEq, Eq, Hash, AsRef, Deref, Default)]
pub struct TransactionSigned {
    /// Transaction hash
    pub hash: TxHash,
    /// The transaction signature values
    pub signature: Signature,
    /// Raw transaction info
    #[deref]
    #[as_ref]
    pub transaction: Transaction,
}


impl From<Value> for TransactionSigned {
    fn from(value: Value) -> Self {
        let tx_value: Transaction = value.clone().into();
        let hash = tx_value.hash();
        let signature = Signature::from(value);
        TransactionSigned { hash, signature, transaction: tx_value }
    }
}

impl AsRef<Self> for TransactionSigned {
    fn as_ref(&self) -> &Self {
        self
    }
}

// === impl TransactionSigned ===

impl TransactionSigned {
    /// Transaction signature.
    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    /// Transaction hash. Used to identify transaction.
    pub fn hash(&self) -> TxHash {
        self.hash
    }

    /// Reference to transaction hash. Used to identify transaction.
    pub fn hash_ref(&self) -> &TxHash {
        &self.hash
    }

    /// Returns the enveloped encoded transactions.
    ///
    /// See also [TransactionSigned::encode_enveloped]
    pub fn envelope_encoded(&self) -> bytes::Bytes {
        let mut buf = BytesMut::new();
        self.encode_enveloped(&mut buf);
        buf.freeze()
    }

    /// Encodes the transaction into the "raw" format (e.g. `eth_sendRawTransaction`).
    /// This format is also referred to as "binary" encoding.
    ///
    /// For legacy transactions, it encodes the RLP of the transaction into the buffer: `rlp(tx)`
    /// For EIP-2718 typed it encodes the type of the transaction followed by the rlp of the
    /// transaction: `type` + `rlp(tx)`
    pub fn encode_enveloped(&self, out: &mut dyn bytes::BufMut) {
        self.encode_inner(out, false)
    }

    /// Inner encoding function that is used for both rlp [`Encodable`] trait and for calculating
    /// hash that for eip2718 does not require rlp header
    pub(crate) fn encode_inner(&self, out: &mut dyn bytes::BufMut, with_header: bool) {
        self.transaction.encode_with_signature(&self.signature, out, with_header);
    }

    /// Output the length of the encode_inner(out, true). Note to assume that `with_header` is only
    /// `true`.
    pub(crate) fn payload_len_inner(&self) -> usize {
        match &self.transaction {
            Transaction::Legacy(legacy_tx) => legacy_tx.payload_len_with_signature(&self.signature),
            Transaction::Eip2930(access_list_tx) => {
                access_list_tx.payload_len_with_signature(&self.signature)
            }
            Transaction::Eip1559(dynamic_fee_tx) => {
                dynamic_fee_tx.payload_len_with_signature(&self.signature)
            }
            Transaction::Eip4844(blob_tx) => blob_tx.payload_len_with_signature(&self.signature),
        }
    }

    /// Calculate a heuristic for the in-memory size of the [TransactionSigned].
    #[inline]
    pub fn size(&self) -> usize {
        mem::size_of::<TxHash>() + self.transaction.size() + self.signature.size()
    }

    /// Decodes legacy transaction from the data buffer into a tuple.
    ///
    /// This expects `rlp(legacy_tx)`
    // TODO: make buf advancement semantics consistent with `decode_enveloped_typed_transaction`,
    // so decoding methods do not need to manually advance the buffer
    pub(crate) fn decode_rlp_legacy_transaction_tuple(
        data: &mut &[u8],
    ) -> Result<(TxLegacy, TxHash, Signature), DecodeError> {
        #[allow(unused_variables)]
        let header = Header::decode(data)?;

        let mut transaction = TxLegacy {
            hash: Decodable::decode(data)?,
            chain_id: Decodable::decode(data)?,
            from: Decodable::decode(data)?,
            nonce: Decodable::decode(data)?,
            gas_price: Decodable::decode(data)?,
            gas_limit: Decodable::decode(data)?,
            to: Decodable::decode(data)?,
            value: Decodable::decode(data)?,
            block_number: Decodable::decode(data)?,
            transaction_index: Decodable::decode(data)?,
            input: Bytes(Decodable::decode(data)?),
        };

        let (signature, extracted_id) = Signature::decode_with_eip155_chain_id(data)?;
        transaction.chain_id = extracted_id.unwrap_or(0);

        let hash = transaction.hash;

        Ok((transaction, hash, signature))
    }

    pub fn decode_rlp_legacy_transaction(
        data: &mut &[u8],
    ) -> Result<TransactionSigned, DecodeError> {
        let (transaction, hash, signature) =
            TransactionSigned::decode_rlp_legacy_transaction_tuple(data)?;
        let signed =
            TransactionSigned { transaction: Transaction::Legacy(transaction), hash, signature };
        Ok(signed)
    }

    /// Decodes en enveloped EIP-2718 typed transaction.
    ///
    /// CAUTION: this expects that `data` is `[id, rlp(tx)]`
    pub fn decode_enveloped_typed_transaction(
        data: &mut &[u8],
    ) -> Result<TransactionSigned, DecodeError> {
        let tx_type = *data.first().ok_or(DecodeError::InputTooShort)?;
        data.advance(1);

        // decode the list header for the rest of the transaction
        let header = Header::decode(data)?;
        if !header.list {
            return Err(DecodeError::Custom("typed tx fields must be encoded as a list"))
        }
        // decode common fields
        let transaction = match tx_type {
            1 => Transaction::Eip2930(TxEip2930::decode_inner(data)?),
            2 => Transaction::Eip1559(TxEip1559::decode_inner(data)?),
            3 => Transaction::Eip4844(TxEip4844::decode_inner(data)?),
            _ => return Err(DecodeError::Custom("unsupported typed transaction type")),
        };

        let signature = Signature::decode(data)?;

        let hash = transaction.hash();
        let signed = TransactionSigned { transaction, hash, signature };
        Ok(signed)
    }

    /// Decodes the "raw" format of transaction (e.g. `eth_sendRawTransaction`).
    ///
    /// The raw transaction is either a legacy transaction or EIP-2718 typed transaction
    /// For legacy transactions, the format is encoded as: `rlp(tx)`
    /// For EIP-2718 typed transaction, the format is encoded as the type of the transaction
    /// followed by the rlp of the transaction: `type` + `rlp(tx)`
    pub fn decode_enveloped(tx: Bytes) -> Result<Self, DecodeError> {
        let mut data = tx.as_ref();

        if data.is_empty() {
            return Err(DecodeError::InputTooShort)
        }

        // Check if the tx is a list
        if data[0] >= EMPTY_LIST_CODE {
            // decode as legacy transaction
            TransactionSigned::decode_rlp_legacy_transaction(&mut data)
        } else {
            TransactionSigned::decode_enveloped_typed_transaction(&mut data)
        }
    }
}


impl Encodable for TransactionSigned {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        self.encode_inner(out, true);
    }

    fn length(&self) -> usize {
        self.payload_len_inner()
    }
}

impl Decodable for TransactionSigned {
    fn decode(buf: &mut &[u8]) -> Result<Self, DecodeError> {
        // decode header
        let mut original_encoding = *buf;
        let header = Header::decode(buf)?;
        // if the transaction is encoded as a string then it is a typed transaction
        if !header.list {
            TransactionSigned::decode_enveloped_typed_transaction(buf)
        } else {
            let tx = TransactionSigned::decode_rlp_legacy_transaction(&mut original_encoding)?;

            // advance the buffer based on how far `decode_rlp_legacy_transaction` advanced the
            // buffer
            *buf = original_encoding;
            Ok(tx)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{transaction::TransactionSigned, TxLegacy, Signature, TxHash};
    use bytes::BytesMut;
    use rlp::{Decodable, DecodeError, Encodable};

    #[test]
    fn test_decode_empty_typed_tx() {
        let input = [0x80u8];
        let res = TransactionSigned::decode(&mut &input[..]).unwrap_err();
        assert_eq!(DecodeError::InputTooShort, res);
    }

    #[test]
    fn encode_decode_tx_legacy() {

        let tx = TxLegacy::default();
        let signature = Signature::default();
        let hash = TxHash::default();

        let tx_signed = TransactionSigned { transaction: crate::Transaction::Legacy(tx), signature, hash };

        let mut buf = BytesMut::new();
        tx_signed.encode(&mut buf);
        let decoded = TransactionSigned::decode(&mut &buf[..]).unwrap();

        assert_eq!(tx_signed, decoded);

    }
    #[test]
    fn encode_decode_eip1559() {
        let tx = crate::TxEip1559::default();
        let signature = Signature::default();
        let hash = TxHash::default();

        let tx_signed = TransactionSigned { transaction: crate::Transaction::Eip1559(tx), signature, hash };

        let mut buf = BytesMut::new();
        tx_signed.encode(&mut buf);
        let decoded = TransactionSigned::decode(&mut &buf[..]).unwrap();

        assert_eq!(tx_signed, decoded);
    }
    #[test]
    fn encode_decode_eip2930() {
        let tx = crate::TxEip2930::default();
        let signature = Signature::default();
        let hash = TxHash::default();

        let tx_signed = TransactionSigned { transaction: crate::Transaction::Eip2930(tx), signature, hash };

        let mut buf = BytesMut::new();
        tx_signed.encode(&mut buf);
        let decoded = TransactionSigned::decode(&mut &buf[..]).unwrap();

        assert_eq!(tx_signed, decoded);
    }

    #[test]
    fn encode_decode_eip4844() {
        let tx = crate::TxEip4844::default();
        let signature = Signature::default();
        let hash = TxHash::default();
        let tx_signed = TransactionSigned { transaction: crate::Transaction::Eip4844(tx), signature, hash };

        let mut buf = BytesMut::new();
        tx_signed.encode(&mut buf);
        let decoded = TransactionSigned::decode(&mut &buf[..]).unwrap();

        assert_eq!(tx_signed, decoded);
    }
}
