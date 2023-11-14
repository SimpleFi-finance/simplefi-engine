
use db::tables::TxIndices;
use interfaces::Result;
use simp_primitives::{
    BlockHashOrNumber, BlockNumber, TxHash, TxNumber, TransactionSigned,
};

use super::BlockNumReader;

///  Client trait for fetching [TransactionSigned] related data.
#[auto_impl::auto_impl(&, Arc)]
pub trait TransactionsProvider: BlockNumReader + Send + Sync {
    /// Get internal transaction identifier by transaction hash.
    ///
    /// This is the inverse of [TransactionsProvider::transaction_by_id].
    /// Returns None if the transaction is not found.
    fn transaction_id(&self, tx_hash: TxHash) -> Result<Option<TxNumber>>;

    fn transaction_by_id(&self, id: TxNumber) -> Result<Option<TransactionSigned>>;

    /// Get transaction by transaction hash.
    fn transaction_by_hash(&self, hash: TxHash) -> Result<Option<TransactionSigned>>;

    /// Get transaction block number
    fn transaction_block(&self, id: TxNumber) -> Result<Option<BlockNumber>>;

    /// Get transactions by block id.
    fn transactions_by_block(
        &self,
        block: BlockHashOrNumber,
    ) -> Result<Option<Vec<TransactionSigned>>>;

    /// Get transactions by block range.
    fn transactions_by_block_range(
        &self,
        start: BlockNumber,
        end: BlockNumber,
    ) -> Result<Vec<Vec<TransactionSigned>>>;

    /// Get transactions by tx range.
    fn transactions_by_tx_range(
        &self,
        start: simp_primitives::TxNumber,
        end: simp_primitives::TxNumber,
    ) -> Result<Vec<TransactionSigned>>;
}

#[auto_impl::auto_impl(&, Arc, Box)]
pub trait TransactionsWriter: Send + Sync {
    fn insert_transactions(
        &self,
        transactions: Vec<TransactionSigned>,
    ) -> Result<(TxIndices, Vec<(TxNumber, TxHash)>)>;
}