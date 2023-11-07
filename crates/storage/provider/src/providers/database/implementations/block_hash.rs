use db::tables::CanonicalHeaders;
use interfaces::Result;
use rocksdb::ReadOptions;

use crate::traits::BlockHashWriter;
use crate::{traits::BlockHashReader, DatabaseProvider};
use simp_primitives::{
    H256, BlockNumber,
};

use db::transaction::DbTx;
use db::tables::utils::decoder;
use db::table::Encode;

impl BlockHashReader for DatabaseProvider {
    fn block_hash(&self, number: u64) -> Result<Option<H256>> {
        Ok(self.db.dae_get::<CanonicalHeaders>(number).unwrap())
    }

    fn block_hashes_range(&self, start: BlockNumber, end: BlockNumber) -> Result<Vec<H256>> {
        let start = start.encode().to_vec();

        let mut opts = ReadOptions::default();
        opts.set_iterate_range(start..end.encode().to_vec());
        let mut iter = self.db.dae_new_cursor::<CanonicalHeaders>(opts).unwrap();
        iter.seek_to_first();
        let mut bn_range = Vec::new();

        while iter.valid() {
            let k = iter.key().unwrap();
            let value = iter.value().unwrap();
            let kv = decoder::<CanonicalHeaders>((k.as_ref().to_vec(), value.as_ref().to_vec()))
                .unwrap();
            bn_range.push(kv.1);
            iter.next();
        }

        Ok(bn_range)
    }
}

impl BlockHashWriter for DatabaseProvider {
    fn insert_block_hash(&self,number:BlockNumber,hash:H256) -> Result<()> {
        // TODO: add validation?
        self.db.dae_put::<CanonicalHeaders>(number, hash)?;
        Ok(())
    }
}

#[cfg(test)] 
mod tests {
    use db::{init_db, test_utils::ERROR_TEMPDIR, implementation::sip_rocksdb::DB};
    use simp_primitives::H256;
    use crate::traits::{BlockHashReader, BlockHashWriter};
    use crate::DatabaseProvider;

    fn get_provider() -> DatabaseProvider {
        let db = init_db(&tempfile::TempDir::new().expect(ERROR_TEMPDIR).into_path()).unwrap();

        DatabaseProvider::new(db, crate::providers::options::AccessType::Primary)
    }

    #[test]
    fn test_insert_and_retrieve_b_hash() {
        let provider = get_provider();

        let hash = H256::default();

        let bn = 1;

        provider.insert_block_hash(bn, hash).unwrap();

        let retrieved = provider.block_hash(bn).unwrap().unwrap();

        assert_eq!(hash, retrieved);
    }

    #[test]
    fn hashes_by_range() {
        let provider = get_provider();
        let mut i: u64 = 0;
        while i < 15 {
            let hash = H256::from_low_u64_be(i);
            let bn = i;
            provider.insert_block_hash(bn, hash).unwrap();
            i += 1;
        }

        let hashes = provider.block_hashes_range(0, 15).unwrap();

        assert_eq!(hashes.len(), 15);
        assert_eq!(hashes[0], H256::from_low_u64_be(0));
    }
}
