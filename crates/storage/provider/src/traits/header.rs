use auto_impl::auto_impl;
use interfaces::Result;
use primitives::{BlockHash, BlockHashOrNumber, BlockNumber, Header};

/// Client trait for fetching `Header` related data.
#[auto_impl(&, Arc)]
pub trait HeaderProvider: HeaderWriter + Send + Sync {
    /// Check if block is known
    fn is_known(&self, block_hash: &BlockHash) -> Result<bool> {
        self.header(block_hash).map(|header| header.is_some())
    }

    /// Get header by block hash
    fn header(&self, block_hash: &BlockHash) -> Result<Option<Header>>;

    /// Get header by block number
    fn header_by_number(&self, num: u64) -> Result<Option<Header>>;

    /// Get header by block number or hash
    fn header_by_hash_or_number(&self, hash_or_num: BlockHashOrNumber) -> Result<Option<Header>> {
        match hash_or_num {
            BlockHashOrNumber::Hash(hash) => self.header(&hash),
            BlockHashOrNumber::Number(num) => self.header_by_number(num),
        }
    }

    /// Get headers in range of block numbers
    fn headers_range(&self, range: (BlockNumber, BlockNumber)) -> Result<Vec<Header>>;

    fn latest_header(&self) -> Result<Option<Header>>;
}

#[auto_impl(&, Arc)]
pub trait HeaderWriter: Send + Sync {
    fn insert_header(&self,block_number: &BlockNumber, header: Header) -> Result<Option<BlockNumber>>;
}