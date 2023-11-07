use crate::DatabaseProvider;
use crate::traits::{TransactionsProvider, TransactionsWriter, BlockNumReader};
use db::tables::{TransactionBlock, self, TxHashNumber, BlockIndices, Transactions, TxIndices};
use interfaces::Result;
use simp_primitives::{BlockNumber, TransactionSigned, BlockHashOrNumber};
use rocksdb::ReadOptions;
use db::tables::utils::decoder;
use db::transaction::DbTx;
use db::table::Encode;

impl TransactionsProvider for DatabaseProvider {
    fn transaction_block(&self, id: simp_primitives::TxNumber) -> Result<Option<BlockNumber>> {
        Ok(self.db.dae_get::<TransactionBlock>(id)?)
    }

    fn transaction_by_hash(&self, hash: simp_primitives::TxHash) -> Result<Option<TransactionSigned>> {
        if let Some(id) = self.transaction_id(hash)? {
            Ok(self.transaction_by_id(id)?)
        } else {
            Ok(None)
        }
    }

    fn transaction_by_id(&self, id: simp_primitives::TxNumber) -> Result<Option<TransactionSigned>> {
        Ok(self.db.dae_get::<tables::Transactions>(id)?)
    }

    fn transaction_id(&self, tx_hash: simp_primitives::TxHash) -> Result<Option<simp_primitives::TxNumber>> {
        Ok(self.db.dae_get::<TxHashNumber>(tx_hash)?)
    }

    fn transactions_by_block(
        &self,
        block: BlockHashOrNumber,
    ) -> Result<Option<Vec<TransactionSigned>>> {
        let block_id = self.convert_hash_or_number(block)?;
        match block_id {
            None => Ok(None),
            Some(bn) => {
                let block_body_index = self.db.dae_get::<BlockIndices>(bn)?;
                match block_body_index {
                    None => Ok(None),
                    Some(index) => {
                        // iter for tx_ids
                        let tx_start = index.first_tx_num;
                        let tx_end = index.last_tx_num();

                        let mut txs = Vec::new();

                        for tx_id in tx_start..=tx_end {
                            let tx = self.db.dae_get::<tables::Transactions>(tx_id)?;
                            match tx {
                                None => continue,
                                Some(tx) => txs.push(tx),
                            }
                        }

                        Ok(Some(txs))
                    }
                }
            }
        }
    }

    fn transactions_by_block_range(
        &self,
        start: BlockNumber,
        end: BlockNumber,
    ) -> Result<Vec<Vec<TransactionSigned>>> {
        if end - start > 50 {
            panic!("Range too big");
        }

        let mut txs = Vec::new();
        let mut opts = ReadOptions::default();
        opts.set_iterate_range(start.encode().as_slice()..end.encode().as_slice());

        let mut iter = self.db.dae_new_cursor::<BlockIndices>(opts).unwrap();
        iter.seek_to_first();

        while iter.valid() {
            let k = iter.key().unwrap();
            let value = iter.value().unwrap();

            let kv = decoder::<BlockIndices>((k.to_vec(), value.to_vec())).unwrap();

            let index = kv.1;
            let tx_start = index.first_tx_num;
            let tx_end = index.last_tx_num();

            let mut txs_block = Vec::new();

            for tx_id in tx_start..=tx_end {
                let tx = self.transaction_by_id(tx_id)?;
                match tx {
                    None => continue,
                    Some(tx) => txs_block.push(tx),
                }
            }

            txs.push(txs_block);
            iter.next();
        }

        Ok(txs)
    }

    fn transactions_by_tx_range(
        &self,
        start: simp_primitives::TxNumber,
        end: simp_primitives::TxNumber,
    ) -> Result<Vec<TransactionSigned>> {
        if end - start > 10000 {
            panic!("Range too big");
        }

        let mut txs = Vec::new();
        let mut opts = ReadOptions::default();
        opts.set_iterate_range(start.encode().as_slice()..end.encode().as_slice());

        let mut iter = self.db.dae_new_cursor::<Transactions>(opts).unwrap();
        iter.seek_to_first();

        while iter.valid() {
            let k = iter.key().unwrap();
            let value = iter.value().unwrap();

            let kv = decoder::<Transactions>((k.to_vec(), value.to_vec())).unwrap();
            txs.push(kv.1);
            iter.next();
        }

        Ok(txs)
    }
}

