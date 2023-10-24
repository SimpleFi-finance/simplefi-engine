use std::collections::HashMap;

use db::{
    common::PairResult,
    tables::{models::TxLogId, ContractLogs},
};
use interfaces::Result;
use primitives::{
    Address, BlockHashOrNumber, BlockNumber, Log, StoredDecodedData, StoredLog, TxHash, TxNumber,
};

use super::{AbiProvider, BlockNumReader, TrackingProvider};

pub enum StoredOrDecodedLog {
    Raw(StoredLog),
    Decoded(StoredDecodedData),
}

///  Client trait for fetching [TransactionSigned] related data.
#[auto_impl::auto_impl(&, Arc)]
pub trait LogsProvider: BlockNumReader + Send + Sync {
    /// Get log by id, computes hash everytime so more expensive.
    fn logs_by_tx_id(
        &self,
        tx_id: TxNumber,
        decoded: bool,
    ) -> Result<Option<Vec<StoredOrDecodedLog>>>;

    /// Get log by transaction hash.
    fn logs_by_tx_hash(
        &self,
        tx_hash: TxHash,
        decoded: bool,
    ) -> Result<Option<Vec<StoredOrDecodedLog>>>;

    /// Get logs by block id.
    fn logs_by_block(
        &self,
        block: BlockHashOrNumber,
        decoded: bool,
    ) -> Result<Option<Vec<StoredOrDecodedLog>>>;

    /// Get logs by block range.
    fn logs_by_block_range(
        &self,
        start: BlockNumber,
        end: BlockNumber,
        decoded: bool,
    ) -> Result<Vec<StoredOrDecodedLog>>;

    fn logs_by_address(
        &self,
        address: Address,
        from: Option<BlockNumber>,
        to: Option<BlockNumber>,
        decoded: bool,
    ) -> Result<Vec<StoredOrDecodedLog>>;

    fn get_address_logs_latest_partition(&self, address: Address) -> PairResult<ContractLogs>;
}

#[auto_impl::auto_impl(&, Arc, Box)]
pub trait LogsWriter: Send + Sync + TrackingProvider + AbiProvider {
    fn insert_raw_logs(&self, log: (TxLogId, StoredLog)) -> Result<()>;

    fn insert_decoded_data(&self, log: (TxLogId, StoredDecodedData)) -> Result<()>;

    fn decode_and_store_logs(&self, decoded_log: &HashMap<Address, Vec<TxLogId>>) -> Result<()>;

    fn insert_logs(
        &self,
        logs: Vec<(TxNumber, Vec<Log>)>,
    ) -> Result<HashMap<Address, Vec<TxLogId>>>;

    fn insert_logs_by_address(&self, logs: &HashMap<Address, Vec<TxLogId>>) -> Result<()>;
}
