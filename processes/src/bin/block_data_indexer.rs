// use block_indexer::utils::{get_block_logs, get_block_with_txs};
// use lapin::{options::{BasicConsumeOptions, BasicAckOptions}, types::FieldTable};
use settings::load_settings as load_global_settings;

use chains_drivers::{
    ethereum::mainnet::ethereum_mainnet, 
    common::{base_chain::GetBlocks, types::evm::{block::Block, transaction::Tx}},
};
use shared_utils::logger::init_logging;

#[tokio::main]
async fn main() {
    let global_settings = load_global_settings().unwrap();
    init_logging(); 

    // connect to redis queue
    // listen to blocknumebrs minted
    // get logs, txs and block data
    // can be used with multiple workers

    let chain_id = "1"; //todo switch to settings
    let block_number = 17_000_000;
    match chain_id {
        "1" => {
            let chain = ethereum_mainnet().await.unwrap();

            // load data of chain, connect to node, digest data, die
            let block_with_txs = chain
                .get_blocks::<Block<String>, Block<String>, Tx>(
                    block_number, 
                    block_number, 
                    true
                ).unwrap();
            // todo add call for logs
        },
        _ => panic!("Chain not implemented for indexing"),
    };

    todo!("add timestamp to logs, decode logs, save all data");

    // let db = chain.db.clone();

    
    // while let Some(delivery) = consumer_stream.next().await {
    //     let delivery_data = delivery.unwrap();
    //     let block: i64 = serde_json::from_slice(&delivery_data.data).unwrap();
    //     info!("Got message: {:?}", block);

    //     let now = Instant::now();
        
    //     // get logs and txs and save in mongo
    //     let u64_block = block as u64;

    //     let (logs, block) = tokio::join!(
    //         get_block_logs(provider_url.clone(), &u64_block, &u64_block),
    //         get_block_with_txs(provider_url.clone(), &u64_block)
    //     );

    //     let logs = logs.unwrap()
    //         .par_iter()
    //         .map(|l| {
    //             let ts = block.0.clone().unwrap().timestamp.clone();
    //             let date = NaiveDateTime::from_timestamp_opt(ts, 0).unwrap();
    //             Log {
    //                 address: l.address.clone(),
    //                 block_hash: l.block_hash.clone(),
    //                 block_number: l.block_number,
    //                 data: l.data.clone(),
    //                 log_index: l.log_index,
    //                 removed: l.removed,
    //                 topics: l.topics.clone(),
    //                 transaction_hash: l.transaction_hash.clone(),
    //                 transaction_index: l.transaction_index,
    //                 transaction_log_index: l.transaction_log_index,
    //                 year: date.year() as i16,
    //                 month: date.month() as i8,
    //                 day: date.day() as i8,
    //                 timestamp: date.timestamp_micros(),
    //                 decoded_data: None,
    //                 log_type: l.log_type.clone(),
    //             }
    //         })
    //         .collect::<Vec<Log>>();
    //         .collect::<Vec<Log>>();
        
    //     let unique_addresses = logs.clone().into_par_iter()
    //         .map(|l| l.address.unwrap())
    //         .collect::<HashSet<String>>()
    //         .into_iter()
    //         .collect::<Vec<String>>();

    //     // todo set client dtnamically
    //     let mut abi_discovery_client = AbiDiscoveryClient::new("http://[::1]:50051".to_string()).await;
    //     let abis_addresses = abi_discovery_client.get_addresses_abi_json(unique_addresses).await;
    //     let abis_response = abis_addresses.into_inner();

    //     let abis = abis_response.addresses_abi.into_par_iter().map(|abi| {
    //         ContractAbi {
    //             address: abi.address,
    //             abi: abi.abi,
    //         }
    //     }).collect::<Vec<ContractAbi>>();

    //     let decoded_logs = evm_logs_decoder(logs, abis).unwrap();

    //     debug!("Time elapsed is: {:?}ms", now.elapsed().as_millis());

    //     let mongo_txs = block.1.unwrap().par_iter().map(|tx| {
    //         let ts = block.0.clone().unwrap().timestamp.clone();
    //         let date = NaiveDateTime::from_timestamp_opt(ts, 0).unwrap();
            
    //         Tx {
    //             timestamp: date.timestamp_micros(),
    //             year: date.year() as i16,
    //             month: date.month() as i8,
    //             day: date.day() as i8,
    //             block_hash: tx.block_hash.clone(),
    //             block_number: tx.block_number,
    //             from: tx.from.clone(),
    //             gas: tx.gas,
    //             gas_price: tx.gas_price,
    //             hash: tx.hash.clone(),
    //             input: tx.input.clone(),
    //             nonce: tx.nonce.clone(),
    //             to: tx.to.clone(),
    //             transaction_index: tx.transaction_index,
    //             value: tx.value.clone(),
    //             v: tx.v,
    //             r: tx.r.clone(),
    //             s: tx.s.clone(),
    //         }
    //     }).collect::<Vec<Tx>>();

    //     let (_, _, _) = tokio::join!(
    //         save_logs(&db, decoded_logs.0),
    //         save_decoding_error(&db, decoded_logs.1),
    //         save_txs(&db, mongo_txs)
    //     );

    //     channel.basic_ack(delivery_data.delivery_tag, BasicAckOptions::default()).await.unwrap();
    // }
}