impl TransactionsWriter for DatabaseProvider {
    // ATTENTION: this method must be called per block.
    fn insert_transactions(&self, transactions: Vec<TransactionSigned>) -> Result<TxIndices> {
        let opts = ReadOptions::default();
        let mut latest_tx = self.db.dae_new_cursor::<Transactions>(opts)?;
        latest_tx.seek_to_last();

        let latest_key = match latest_tx.valid() {
            true => {
                let (key, val) = (latest_tx.key().unwrap(), latest_tx.value().unwrap());
                let latest_key = decoder::<Transactions>((key.to_vec(), val.to_vec())).unwrap();
                latest_key.0
            },
            // TODO: error handling, dont assueme its just empty
            false => 0,
        };

        let mut key_holder = latest_key.clone() + 1;
        for tx in transactions.clone() {
            self.db.dae_put::<Transactions>(key_holder, tx.clone())?;
            self.db.dae_put::<TxHashNumber>(tx.hash(), key_holder)?;
            self.db
                .dae_put::<TransactionBlock>(key_holder, tx.block_number())?;
            key_holder += 1;
        }

        Ok(TxIndices {
            first_tx_num: latest_key.clone() + 1,
            tx_count: transactions.len() as u64,
        })
    }
}

#[cfg(test)] 
mod tests {
    use std::fs;

    use db::tables::{Transactions, BlockBodyIndices};
    use db::transaction::DbTx;
    use db::{init_db, test_utils::ERROR_TEMPDIR, implementation::sip_rocksdb::DB};
    use simp_primitives::TransactionSigned;
    use serde_json::Value;
    use crate::traits::{TransactionsWriter, TransactionsProvider, BlockBodyIndicesWriter};
    use crate::DatabaseProvider;

    fn get_provider() -> DatabaseProvider {
        let db = init_db(&tempfile::TempDir::new().expect(ERROR_TEMPDIR).into_path()).unwrap();
        DatabaseProvider::new(db, crate::providers::options::AccessType::Primary)
    }

    fn get_mock_txs() -> String {
        let path = "./src/mocks/mock_txs.json";
        fs::read_to_string(path).unwrap()
    }
    
    #[test]
    fn test_insert_and_read_tx() {
        let txs = get_mock_txs();

        let txs: Vec<Value> = serde_json::from_str(&txs).unwrap();

        let txs = txs.iter().map(|tx| {
            let tx = TransactionSigned::from(tx.clone());
            tx
        }).collect::<Vec<TransactionSigned>>();

        let provider = get_provider();

        let inserted = provider.insert_transactions(txs.clone()).unwrap();

        assert_eq!(inserted.tx_count, txs.len() as u64);
        assert!(inserted.first_tx_num > 0);

        let inserted = provider.insert_transactions(txs.clone()).unwrap();
        assert_eq!(inserted.tx_count, txs.len() as u64);
        assert_eq!(inserted.first_tx_num as usize, txs.len() + 1);
    }

    #[test]

    fn get_txs_by_range() {
        let provider = get_provider();

        let txs = get_mock_txs();

        let txs: Vec<Value> = serde_json::from_str(&txs).unwrap();

        let txs = txs.iter().map(|tx| {
            let tx = TransactionSigned::from(tx.clone());
            tx
        }).collect::<Vec<TransactionSigned>>();

        let mut inserted_num = 0;

        while inserted_num < 15000 {
            for tx in txs.clone().into_iter() {
                provider.db.dae_put::<Transactions>(inserted_num + 1, tx).unwrap();
                inserted_num += 1;
            }
        }
        let txs_retrieved = provider.transactions_by_tx_range(1, 200).unwrap();
    
        assert_eq!(txs_retrieved.len(), 199);
        assert_eq!(txs_retrieved[0].hash(), txs[0].hash());

        let txs_retrieved: Vec<TransactionSigned> = provider.transactions_by_tx_range(10000, 12000).unwrap();

        assert_eq!(txs_retrieved.len(), 2000);

    }

