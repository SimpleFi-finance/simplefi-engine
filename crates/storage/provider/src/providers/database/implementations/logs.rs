use std::collections::HashMap;

use chrono::Utc;
use db::{
    common::PairResult,
    tables::{
        models::{sharded_key::NUM_OF_INDICES_IN_SHARD, TxLogId},
        BlockIndices, ContractLogs, DecodedLogs, Logs, ShardedKey, TransactionLogs, TxLogs,
    },
};
use primitives::{
    Address, BlockHashOrNumber, BlockNumber, Log, StoredDecodedData, StoredLog, TxNumber,
};

use rocksdb::{PrefixRange, ReadOptions};

use crate::traits::{
    AbiProvider, BlockNumReader, LogsProvider, LogsWriter, TrackingProvider, TransactionsProvider, AbiWriter,
};
use crate::{traits::StoredOrDecodedLog, DatabaseProvider};
use db::table::Encode;
use db::tables::utils::decoder;
use db::transaction::DbTx;
use interfaces::Result;

impl LogsProvider for DatabaseProvider {
    fn logs_by_tx_id(
        &self,
        tx_id: primitives::TxNumber,
        decoded: bool,
    ) -> Result<Option<Vec<StoredOrDecodedLog>>> {
        let mut logs = Vec::new();

        let mut opts = ReadOptions::default();
        // converts the TxNumber to a string and then to bytes
        opts.set_iterate_range(PrefixRange(tx_id.to_string().as_bytes()));

        if decoded {
            let mut iter = self.db.new_cursor::<DecodedLogs>(opts).unwrap();
            iter.seek_to_first();

            while iter.valid() {
                let k = iter.key().unwrap();
                let value = iter.value().unwrap();
                let kv = decoder::<DecodedLogs>((k.as_ref().to_vec(), value.as_ref().to_vec())).unwrap();
                let decoded_log = StoredOrDecodedLog::Decoded(kv.1);
                logs.push(decoded_log);
                iter.next();
            }
        } else {

            let mut iter = self.db.new_cursor::<Logs>(opts).unwrap();
            iter.seek_to_first();
    
            while iter.valid() {
                let k = iter.key().unwrap();
                let value = iter.value().unwrap();
                let kv = decoder::<Logs>((k.as_ref().to_vec(), value.as_ref().to_vec())).unwrap();
                let raw_log = StoredOrDecodedLog::Raw(kv.1);
                logs.push(raw_log);
                iter.next();
            }
        }
        Ok(Some(logs))

    }

    fn logs_by_block(
        &self,
        block: BlockHashOrNumber,
        decoded: bool,
    ) -> Result<Option<Vec<StoredOrDecodedLog>>> {
        let bn = self.convert_hash_or_number(block)?;
        match bn {
            None => Ok(None),
            Some(bn) => {
                let block_body_index = self.db.get::<BlockIndices>(bn)?;
                match block_body_index {
                    None => Ok(None),
                    Some(index) => {
                        // iter for tx_ids
                        let tx_start = index.first_tx_num;
                        let tx_end = index.last_tx_num();

                        let mut logs = Vec::new();

                        for tx_id in tx_start..=tx_end {
                            let tx_logs = self.logs_by_tx_id(tx_id, decoded)?;
                            match tx_logs {
                                None => continue,
                                Some(tx_logs) => {
                                    logs.extend(tx_logs);
                                }
                            }
                        }

                        Ok(Some(logs))
                    }
                }
            }
        }
    }

    fn logs_by_block_range(
        &self,
        start: BlockNumber,
        end: BlockNumber,
        decoded: bool,
    ) -> Result<Vec<StoredOrDecodedLog>> {
        if end - start > 10 {
            panic!("Range too big");
        }

        let mut logs = Vec::new();

        for bn in start..=end {
            let bn_logs = self.logs_by_block(bn.into(), decoded)?;

            match bn_logs {
                None => continue,
                Some(bn_logs) => {
                    logs.extend(bn_logs);
                }
            }
        }

        Ok(logs)
    }

    fn logs_by_tx_hash(
        &self,
        tx_hash: primitives::TxHash,
        decoded: bool,
    ) -> Result<Option<Vec<StoredOrDecodedLog>>> {
        let tx = self.transaction_id(tx_hash)?;
        match tx {
            None => Ok(None),
            Some(tx_id) => {
                let logs = self.logs_by_tx_id(tx_id, decoded)?;
                Ok(logs)
            }
        }
    }

