use auto_impl::auto_impl;
use interfaces::Result;
use primitives::{BlockHashOrNumber, BlockNumber, H256};

/// Client trait for fetching block hashes by number.
#[auto_impl(&, Arc, Box)]
pub trait BlockHashReader: BlockHashWriter + Send + Sync {
    /// Get the hash of the block with the given number. Returns `None` if no block with this number
    /// exists.
    fn block_hash(&self, number: BlockNumber) -> Result<Option<H256>>;

    /// Get the hash of the block with the given number. Returns `None` if no block with this number
    /// exists.
    fn convert_block_hash(&self, hash_or_number: BlockHashOrNumber) -> Result<Option<H256>> {
        match hash_or_number {
            BlockHashOrNumber::Hash(hash) => Ok(Some(hash)),
            BlockHashOrNumber::Number(num) => self.block_hash(num),
        }
    }

    /// Get headers in range of block hashes or numbers
    fn block_hashes_range(&self, start: BlockNumber, end: BlockNumber) -> Result<Vec<H256>>;
}

#[auto_impl(&, Arc, Box)]
pub trait BlockHashWriter: Send + Sync {
    /// Set the hash of the block with the given number.
    fn insert_block_hash(&self, number: BlockNumber, hash: H256) -> Result<()>;
}