use db::tables::HeaderNumbers;
use interfaces::Result;
use primitives::{BlockNumber, H256};
use rocksdb::ReadOptions;

use crate::traits::BlockNumWriter;
use crate::{traits::BlockNumReader, DatabaseProvider};
use db::transaction::DbTx;
use db::tables::utils::decoder;

impl BlockNumReader for DatabaseProvider {
    fn last_block_number(&self) -> Result<BlockNumber> {
        let opts = ReadOptions::default();
        let mut iter = self.db.new_cursor::<HeaderNumbers>(opts).unwrap();
        iter.seek_to_last();
        if iter.valid() {
            let k = iter.key().unwrap();
            let value = iter.value().unwrap();
            let kv =
                decoder::<HeaderNumbers>((k.as_ref().to_vec(), value.as_ref().to_vec())).unwrap();
            Ok(kv.1)
        } else {
            panic!("No valid block number found")
        }
    }

    fn block_number(&self, hash: H256) -> Result<Option<BlockNumber>> {
        Ok(self.db.get::<HeaderNumbers>(hash).unwrap())
    }
}

impl BlockNumWriter for DatabaseProvider {
    fn insert_block_number(&self, hash: H256, number: BlockNumber) -> Result<()> {
        // TODO: insert validation?
        self.db.put::<HeaderNumbers>(hash, number)?;
        Ok(())
    }
}


#[cfg(test)] 
mod tests {
    use db::{init_db, test_utils::ERROR_TEMPDIR, implementation::sip_rocksdb::DB};
    use primitives::H256;
    use crate::traits::{BlockNumWriter, BlockNumReader};
    use crate::DatabaseProvider;

    fn get_provider() -> DatabaseProvider {
        let db = init_db(&tempfile::TempDir::new().expect(ERROR_TEMPDIR).into_path());

        let db = DB::new(db.unwrap());

        DatabaseProvider::new(db, crate::providers::options::AccessType::Primary)
    }

    #[test]
    fn insert_and_retrieve_bn() {
        let provider = get_provider();

        let mut i = 0;

        while i < 20 {
            let hash = H256::from_low_u64_be(i);
            provider.insert_block_number(hash, i).unwrap();
            
            let retrieved = provider.block_number(hash).unwrap();
            assert_eq!(retrieved, Some(i));

            i += 1;
        }

        let last = provider.last_block_number().unwrap();
        assert_eq!(last, 19);
    }
}