    fn logs_by_address(
        &self,
        address: Address,
        from: Option<BlockNumber>,
        to: Option<BlockNumber>,
        decoded: bool,
    ) -> Result<Vec<StoredOrDecodedLog>> {
        let mut logs = Vec::new();
        let mut opts = ReadOptions::default();
        opts.set_iterate_range(PrefixRange(address.encode().as_slice()));
        if decoded {
            let mut iter = self
                .db
                .new_cursor::<ContractLogs>(opts)
                .unwrap();
            iter.seek_to_first();
            let min = from.unwrap_or(0);
            let max = match to {
                Some(x) => x,
                None => u64::MAX,
            };

            while iter.valid() {
                let k = iter.key().unwrap();
                let value = iter.value().unwrap();
                let kv =
                    decoder::<ContractLogs>((k.as_ref().to_vec(), value.as_ref().to_vec())).unwrap();
                let shard = kv.0.max_shard_value;
                if shard <= min {
                    iter.next()
                } else {
                    let logs_ids = kv.1.log_ids;

                    for log_id in logs_ids {
                        if log_id.block_number <= max && log_id.block_number >= min {
                            let id: String = log_id.into();
                            let log = self.db.get::<DecodedLogs>(String::from(id))?;
                            match log {
                                None => continue,
                                Some(log) => {
                                    let decoded_log = StoredOrDecodedLog::Decoded(log);
                                    logs.push(decoded_log)
                                },
                            }
                        }
                    }
                    iter.next()
                }
            }
        } else {
            let mut iter = self
                .db
                .new_cursor::<ContractLogs>(opts)
                .unwrap();

            iter.seek_to_first();

            let min = from.unwrap_or(0);
            let max = match to {
                Some(x) => x,
                None => u64::MAX,
            };

            while iter.valid() {
                let k = iter.key().unwrap();
                let value = iter.value().unwrap();
                let kv =
                    decoder::<ContractLogs>((k.as_ref().to_vec(), value.as_ref().to_vec())).unwrap();
                let shard = kv.0.max_shard_value;
                if shard <= min {
                    iter.next()
                } else {
                    let logs_ids = kv.1.log_ids;

                    for log_id in logs_ids {
                        if log_id.block_number <= max && log_id.block_number >= min {
                            let id: String = log_id.into();
                            let log = self.db.get::<Logs>(String::from(id))?;
                            match log {
                                None => continue,
                                Some(log) => {
                                    let raw_log = StoredOrDecodedLog::Raw(log);
                                    logs.push(raw_log)
                                },
                            }
                        }
                    }

                    if shard >= max {
                        break;
                    } else {
                        iter.next()
                    }
                }
            }
        }
        
        Ok(logs)
    }

    fn get_address_logs_latest_partition(&self, address: Address) -> PairResult<ContractLogs> {
        let mut opts = ReadOptions::default();
        opts.set_iterate_range(PrefixRange(address.encode().as_slice()));

        let mut iter = self.db.new_cursor::<ContractLogs>(opts).unwrap();
        iter.seek_to_last();
        if iter.valid() {
            let k = iter.key().unwrap();
            let value = iter.value().unwrap();
            let kv =
                decoder::<ContractLogs>((k.as_ref().to_vec(), value.as_ref().to_vec())).unwrap();

            Ok(Some(kv))
        } else {
            Ok(None)
        }
    }
}

impl LogsWriter for DatabaseProvider {
    fn insert_raw_logs(&self, log: (TxLogId, StoredLog)) -> Result<()> {
        self.db.put::<Logs>(log.0.into(), log.1)?;
        Ok(())
    }

    fn insert_decoded_data(&self, log: (TxLogId, primitives::StoredDecodedData)) -> Result<()> {
        self.db.put::<DecodedLogs>(log.0.into(), log.1)?;
        Ok(())
    }

    fn decode_and_store_logs(&self, logs: &HashMap<Address, Vec<TxLogId>>) -> Result<()> {
        // receive
        for address in logs.keys() {
            let is_required = self.is_contract_tracked(*address)?;
            if is_required {
                let abi = self.get_abis_by_address(*address)?.unwrap();
                let mut stored_logs = Vec::new();
                let address_logs_ids = logs.get(address).unwrap();

                for log_id in address_logs_ids {
                    let id: String = log_id.clone().into();
                    // TODO: method to catch missing logs
                    let stored_log = self.db.get::<Logs>(String::from(id))?.unwrap();
                    stored_logs.push(stored_log);
                }

                let decoded_logs = self.decode_logs(stored_logs, &abi)?;
                for (index, log_data) in decoded_logs.into_iter().enumerate() {
                    let id = address_logs_ids[index];
                    if log_data.is_some() {
                        let data = StoredDecodedData {
                            data: log_data.unwrap(),
                        };

                        self.insert_decoded_data((id, data))?;
                    }
                }
            } else {
                let abi_exists = self.get_contract_data(*address)?;
                if abi_exists.is_none() {
                    let timestamp = Utc::now().timestamp_micros() as u32;
                    self.insert_unknown_contract(*address, timestamp)?;
                }
            }
        }
        Ok(())
    }

