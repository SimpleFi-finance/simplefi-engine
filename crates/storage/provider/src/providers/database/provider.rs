use db::{
    implementation::sip_rocksdb::DB,
    tables::utils::decoder, table::Encode,
};
use interfaces::Result;
use rocksdb::{BoundColumnFamily, ReadOptions, TransactionDB};

use std::sync::Arc;

use crate::traits::ShardedTableProvider;

use super::options::AccessType;
use db::transaction::DbTx;
/// A provider struct that fetchs data from the database.
pub struct DatabaseProvider {
    pub db: DB,
    pub access_type: AccessType,
    _phantom_data: std::marker::PhantomData<TransactionDB>,
}

impl DatabaseProvider {
    /// Creates a provider with an inner read-only transaction.
    pub fn new(db: DB, access_type: AccessType) -> Self {
        Self {
            db,
            access_type,
            _phantom_data: std::marker::PhantomData,
        }
    }

    pub fn get_cf(&self, table: &str) -> Option<Arc<BoundColumnFamily>> {
        let cf = self.db.db.cf_handle(table);

        cf
    }

    pub fn into_db(self) -> TransactionDB {
        self.db.db
    }
}

impl ShardedTableProvider for DatabaseProvider {
    #[allow(unused_variables)]
    fn get_latest_shard<T: db::table::Table>(&self, prefix: &[u8]) -> Result<Option<&[u8]>> {

        unimplemented!()

        // let mut opts = ReadOptions::default();
        // opts.set_iterate_range(PrefixRange(prefix));

        // let mut iter = self.db.new_cursor::<T>(opts).unwrap();
        
        // iter.seek_to_last();

        // if iter.valid() {
        //     let k = decoder::<T>(
        //         (iter.key().unwrap().to_vec(),
        //         iter.value().unwrap().to_vec()),
        //     ).unwrap();
        //     let encoded_k = k.0.encode();
        //     Ok(Some(encoded_k))
        // } else {
        //     Ok(None)
        // }
    }

    fn get_shard<T: db::table::Table>(&self, prefix:  T::Key) -> Result<Option<T::Value>> {
        // resolves for the specified shard or finds the nearest one using a cursor
        let opts = ReadOptions::default();

        let mut iter = self.db.new_cursor::<T>(opts).unwrap();
        iter.seek(prefix.encode());

        if iter.valid() {
            let k = iter.key().unwrap();
            let v = iter.value().unwrap();

            let (_key, val) = decoder::<T>((k.to_vec(),v.to_vec())).unwrap();

            Ok(Some(val))
        } else {
            // if shard key is incorrect (looking at wrong shard division)
            Ok(None)
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use db::{
        implementation::sip_rocksdb::DB, init_db, tables::{Headers, TxLogs, models::TxLogId, ContractLogs, ShardedKey},
        test_utils::ERROR_TEMPDIR,
    };
    use simp_primitives::{Header, Address};
    use rocksdb::PrefixRange;
    use crate::traits::HeaderProvider;
    use db::table::Encode;

    #[test]
    fn test_db() {
        let db = init_db(&tempfile::TempDir::new().expect(ERROR_TEMPDIR).into_path());

        let db = DB::new(db.unwrap());

        let headers = vec![
            (0, Header::default()),
            (1, Header::default()),
            (2, Header::default()),
            (3, Header::default()),
            (4, Header::default()),
            (5, Header::default()),
            (6, Header::default()),
            (7, Header::default()),
            (8, Header::default()),
            (9, Header::default()),
            (10, Header::default()),
        ];

        for h in headers.clone() {
            db.put::<Headers>(h.0, h.1).unwrap();
        }

        let provider = DatabaseProvider::new(db, AccessType::Primary);
        let bn = provider.header_by_number(1);
        assert_eq!(bn.clone().unwrap().is_some(), true);
        let headers_db = provider.headers_range((1, 11)).unwrap();
        assert_eq!(headers_db.len(), 10);
        // bounds in rocksDB are non-inclusive so we need to substract 1

        let headers_db = provider.headers_range((1, 8)).unwrap();

        assert_eq!(headers_db.len(), 7);
    }

    #[test]
    fn test_logs_rw_by_address() {
        let db = init_db(&tempfile::TempDir::new().expect(ERROR_TEMPDIR).into_path());

        let db = DB::new(db.unwrap());
        let address_0 = Address::from(0);
        let address_1 = Address::from(1);

        let logs = vec![
            (
                address_0,
                1,
                TxLogs {
                    log_ids: vec![
                        TxLogId { tx: 1, log: 1, block_number: 1 },
                        TxLogId { tx: 1, log: 2, block_number: 1 },
                        TxLogId { tx: 1, log: 3, block_number: 1 },
                    ],
                },
            ),
            (
                address_0,
                2,
                TxLogs {
                    log_ids: vec![
                        TxLogId { tx: 1, log: 1, block_number: 1 },
                        TxLogId { tx: 1, log: 2, block_number: 1 },
                        TxLogId { tx: 1, log: 3, block_number: 1 },
                    ],
                },
            ),
            (
                address_0,
                3,
                TxLogs {
                    log_ids: vec![
                        TxLogId { tx: 1, log: 1, block_number: 1 },
                        TxLogId { tx: 1, log: 2, block_number: 1 },
                        TxLogId { tx: 1, log: 3, block_number: 1 },
                    ],
                },
            ),
            (
                address_0,
                4,
                TxLogs {
                    log_ids: vec![
                        TxLogId { tx: 1, log: 1, block_number: 1 },
                        TxLogId { tx: 1, log: 2, block_number: 1 },
                        TxLogId { tx: 1, log: 3, block_number: 1 },
                    ],
                },
            ),
            (
                address_0,
                5,
                TxLogs {
                    log_ids: vec![
                        TxLogId { tx: 1, log: 1, block_number: 1 },
                        TxLogId { tx: 1, log: 2, block_number: 1 },
                        TxLogId { tx: 1, log: 3, block_number: 1 },
                    ],
                },
            ),
            (
                address_1,
                6,
                TxLogs {
                    log_ids: vec![
                        TxLogId { tx: 1, log: 1, block_number: 1 },
                        TxLogId { tx: 1, log: 2, block_number: 1 },
                        TxLogId { tx: 1, log: 3, block_number: 1 },
                    ],
                },
            ),
            (
                address_1,
                7,
                TxLogs {
                    log_ids: vec![
                        TxLogId { tx: 1, log: 1, block_number: 1 },
                        TxLogId { tx: 1, log: 2, block_number: 1 },
                        TxLogId { tx: 1, log: 3, block_number: 1 },
                    ],
                },
            ),
        ];

        let provider = DatabaseProvider::new(db, AccessType::Primary);

        for log in logs.clone() {
            let key = ShardedKey::new(log.0, log.1);
            provider.db.put::<ContractLogs>(key, log.2).unwrap();
        }

        let mut opts = ReadOptions::default();
        opts.set_iterate_range(PrefixRange(address_0.encode().as_slice()));

        let mut iter = provider.db.new_cursor::<ContractLogs>(opts).unwrap();

        iter.seek_to_first();
        let mut logs_0 = Vec::new();
        while iter.valid() {
            let k = iter.key().unwrap();
            let value = iter.value().unwrap();
            let kv = decoder::<ContractLogs>((k.to_vec(), value.to_vec())).unwrap();
            println!("{:?}, {:?}", kv.0, kv.1);
            logs_0.push(kv.1);
            iter.next();
        }

        assert_eq!(logs_0.len(), 5);
    }
}
