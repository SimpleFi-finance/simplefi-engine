use super::BlockHashReader;
use interfaces::Result;
use primitives::{BlockHashOrNumber, BlockNumber, H256};

/// Client trait for getting important block numbers (such as the latest block number), converting
/// block hashes to numbers, and fetching a block hash from its block number.
///
/// This trait also supports fetching block hashes and block numbers from a [BlockHashOrNumber].
#[auto_impl::auto_impl(&, Arc)]
pub trait BlockNumReader: BlockHashReader + BlockNumWriter + Send + Sync {
    /// Returns the last block number associated with the last canonical header in the database.
    fn last_block_number(&self) -> Result<BlockNumber>;

    /// Gets the `BlockNumber` for the given hash. Returns `None` if no block with this hash exists.
    fn block_number(&self, hash: H256) -> Result<Option<BlockNumber>>;

    /// Gets the block number for the given `BlockHashOrNumber`. Returns `None` if no block with
    /// this hash exists. If the `BlockHashOrNumber` is a `Number`, it is returned as is.
    fn convert_hash_or_number(&self, id: BlockHashOrNumber) -> Result<Option<BlockNumber>> {
        match id {
            BlockHashOrNumber::Hash(hash) => self.block_number(hash),
            BlockHashOrNumber::Number(num) => Ok(Some(num)),
        }
    }

    /// Gets the block hash for the given `BlockHashOrNumber`. Returns `None` if no block with this
    /// number exists. If the `BlockHashOrNumber` is a `Hash`, it is returned as is.
    fn convert_number(&self, id: BlockHashOrNumber) -> Result<Option<H256>> {
        match id {
            BlockHashOrNumber::Hash(hash) => Ok(Some(hash)),
            BlockHashOrNumber::Number(num) => self.block_hash(num),
        }
    }
}

#[auto_impl::auto_impl(&, Arc)]
pub trait BlockNumWriter: Send + Sync {
    /// Inserts the given block number for the given hash.
    fn insert_block_number(&self, hash: H256, number: BlockNumber) -> Result<()>;
}