    fn insert_logs_by_address(&self, logs: &HashMap<Address, Vec<TxLogId>>) -> Result<()> {
        for address in logs.keys() {
            // resolve partition

            let logs = logs.get(address).unwrap();
            // ATTENTION: this assumes that logs are always stored in order
            let latest_shard = self.get_address_logs_latest_partition(*address).unwrap();
            // if shard is full create a new one, else write as many in there and update the key max value
            match latest_shard {
                Some(shard) => {
                    if shard.1.log_ids.len() >= NUM_OF_INDICES_IN_SHARD {
                        // new shard
                        let logs_max = logs.last().unwrap().block_number;

                        let shard = ShardedKey::new(*address, logs_max);

                        self.db.put::<ContractLogs>(
                            shard,
                            TxLogs {
                                log_ids: logs.to_vec(),
                            },
                        )?;
                    } else {
                        // fill shard to max and then continue on new shard
                        let available_slots = NUM_OF_INDICES_IN_SHARD - shard.1.log_ids.len();
                        let mut new_logs_ids = shard.1.log_ids.clone();
                        let mut new_key = shard.0.clone();

                        let logs_to_save = &logs[0..available_slots];
                        new_logs_ids.extend(logs_to_save);

                        let bn = logs_to_save.last().unwrap().block_number;
                        new_key.max_shard_value = bn;
                        self.db.put::<ContractLogs>(
                            new_key.clone(),
                            TxLogs {
                                log_ids: new_logs_ids,
                            },
                        )?;

                        self.db.delete::<ContractLogs>(shard.0.clone())?;
                        // new shard to be created
                        if logs.len() > logs_to_save.len() {
                            let logs_to_save = &logs[available_slots..=logs.len()];
                            let mut new_shard = new_key.clone();
                            let max_bn = logs_to_save.last().unwrap().block_number;
                            new_shard.max_shard_value = max_bn;

                            self.db.put::<ContractLogs>(
                                new_shard,
                                TxLogs {
                                    log_ids: logs_to_save.to_vec(),
                                },
                            )?;
                        }
                    }
                }
                None => {
                    let mut logs_to_save = logs.clone();

                    while logs_to_save.len() > 0 {
                        let max = match logs_to_save.len() < NUM_OF_INDICES_IN_SHARD {
                            true => logs_to_save.len(),
                            false => NUM_OF_INDICES_IN_SHARD,
                        };

                        let l = logs_to_save.clone()[0..max].to_vec();
                        logs_to_save = logs_to_save.clone()[max..logs_to_save.len()].to_vec();

                        let logs_max = l.last().unwrap().block_number;
                        
                        // check amount of ids 
                        let shard = ShardedKey::new(*address, logs_max);

                        self.db.put::<ContractLogs>(
                            shard,
                            TxLogs {
                                log_ids: l.to_vec(),
                            },
                        )?;
                    }
                    
                }
            };
        }
        Ok(())
    }

    fn insert_logs(
        &self,
        logs: Vec<(TxNumber, Vec<Log>)>,
    ) -> Result<HashMap<Address, Vec<TxLogId>>> {
        // the ID of a saved log is the composition of the txNumber_logNumber starting from zero
        let mut tx_log_ids = vec![];

        let mut logs_by_address: HashMap<Address, Vec<TxLogId>> = HashMap::new();

        for tx in logs {
            let tx_id = tx.0;
            let mut log_id = 0;
            for log in tx.1 {
                let stored_log = StoredLog::from(log);

                let tx_log_id = TxLogId::from((
                    tx_id.clone(),
                    log_id.clone(),
                    stored_log.block_number.clone(),
                ));

                self.insert_raw_logs((tx_log_id.clone(), stored_log.clone()))?;

                logs_by_address
                    .entry(stored_log.address)
                    .or_insert(vec![])
                    .push(tx_log_id);

                tx_log_ids.push(tx_log_id);

                log_id += 1;
            }

            let tx_logs = TxLogs {
                log_ids: tx_log_ids.clone(),
            };

            // save all logs by tx
            self.db.put::<TransactionLogs>(tx.0, tx_logs)?;
        }

        self.insert_logs_by_address(&logs_by_address).unwrap();
        self.decode_and_store_logs(&logs_by_address).unwrap();

        Ok(logs_by_address)
    }
}

#[cfg(test)]
mod test {
    use crate::traits::{AbiProvider, LogsProvider, LogsWriter};
    use crate::{providers::options::AccessType, DatabaseProvider};
    use db::tables::models::AbiContract;
    use db::tables::AbiData;
    use db::{
        implementation::sip_rocksdb::DB, init_db, tables::models::TxLogId,
        test_utils::ERROR_TEMPDIR,
    };
    use hex_literal::hex;
    use primitives::Log;
    use primitives::{Address, H256};
    use std::collections::HashMap;
    use std::fs;
    fn get_provider() -> DatabaseProvider {
        let db = init_db(&tempfile::TempDir::new().expect(ERROR_TEMPDIR).into_path());

        let db = DB::new(db.unwrap());

        DatabaseProvider::new(db, AccessType::Primary)
    }

