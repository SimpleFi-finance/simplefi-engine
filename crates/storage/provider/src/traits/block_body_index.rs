use db::tables::BlockBodyIndices;
use primitives::{BlockHashOrNumber, BlockNumber};

use interfaces::Result;
use auto_impl::auto_impl;

#[auto_impl(&, Arc, Box)]
pub trait BlockBodyIndicesProvider: Send + Sync {
    fn block_body_indices(&self, block: BlockHashOrNumber) -> Result<Option<BlockBodyIndices>>;
}

#[auto_impl(&, Arc, Box)]
pub trait BlockBodyIndicesWriter: Send + Sync {
    fn insert_block_body_indices(&self, block_number: BlockNumber, index: BlockBodyIndices) -> Result<BlockBodyIndices>;
}

