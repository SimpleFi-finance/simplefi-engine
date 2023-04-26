use std::collections::HashMap;
use std::str::FromStr;

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
    let mut decoded_logs: Vec<Log> = Vec::new();
    // get bn timestamp from mongo
    let mut abi_discovery_client = AbiDiscoveryClient::new("http://[::1]:50051".to_string()).await;

    // todo get block data from database or default to null

    let block_data = Block::default();

    let mut logs_by_address = logs
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

    let abis_addresses = abi_discovery_client.get_addresses_abi_json(unique_addresses).await;
    let abis_response = abis_addresses.into_inner();

    for i in abis_response.addresses_abi {
        let data = i;
        let abi = &data.abi;
        let contract: Contract = serde_json::from_str(abi.as_str()).unwrap();

        let mut eventhm = HashMap::new();
        for event in &contract.events {
            let e = event.1[0].clone();

            eventhm.insert(e.signature(), e);
        }

        let logs_of_address = logs_by_address.get(&data.address).unwrap();

        logs_of_address.iter().for_each(|log| {
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
                            let decoded = Log {
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
                            // println!("decoded_log: {:?}", &decoded_log);
                            decoded_logs.push(decoded);
                        },
                        Err(e) => {
                            decoded_logs.push(log.clone());
                            // println!("error: {:?}", e);
                        }
                    }
                },
                None => {
                    decoded_logs.push(log.clone());
                    println!("event not found for address {:?}", &log.address);
                }
            }
        });

        // remove key value from hm
        logs_by_address.remove(&data.address);
    }

    logs_by_address
        .values()
        .flatten()
        .for_each(|l| {
            decoded_logs.push(l.clone());
        });
    println!("original logs {:?}, decoded_logs: {:?}", logs.len(), decoded_logs.len());
    Ok(decoded_logs)
}