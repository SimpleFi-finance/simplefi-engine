use std::collections::{HashSet};
use grpc_server::client::AbiDiscoveryClient;
use rayon::{iter::ParallelIterator};
use rayon::prelude::{IntoParallelRefIterator, IntoParallelIterator};
use shared_utils::decoder::logs::evm::evm_logs_decoder;
use shared_utils::decoder::types::ContractAbi;
use third_parties::mongo::Mongo;
use third_parties::mongo::lib::bronze::blocks::getters::get_blocks;
use third_parties::mongo::lib::bronze::decoding_error::types::DecodingError;
use third_parties::mongo::lib::bronze::logs::types::Log as MongoLog;

use crate::ethereum::types::raw::log::EthLog;

// returns logs with extra info such as timestamp, year, month, day, decoded_data. if a log does not have an abi available the decoded_data field will be empty
pub async fn decode_logs(logs: Vec<EthLog>, db: &Mongo) -> Result<(Vec<MongoLog>, Vec<DecodingError>), Box<dyn std::error::Error>> {
    // todo remove hardcode of the service url
    let mut abi_discovery_client = AbiDiscoveryClient::new("http://[::1]:50051".to_string()).await;

    let block_data = get_blocks(db, None, None, None, None).await.unwrap();

    let block = block_data.first();

    let block_time = match block {
        Some(block) => (block.timestamp, block.year, block.month, block.day),
        None => (0, 0, 0, 0),
    };


    let unique_addresses = logs.clone().into_par_iter()
        .map(|log| log.address.unwrap())
        .collect::<HashSet<String>>()
        .into_iter()
        .collect::<Vec<String>>();

    // get abis
    // todo add pagination of abi requests (and pagination of logs decoding)

    let abis_addresses = abi_discovery_client.get_addresses_abi_json(unique_addresses.clone()).await;

    let abis_response = abis_addresses.into_inner();

    let logs = logs.par_iter().map(|log| {

        MongoLog {
            address: log.address.clone(),
            topics: log.topics.clone(),
            data: log.data.clone(),
            block_number: log.block_number,
            transaction_hash: log.transaction_hash.clone(),
            transaction_index: log.transaction_index,
            block_hash: log.block_hash.clone(),
            log_type: log.log_type.clone(),
            transaction_log_index: log.transaction_log_index,
            log_index: log.log_index,
            removed: log.removed,
            timestamp: block_time.0,
            year: block_time.1,
            month: block_time.2,
            day: block_time.3,
            decoded_data: None,
        }
    }).collect::<Vec<MongoLog>>();

    let abis = abis_response.addresses_abi.into_par_iter().map(|abi| {
        ContractAbi {
            address: abi.address,
            abi: abi.abi,
        }
    }).collect::<Vec<ContractAbi>>();

    let decoded_logs = evm_logs_decoder(logs, abis);

    decoded_logs
}