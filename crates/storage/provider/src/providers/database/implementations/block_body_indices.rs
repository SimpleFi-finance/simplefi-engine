use db::tables::{BlockBodyIndices, BlockIndices};
use simp_primitives::{BlockHashOrNumber, BlockNumber};
use interfaces::Result;
use crate::traits::{BlockBodyIndicesProvider,BlockBodyIndicesWriter};
use crate::DatabaseProvider;
use crate::traits::BlockNumReader;
use db::transaction::DbTx;

impl BlockBodyIndicesProvider for DatabaseProvider {
    fn block_body_indices(&self, block: BlockHashOrNumber) -> Result<Option<BlockBodyIndices>> {
        let bn = self.convert_hash_or_number(block)?;
        match bn {
            None => Ok(None),
            Some(bn) => {
                let block_body_index = self.db.dae_get::<BlockIndices>(bn)?;
                Ok(block_body_index)
            }
        }
    }
}

impl BlockBodyIndicesWriter for DatabaseProvider {
    fn insert_block_body_indices(
        &self,
        block_number: BlockNumber,
        index: BlockBodyIndices,
    ) -> Result<BlockBodyIndices> {
        // TODO: maybevalidate

        self.db.dae_put::<BlockIndices>(block_number, index.clone())?;
        Ok(index)
    }
}


#[cfg(test)]
mod test {
    use crate::{providers::options::AccessType, DatabaseProvider};
    use db::{
        init_db,
        test_utils::ERROR_TEMPDIR, tables::BlockBodyIndices,
    };
    use crate::traits::{BlockBodyIndicesWriter, BlockBodyIndicesProvider};


    fn get_provider() -> DatabaseProvider {
        let db = init_db(&tempfile::TempDir::new().expect(ERROR_TEMPDIR).into_path()).unwrap();

        DatabaseProvider::new(db, AccessType::Primary)
    }

    #[test]
    fn test_insert_and_retreive_index() {
        let provider = get_provider();

        let index = BlockBodyIndices {
            first_tx_num: 1,
            tx_count: 3000,
        };

        let block_number = 1;

        let saves = provider
            .insert_block_body_indices(block_number, index.clone())
            .unwrap();

        assert_eq!(index, saves);

        let retreived = provider
            .block_body_indices(block_number.into())
            .unwrap()
            .unwrap();

        assert_eq!(index, retreived);
        
    }
}