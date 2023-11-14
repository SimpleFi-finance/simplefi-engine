use core::panic;
use std::collections::HashMap;

use crate::{
    error::StageError,
    stage::{ExecInput, ExecOutput},
    Stage,
};
use db::tables::{BlockBodyIndices, TxLogs};
use simp_primitives::{ChainSpec, StageId, ComputationEngine, ChainRpcProvider, Log, Transaction, TransactionSigned, TxNumber, TxHash, };
use storage_provider::{DatabaseProvider, traits::{TransactionsWriter, BlockBodyIndicesWriter, LogsWriter}};
use serde_json::Value;

pub struct BlockIndexingStage;

#[async_trait::async_trait]
impl Stage for BlockIndexingStage {
    fn id(&self) -> StageId {
        StageId::BlockIndexing
    }
    /// saves the Sealed header of the block in the database
    async fn execute(
        &mut self,
        input: ExecInput,
        db_provider: &DatabaseProvider,
        chain: &ChainSpec,
    ) -> Result<ExecOutput, StageError> {
        let target = input.target();
        let checkpoint = input.checkpoint() + 1;
        // load chain and load appropriate method

        for block in checkpoint..=target {
            match chain.chain_type() {
                ComputationEngine::EVM | ComputationEngine::EVMCompatible => {

                    let logs = chain.get_block_logs::<Log>(block).unwrap();

                    let txs = chain.get_block_txs::<String>(block).unwrap();

                    let txs = txs.iter().map(|tx| {
                        let tx = serde_json::from_str::<Value>(tx).unwrap();
                        TransactionSigned::from(tx)
                    }).collect::<Vec<TransactionSigned>>();

                    let (tx_indices, tx_num_hash) = db_provider.insert_transactions(txs).unwrap();
                    
                    let tx_hash_num = tx_num_hash.iter().map(|(num, hash)| {
                        (hash.clone(), num.clone())
                    }).collect::<Vec<(TxHash, TxNumber)>>();
                    
                    db_provider.insert_block_body_indices(block, BlockBodyIndices {
                        first_tx_num: tx_indices.first_tx_num,
                        tx_count: tx_indices.tx_count,
                    }).unwrap();

                    let mut tx_hash_logs = HashMap::new();

                    for log in logs.iter() {
                        let tx_hash = log.transaction_hash;
                        tx_hash_logs.entry(tx_hash).or_insert(vec![]).push(log.clone());
                    }

                    let tx_hash_logs = tx_hash_logs.iter().map(|(hash, logs)| {
                        let tx_num = tx_hash_num.iter().find(|(h, _)| h == hash).unwrap().1.clone();
                        (tx_num.clone(), logs.clone())
                    }).collect::<Vec<(TxNumber, Vec<Log>)>>();

                    db_provider.insert_logs(tx_hash_logs).unwrap();
                    // check bn to TxLogs
                    // check logid to storedLog

                    // TODO: create traces type

                    // let traces = chain.get_block_traces::<Trace).unwrap();
                },
                _ => panic!("chain not supported")
            }
        }
        Ok(ExecOutput {
            checkpoint: input.target(),
            done: true,
        })
    }
}