    #[test]
    fn test_txs_by_block_range_and_by_block() {
        let txs = get_mock_txs();

        let txs: Vec<Value> = serde_json::from_str(&txs).unwrap();
        let signed_txs = txs.iter().enumerate().map(|(i, tx)| {
            let mut tx = tx.clone();
            tx["block_number"] = serde_json::Value::Number(serde_json::Number::from(i));

            let tx = TransactionSigned::from(tx.clone());
            tx
        }).collect::<Vec<TransactionSigned>>();

        let provider = get_provider();
        for (i, tx) in signed_txs.iter().enumerate() {
            let inserted = provider.insert_transactions(vec![tx.clone()]).unwrap();
            let bn = match i > u64::MAX as usize {
                true => u64::MAX,
                false => i as u64,
            };
            let index = BlockBodyIndices {
                first_tx_num: inserted.first_tx_num,
                tx_count: inserted.tx_count,
            };

            provider.insert_block_body_indices(bn, index).unwrap();
        }

        let txs_found = provider.transactions_by_block_range(1, 10).unwrap();

        assert_eq!(txs_found.len(), 9);

        assert_eq!(txs_found[0][0].hash(), signed_txs[1].hash());

        let txs_by_block = provider.transactions_by_block(simp_primitives::BlockHashOrNumber::Number(1)).unwrap().unwrap();

        assert_eq!(txs_by_block.len(), 1);
    }

    #[test]
    fn test_tx_by_id() {
        let provider = get_provider();

        let txs = get_mock_txs();

        let txs: Vec<Value> = serde_json::from_str(&txs).unwrap();

        let txs = txs.iter().map(|tx| {
            let tx = TransactionSigned::from(tx.clone());
            tx
        }).collect::<Vec<TransactionSigned>>();

        let inserted = provider.insert_transactions(txs.clone()).unwrap();

        let tx = provider.transaction_by_id(inserted.first_tx_num).unwrap().unwrap();

        assert_eq!(tx.hash(), txs[0].hash());
    }

    #[test]
    fn test_tx_id() {
        let provider = get_provider();

        let txs = get_mock_txs();

        let txs: Vec<Value> = serde_json::from_str(&txs).unwrap();

        let txs = txs.iter().map(|tx| {
            let tx = TransactionSigned::from(tx.clone());
            tx
        }).collect::<Vec<TransactionSigned>>();

        let inserted = provider.insert_transactions(txs.clone()).unwrap();

        let tx_id = provider.transaction_id(txs[0].hash()).unwrap().unwrap();

        assert_eq!(tx_id, inserted.first_tx_num);
    }

    #[test]
    fn test_tx_by_hash() {
        let provider = get_provider();

        let txs = get_mock_txs();

        let txs: Vec<Value> = serde_json::from_str(&txs).unwrap();

        let txs = txs.iter().map(|tx| {
            let tx = TransactionSigned::from(tx.clone());
            tx
        }).collect::<Vec<TransactionSigned>>();

        provider.insert_transactions(txs.clone()).unwrap();

        let tx = provider.transaction_by_hash(txs[0].hash()).unwrap().unwrap();

        assert_eq!(tx.hash(), txs[0].hash());
    }

    #[test]
    fn test_tx_block() {
        let provider = get_provider();

        let txs = get_mock_txs();

        let txs: Vec<Value> = serde_json::from_str(&txs).unwrap();

        let txs = txs.iter().map(|tx| {
            let tx = TransactionSigned::from(tx.clone());
            tx
        }).collect::<Vec<TransactionSigned>>();

        let inserted = provider.insert_transactions(txs.clone()).unwrap();

        let block = provider.transaction_block(inserted.first_tx_num).unwrap().unwrap();

        assert_eq!(block, txs[0].block_number());
    }
}