    fn get_uni_factory_abi() -> String {
        let path = "./src/mocks/uni_v2_factory.json";
        fs::read_to_string(path).unwrap()
    }

    fn get_uni_factory_logs() -> String {
        let path = "./src/mocks/uni_factory_logs.json";
        fs::read_to_string(path).unwrap()
    }

    fn get_mock_decoded_logs() -> Vec<Option<Vec<primitives::DecodedData>>> {
        let provider = get_provider();
        let abi = get_uni_factory_abi();
        let logs = get_uni_factory_logs();

        let logs: Vec<Log> = serde_json::from_str(&logs).unwrap();
        let stored_logs = logs
            .iter()
            .map(|log| {
                let stored_log = primitives::StoredLog::from(log.clone());
                stored_log
            })
            .collect::<Vec<primitives::StoredLog>>();

        let contract = AbiContract {
            address: Address::from(hex!("5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f")),
            abi: AbiData {
                body: abi.as_bytes().to_vec(),
                hash: H256::default(),
            },
            contract_type: "contract".to_string(),
        };

        let decoded_logs: Vec<Option<Vec<primitives::DecodedData>>> =
            provider.decode_logs(stored_logs, &vec![contract]).unwrap();
        decoded_logs
    }

    #[test]
    fn insert_raw_log_and_retrieve() {
        let provider = get_provider();

        let logs = get_uni_factory_logs();

        let logs: Vec<Log> = serde_json::from_str(&logs).unwrap();
        let mut tx_ids = vec![];
        for (i, log) in logs.iter().enumerate() {
            let stored_log = primitives::StoredLog::from(log.clone());

            let tx_log_id = TxLogId::from((1, i as u64, stored_log.block_number.clone()));
            tx_ids.push(tx_log_id.clone());

            provider
                .insert_raw_logs((tx_log_id.into(), stored_log))
                .unwrap();
        }

        let logs = provider.logs_by_tx_id(1, false).unwrap();

        assert_eq!(logs.unwrap().len(), tx_ids.len());
    }

    #[test]
    fn insert_decoded_data_and_retrieve() {
        let provider = get_provider();
        let logs = get_uni_factory_logs();

        let logs: Vec<Log> = serde_json::from_str(&logs).unwrap();

        let decoded_logs = get_mock_decoded_logs();

        assert_eq!(logs.len(), decoded_logs.len());

        // store decoded logs
        for (i, log) in logs.iter().enumerate() {
            let tx_log_id = TxLogId::from((1, i as u64, log.block_number.clone()));
            provider
                .insert_decoded_data((
                    tx_log_id.into(),
                    primitives::StoredDecodedData {
                        data: decoded_logs[i].clone().unwrap(),
                    },
                ))
                .unwrap();
        }

        // retrieve decoded logs

        let logs = provider.logs_by_tx_id(1, true).unwrap();

        assert_eq!(logs.unwrap().len(), decoded_logs.len());
    }

    #[test]
    fn insert_logs_by_address_and_retrieve() {
        let provider = get_provider();
        let logs = get_uni_factory_logs();
        let logs: Vec<Log> = serde_json::from_str(&logs).unwrap();

        let mut logs_to_store = vec![];
        let mut i = 1;

        while logs_to_store.len() < 11000 {
            for log in logs.clone() {
                let mut log = log.clone();

                log.block_number = i;
                logs_to_store.push(log.clone());
                i += 1;
            }
        }

        let mut logs_by_address = HashMap::new();
        for (i, log) in logs_to_store.iter().enumerate() {
            let id: TxLogId = TxLogId::from((1, i as u64, log.block_number.clone()));
            provider.insert_raw_logs((id, primitives::StoredLog::from(log.clone()))).unwrap();
            logs_by_address
                .entry(log.address)
                .or_insert(vec![])
                .push(TxLogId::from((1, i as u64, log.block_number.clone())));
        }

        provider.insert_logs_by_address(&logs_by_address).unwrap();

        let logs_by_address = provider
            .logs_by_address(logs[0].address, Some(0), Some(10000), false)
            .unwrap();

        assert!(logs_by_address.len() == 10000);
        assert!(logs_by_address.len() > 0);

        let logs_by_address = provider
            .logs_by_address(logs[0].address, Some(5000), Some(11000), false)
            .unwrap();

        assert!(logs_by_address.len() == 6001);
        assert!(logs_by_address.len() > 0);

    }
}
