use auto_impl::auto_impl;
// use db::models::StoredBlockBodyIndices;
use interfaces::Result;
use simp_primitives::{
    Address, Block, BlockHashOrNumber, H256,
};

use super::{HeaderProvider, BlockNumReader, LogsProvider, TransactionsProvider, BlockBodyIndicesProvider, BlockBodyIndicesWriter, transactions::TransactionsWriter, logs::LogsWriter};

/// A helper enum that represents the origin of the requested block.
///
/// This helper type's sole purpose is to give the caller more control over from where blocks can be
/// fetched.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BlockSource {
    /// Check all available sources.
    ///
    /// Note: it's expected that looking up pending blocks is faster than looking up blocks in the
    /// database so this prioritizes Pending > Database.
    #[default]
    Any,
    /// The block was fetched from the pending block source, the blockchain tree that buffers
    /// blocks that are not yet finalized.
    Pending,
    /// The block was fetched from the database.
    Database,
}
#[allow(dead_code)]
impl BlockSource {
    /// Returns `true` if the block source is `Pending` or `Any`.
    pub fn is_pending(&self) -> bool {
        matches!(self, BlockSource::Pending | BlockSource::Any)
    }

    /// Returns `true` if the block source is `Database` or `Any`.
    pub fn is_database(&self) -> bool {
        matches!(self, BlockSource::Database | BlockSource::Any)
    }
}

/// Api trait for fetching `Block` related data.
///
/// If not requested otherwise, implementers of this trait should prioritize fetching blocks from
/// the database.
#[auto_impl::auto_impl(&, Arc)]
pub trait BlockReader:
    BlockNumReader
    + HeaderProvider
    + TransactionsProvider
    + LogsProvider
    + BlockBodyIndicesProvider
    + Send
    + Sync
{
    /// Returns the block with given id from the database.
    ///
    /// Returns `None` if block is not found.
    fn block(&self, id: BlockHashOrNumber) -> Result<Option<Block>>;

    /// Returns the block with matching hash from the database.
    ///
    /// Returns `None` if block is not found.
    fn block_by_hash(&self, hash: H256) -> Result<Option<Block>> {
        self.block(hash.into())
    }

    /// Returns the block with matching number from database.
    ///
    /// Returns `None` if block is not found.
    fn block_by_number(&self, num: u64) -> Result<Option<Block>> {
        self.block(num.into())
    }
}

/// Block Writer
#[auto_impl(&, Arc, Box)]
pub trait BlockWriter: 
    BlockBodyIndicesWriter
    + TransactionsWriter
    + LogsWriter
    + Send 
    + Sync 
{
    /// Insert full block and make it canonical. Parent tx num and transition id is taken from
    /// parent block in database.
    ///
    /// Return [StoredBlockBodyIndices] that contains indices of the first and last transactions and
    /// transition in the block.
    fn insert_block(
        &self,
        block: Block,
        senders: Option<Vec<Address>>,
    ) -> Result<()>;
}
