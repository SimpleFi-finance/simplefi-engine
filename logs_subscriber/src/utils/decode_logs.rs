use std::collections::HashMap;
use std::str::FromStr;
use log::{debug, info, error};
use chrono::{Datelike, NaiveDateTime};
use ethabi::ethereum_types::H256;
use ethabi::{RawLog, Contract};
use grpc_server::client::AbiDiscoveryClient;
use rayon::{iter::ParallelIterator};
use rayon::prelude::IntoParallelRefIterator;
use third_parties::mongo::lib::bronze::blocks::types::Block;
use third_parties::mongo::lib::bronze::logs::types::Log;

// returns logs with extra info such as timestamp, year, month, day, decoded_data. if a log does not have an abi available the decoded_data field will be empty
pub async fn decode_logs(logs: Vec<Log>) -> Result<Vec<Log>, Box<dyn std::error::Error>> {
    // get bn timestamp from mongo
    let mut abi_discovery_client = AbiDiscoveryClient::new("http://[::1]:50051".to_string()).await;

    // todo get block data from database or default to null

    let block_data = Block::default();

    let logs_by_address = logs
        .par_iter()
        .fold(||HashMap::new(), |mut acc, log| {
            let mut log_data = log.clone();
            let ts = block_data.timestamp.clone();
            let date = NaiveDateTime::from_timestamp_opt(ts, 0).unwrap();

            log_data.timestamp = Some(date.timestamp_micros());
            log_data.year = Some(date.year() as i16);
            log_data.month = Some(date.month() as i8);
            log_data.day = Some(date.day() as i8);
            log_data.decoded_data = None;

            acc
                .entry(log_data.address.clone().unwrap())
                .or_insert(vec![])
                .push(log_data);

            acc
        })
        .reduce(||HashMap::new(), |mut acc, hm| {
            for (key, value) in hm {
                acc
                .entry(key)
                .or_insert(vec![])
                .extend(value);
            }
            acc
        });

    let unique_addresses = Vec::from_iter(logs_by_address.keys().cloned());

    let abis_addresses = abi_discovery_client.get_addresses_abi_json(unique_addresses.clone()).await;

    let abis_response = abis_addresses.into_inner();

    let mut eventhm = HashMap::new();

    let contracts_with_abi = abis_response.addresses_abi.iter().map(|a| {
        let abi = &a.abi;
        let contract: Contract = serde_json::from_str(abi.as_str()).unwrap();
        for event in &contract.events {
            let e = event.1[0].clone();
    
            eventhm.insert(e.signature(), e);
        }
        a.address.clone()
    })
    .collect::<Vec<String>>();

    let all_addresses = unique_addresses.clone();

    let contract_no_abi = all_addresses
        .par_iter()
        .filter(|a| !contracts_with_abi.contains(a))
        .collect::<Vec<&String>>();

    let decoded = contracts_with_abi
        .par_iter()
        .map(|address| {
            let logs_of_address = logs_by_address.get(address).unwrap();

            let decoded = logs_of_address
            .par_iter()
            .map(|log| {
                let log = log.clone();
                let h256_topics = log.topics.iter().map(|t| H256::from_str(t).unwrap()).collect::<Vec<H256>>();
                let bytes = hex::decode(log.data.clone().unwrap().strip_prefix("0x").unwrap()).unwrap();
                let raw_log = RawLog {
                    topics: h256_topics.clone(),
                    data: bytes,
                };
    
                let event = eventhm.get(&h256_topics[0]);
                match event {
                    Some(event) => {
                        let decoded_log = event.parse_log(raw_log);
                        match decoded_log {
                            Ok(decoded_log) => {
                                let decoded_log = Log {
                                    address: log.address,
                                    log_type: log.log_type,
                                    block_number: log.block_number,
                                    block_hash: log.block_hash,
                                    data: log.data,
                                    log_index: log.log_index,
                                    removed: log.removed,
                                    topics: log.topics,
                                    transaction_hash: log.transaction_hash,
                                    transaction_index: log.transaction_index.clone(),
                                    transaction_log_index: log.transaction_log_index.clone(),
                                    // todo shape the decoded data appropriately
                                    decoded_data: None,
                                    timestamp: log.timestamp,
                                    year: log.year,
                                    month: log.month,
                                    day: log.day,
                                };
                                decoded_log
                            },
                            Err(e) => {
                                error!("error: {:?}", e);
                                log
                            }
                        }
                    },
                    None => {
                        error!("event not found for address {:?}", &log.address);
                        log
                    }
                }
            });

            decoded
        })
        .flatten()
        .collect::<Vec<Log>>();

    let undecoded = contract_no_abi
        .par_iter()
        .map(|address| {
            let address= address.clone();
            let logs_of_address = logs_by_address.get(address).unwrap();
            let decoded = Vec::from_iter(logs_of_address.clone());
            decoded
        })
        .flatten()
        .collect::<Vec<Log>>();

    let decoded_logs: Vec<Log> = [decoded, undecoded].concat();
    Ok(decoded_logs)
}