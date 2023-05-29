use std::collections::HashSet;

use chrono::{Datelike, NaiveDateTime};
use grpc_server::client::AbiDiscoveryClient;
use rayon::prelude::IntoParallelIterator;
// use block_indexer::utils::{get_block_logs, get_block_with_txs};
use settings::load_settings as load_global_settings;

use chains_drivers::{
    ethereum::mainnet::ethereum_mainnet, 
    common::{base_chain::{GetBlocks, GetLogs}, types::evm::{block::{Block, self}, transaction::Tx, log::Log}},
};
use shared_types::data_lake::{SupportedDataLevels, SupportedDataTypes};
use shared_utils::{logger::init_logging, decoder::{types::ContractAbi, logs}};
use rayon::iter::ParallelIterator;
use third_parties::{mongo::lib::bronze::{
    blocks::types::Block as MongoBlock,
    logs::types::Log as MongoLog,
    txs::types::Tx as MongoTx,
}};
// todo add option to query only confirmed block data
#[tokio::main]
async fn main() {
    let global_settings = load_global_settings().unwrap();
    // todo add local settings
    init_logging(); 

    // connect to redis queue

    let chain_id = "1"; //todo switch to settings
    let block_number = 17_000_000;
    match chain_id {
        "1" => {
            let chain = ethereum_mainnet().await.unwrap();

            let block_with_txs = chain
                .get_blocks::<Block<Tx>, Block<Tx>, Tx>(
                    block_number, 
                    block_number, 
                    true
                ).unwrap();

            let logs = chain
                .get_logs::<Log>(block_number, block_number).unwrap();

            let unique_addresses = logs.clone().into_par_iter()
                .map(|l| l.address.unwrap())
                .collect::<HashSet<String>>()
                .into_iter()
                .collect::<Vec<String>>();

            let mut abi_discovery_client = AbiDiscoveryClient::new("http://[::1]:50051".to_string()).await;
            let abis_addresses = abi_discovery_client.get_addresses_abi_json(unique_addresses).await;
            let abis_response = abis_addresses.into_inner();

            let abis = abis_response.addresses_abi.into_par_iter().map(|abi| {
                ContractAbi {
                    address: abi.address,
                    abi: abi.abi,
                }
            }).collect::<Vec<ContractAbi>>();

            let timestamp = block_with_txs[0].timestamp.clone();

            let date = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();

            let txs = block_with_txs[0]
                .transactions
                .clone()
                .unwrap()
                .into_par_iter().map(|tx| {
                    MongoTx {
                        timestamp: date.timestamp_micros(),
                        year: date.year() as i16,
                        month: date.month() as i8,
                        day: date.day() as i8,
                        block_hash: tx.block_hash.clone(),
                        block_number: tx.block_number,
                        from: tx.from.clone(),
                        gas: tx.gas,
                        gas_price: tx.gas_price,
                        hash: tx.hash.clone(),
                        input: tx.input.clone(),
                        nonce: tx.nonce.clone(),
                        to: tx.to.clone(),
                        transaction_index: tx.transaction_index,
                        value: tx.value.clone(),
                        v: tx.v,
                        r: tx.r.clone(),
                        s: tx.s.clone(),
                    }
                })
                .collect::<Vec<MongoTx>>();

            let logs_mongo = logs
                .into_par_iter().map(|l| {
                    // todo decode logs
                    MongoLog {
                        address: l.address.clone(),
                        block_hash: l.block_hash.clone(),
                        block_number: l.block_number,
                        data: l.data.clone(),
                        log_index: l.log_index,
                        removed: l.removed,
                        topics: l.topics.clone(),
                        transaction_hash: l.transaction_hash.clone(),
                        transaction_index: l.transaction_index,
                        transaction_log_index: l.transaction_log_index,
                        year: date.year() as i16,
                        month: date.month() as i8,
                        day: date.day() as i8,
                        timestamp: date.timestamp_micros(),
                        decoded_data: None,
                        log_type: l.log_type.clone(),
                    }
                })
                .collect::<Vec<MongoLog>>();

            // Note: Blocks dont need saving as they already saved with websocket
            let(_,_) = tokio::join!(
                chain.chain.save_to_db(logs_mongo, &SupportedDataTypes::Logs, &SupportedDataLevels::Bronze),
                chain.chain.save_to_db(txs, &SupportedDataTypes::Transactions, &SupportedDataLevels::Bronze),
            );
        },
        _ => panic!("Chain not implemented for indexing"),
    };
}
