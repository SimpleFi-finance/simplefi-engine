use interfaces::db::DatabaseError;
use rocksdb::{TransactionDB, Options, ReadOptions, DBRawIteratorWithThreadMode, MultiThreaded};

use crate::{
    tables::utils::{decode_one, decoder},
    transaction::DbTx,
    table::{Compress, Encode, Table}, common::PairResult,
};

impl DbTx for TransactionDB<MultiThreaded> {
    fn dae_get<T: Table>(&self, key: T::Key) -> Result<Option<T::Value>, DatabaseError> {
        let tx = self.transaction();

        let cf = self.cf_handle(T::NAME).unwrap();
        let value = tx.get_cf(&cf, key.encode()).unwrap();

        match value {
            None => Ok(None),
            Some(val) => {
                let value = decode_one::<T>(val).unwrap();
                tx.commit().unwrap();
                Ok(Some(value))
            }
        }
    }

    fn dae_get_last<T: Table>(&self) -> PairResult<T> {
        let opts = ReadOptions::default();
        let mut iter = self.dae_new_cursor::<T>(opts).unwrap();
        iter.seek_to_last();
        if iter.valid() {
            let k = iter.key().unwrap();
            let v = iter.value().unwrap();
            let kv = decoder::<T>((k.to_vec(), v.to_vec())).unwrap();
            Ok(Some(kv))
        } else {
            Ok(None)
        }
    }

    fn dae_get_first<T: Table>(&self) -> PairResult<T> {
        let opts = ReadOptions::default();
        let mut iter = self.dae_new_cursor::<T>(opts).unwrap();
        iter.seek_to_first();
        if iter.valid() {
            let k = iter.key().unwrap();
            let v = iter.value().unwrap();
            let kv = decoder::<T>((k.to_vec(), v.to_vec())).unwrap();
            Ok(Some(kv))
        } else {
            Ok(None)
        }
    }

    fn dae_put<T: Table>(&self, key: T::Key, value: T::Value) -> Result<(), DatabaseError> {
        let cf = self.cf_handle(T::NAME).unwrap();

        let tx = self.transaction();
        tx.put_cf(&cf, key.encode(), value.compress()).unwrap();
        tx.commit().unwrap();
        Ok(())
    }

    fn dae_clear<T: Table>(&self) -> Result<(), DatabaseError> {
        let tx = self.transaction();

        self.drop_cf(T::NAME).unwrap();

        self.create_cf(T::NAME, &Options::default()).unwrap();

        tx.commit().unwrap();        
        Ok(())
    }

    fn dae_delete<T: Table>(&self, key: T::Key) -> Result<bool, DatabaseError> {
        let tx = self.transaction();

        let cf = self.cf_handle(T::NAME).unwrap();

        tx.delete_cf(&cf, key.encode()).unwrap();

        tx.commit().unwrap();

        Ok(true)
    }

    fn dae_entries<T: Table>(&self) -> Result<usize, DatabaseError> {
        let tx = self.transaction();

        let cf = self.cf_handle(T::NAME);

        match cf {
            None => {
                tx.commit().unwrap();    
                Ok(0)
            },
            Some(cf) => {
                let iter = tx.full_iterator_cf(&cf, rocksdb::IteratorMode::Start);
                
                let count = iter.count();

                tx.commit().unwrap();

                Ok(count)
            }
        }
    }

    // fn dae_drop(&self) {
    //     drop(&self.transaction())
    // }

    fn dae_new_cursor<T: Table>(&self, opts: ReadOptions) -> Result<DBRawIteratorWithThreadMode<TransactionDB>, DatabaseError> {
        let cf_handle = self.cf_handle(T::NAME).unwrap();
        let iter = self.raw_iterator_cf_opt(&cf_handle, opts);

        Ok(iter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        tables::Headers,
        test_utils::*,
        transaction::DbTx,
        init_db,
    };
    use simp_primitives::Header;

    /// Create database for testing
    fn create_test_db() -> eyre::Result<TransactionDB> {
        init_db(
            &tempfile::TempDir::new().expect(ERROR_TEMPDIR).into_path(),
        )
    }

    #[test]
    fn db_manual_put_get() {
        let db = create_test_db().unwrap();

        // let db = DB::new(db);

        let value = Header::default();
        let key = 1u64;

        db.dae_put::<Headers>(key.clone(), value.clone()).unwrap();

        let count = db.dae_entries::<Headers>().unwrap();
        assert_eq!(count, 1);
        let data = db.dae_get::<Headers>(key).unwrap();

        assert_eq!(data, Some(value.clone()));

        db.dae_delete::<Headers>(key).unwrap();

        let count = db.dae_entries::<Headers>().unwrap();

        assert_eq!(count, 0);

        db.dae_put::<Headers>(key.clone(), value.clone()).unwrap();
        db.dae_clear::<Headers>().unwrap();

        let count = db.dae_entries::<Headers>().unwrap();

        assert_eq!(count, 0);
    }
}
