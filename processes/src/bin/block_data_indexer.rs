use chrono::{Datelike, NaiveDateTime};
use rayon::prelude::IntoParallelIterator;
use settings::load_settings as load_global_settings;

use chains_drivers::{
    ethereum::mainnet::ethereum_mainnet, 
    common::{base_chain::{GetBlocks, GetLogs}, types::evm::{block::Block, transaction::Tx, log::Log}},
};
use shared_types::data_lake::{SupportedDataLevels, SupportedDataTypes};
use shared_utils::{logger::init_logging};
use rayon::iter::ParallelIterator;
use third_parties::{mongo::lib::bronze::{
    logs::types::Log as MongoLog,
    txs::types::Tx as MongoTx,
}};

// todo add option to query only confirmed block data

async fn index_eth_mainnet_blocks (block_number: u64) {
    let chain = ethereum_mainnet().await.unwrap();

    let block_with_txs = chain
        .get_blocks::<Block<Tx>, Block<Tx>, Tx>(
            block_number, 
            block_number, 
            true
        ).unwrap();

    let logs = chain
        .get_logs::<Log>(block_number, block_number).unwrap();

    // let unique_addresses = logs.clone().into_par_iter()
    //     .map(|l| l.address.unwrap())
    //     .collect::<HashSet<String>>()
    //     .into_iter()
    //     .collect::<Vec<String>>();

    // let abis = get_chain_abis(&chain.chain.chain_id, &unique_addresses).await.unwrap();
    // todo implement decoding

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


    let decoded_logs = chain.decode_logs(logs_mongo).await.unwrap();
    // Note: Blocks dont need saving as they are already saved by websocket process
    let(_,_,_) = tokio::join!(
        chain.chain.save_to_db(decoded_logs.0, &SupportedDataTypes::Logs, &SupportedDataLevels::Bronze),
        chain.chain.save_to_db(decoded_logs.1, &SupportedDataTypes::DecodingError, &SupportedDataLevels::Bronze),
        chain.chain.save_to_db(txs, &SupportedDataTypes::Transactions, &SupportedDataLevels::Bronze),
    );
}
#[tokio::main]
async fn main() {
    let global_settings = load_global_settings().unwrap();
    // todo add local settings
    init_logging(); 

    //todo connect to redis queue

    let chain_id = "1"; //todo switch to settings
    let block_number = 17_000_000;
    match chain_id {
        "1" => index_eth_mainnet_blocks(block_number).await,
        _ => panic!("Chain not implemented for indexing"),
    };
}
