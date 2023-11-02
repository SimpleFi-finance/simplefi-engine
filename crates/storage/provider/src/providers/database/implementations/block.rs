use simp_primitives::{BlockHashOrNumber, Block, H256};
use interfaces::Result;
use crate::DatabaseProvider;
use crate::traits::{BlockNumReader,HeaderProvider, TransactionsProvider, BlockReader};

impl BlockReader for DatabaseProvider {
    fn block(&self, id: BlockHashOrNumber) -> Result<Option<Block>> {
        if let Some(number) = self.convert_hash_or_number(id)? {
            if let Some(header) = self.header_by_number(number)? {

                let txs = self.transactions_by_block(id)?;
                return Ok(Some(Block {
                    header,
                    body: txs.unwrap_or_default(),
                }));
            }
        }

        Ok(None)
    }

    fn block_by_hash(&self, hash: H256) -> Result<Option<Block>> {
        self.block(hash.into())
    }

    fn block_by_number(&self, num: u64) -> Result<Option<Block>> {
        self.block(num.into())
    }
}
