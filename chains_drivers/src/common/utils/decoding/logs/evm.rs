use std::{error::Error, collections::HashMap};
use log::info;
use third_parties::mongo::lib::bronze::decoding_error::types::{DecodingError, ErrorType};
use std::str::FromStr;
use rayon::{iter::ParallelIterator};
use rayon::prelude::{IntoParallelRefIterator};
use third_parties::mongo::lib::bronze::logs::types::{Log, DecodedData};
use ethabi::ethereum_types::H256;
use ethabi::{RawLog, Contract, Token};

struct TokenWType {
    value: String,
    token_type: String,
}

fn get_token_type(token: Token) -> Result<TokenWType, Box<dyn Error>> {

    Ok(match token {
        Token::Bool(token) => TokenWType {
            token_type: "bool".to_string(),
            value: token.to_string(),
        },
        Token::String(token) => TokenWType {
            token_type: "string".to_string(),
            value: token.to_string(),
        },
        Token::Address(token) => TokenWType {
            token_type: "address".to_string(),
            value: format!("0x{:x}", token),
        },
        Token::Bytes(token) | Token::FixedBytes(token) => TokenWType {
            token_type: "bytes".to_string(),
            value: serde_json::to_string(&token).unwrap(),
        },
        Token::Uint(token) | Token::Int(token) => TokenWType {
            token_type: "int".to_string(),
            value: token.to_string(),
        },
        Token::Array(token) | Token::FixedArray(token) => TokenWType {
            token_type: "array".to_string(),
            value: serde_json::to_string(&token).unwrap(),
        },
        Token::Tuple(token) => TokenWType {
            token_type: "tuple".to_string(),
            value: serde_json::to_string(&token).unwrap(),
        },
    })
}

pub fn evm_logs_decoder(logs_by_address: HashMap<String, Vec<Log>>, abis: Vec<grpc_server::abi_discovery_proto::AddressAbiJson>) -> Result<(Vec<Log>, Vec<DecodingError>), Box<dyn Error>>{

    let mut eventhm = HashMap::new();

    let contracts_with_abi = abis.iter().map(|a| {
        let abi = &a.abi;
        let contract: Contract = serde_json::from_str(abi.as_str()).unwrap();
        for event in &contract.events {
            let e = event.1[0].clone();
    
            eventhm.insert(e.signature(), e);
        }
        a.address.clone()
    })
    .collect::<Vec<String>>();

    let unique_addresses = Vec::from_iter(logs_by_address.keys().cloned());

    let all_addresses = unique_addresses.clone();

    let contract_no_abi = all_addresses
        .par_iter()
        .filter(|a| !contracts_with_abi.contains(a))
        .collect::<Vec<&String>>();

    let decoded_logs = contracts_with_abi
        .par_iter()
        .map(|address| {
            let logs_of_address = logs_by_address.get(address).unwrap();
            
            let mut errors = vec![];

            let decoded = logs_of_address
            .iter()
            .map(|log| {

                let log = log.clone();
                let tx_hash = log.transaction_hash.clone().unwrap();

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
                                let decoded_data = decoded_log.params.iter().enumerate().map(|(i,d)| {
                                    
                                    let token_type = get_token_type(d.value.clone());

                                    match token_type {
                                        Ok(token_type) => {
                                            let decoded_data = DecodedData {
                                                name: d.name.clone(),
                                                value: token_type.value,
                                                kind: token_type.token_type,
                                                indexed: event.inputs[i].indexed,
                                                hash_signature: format!("0x{:x}", event.signature()),
                                                signature: event.name.clone(),
                                            };
                                            decoded_data
                                        },
                                        Err(e) => {
                                            info!("unsupported_data_type: {:?}", e);
                                            // push error to mongo
                                            let error = DecodingError {
                                                timestamp: log.timestamp,
                                                error: ErrorType::UnsupportedDataType,
                                                contract_address: log.address.clone().unwrap(),
                                                log: format!("{}|{}|{}", tx_hash, log.transaction_index, log.log_index),
                                            };

                                            errors.push(error);

                                            DecodedData {
                                                name: d.name.clone(),
                                                value: d.value.to_string(),
                                                kind: event.inputs[i].kind.to_string(),
                                                indexed: event.inputs[i].indexed,
                                                hash_signature: format!("0x{:x}", event.signature()),
                                                signature: event.name.clone(),
                                            }
                                        }
                                    }
                                }).collect::<Vec<DecodedData>>();    

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
                                    decoded_data: Some(decoded_data),
                                    timestamp: log.timestamp,
                                    year: log.year,
                                    month: log.month,
                                    day: log.day,
                                };
                                decoded_log
                            },
                            Err(e) => {
                                info!("error invalid data: {:?}", e);
                                let error = DecodingError {
                                    timestamp: log.timestamp,
                                    error: ErrorType::InvalidData,
                                    contract_address: log.address.clone().unwrap(),
                                    log: format!("{}|{}|{}", tx_hash, log.transaction_index, log.log_index),
                                };
                                errors.push(error);
                                log.clone()
                            }
                        }
                    },
                    None => {
                        info!("event not found for address {:?}", &log.address);
                        let error = DecodingError {
                            timestamp: log.timestamp,
                            error: ErrorType::EventNotFound,
                            contract_address: log.address.clone().unwrap(),
                            log: format!("{}|{}|{}", tx_hash, log.transaction_index, log.log_index),
                        };
                        errors.push(error);
                        log.clone()
                    }
                }
            })
            .collect::<Vec<Log>>();

            (decoded, errors)
        })
        .collect::<Vec<(Vec<Log>, Vec<DecodingError>)>>();
    
    let mut decoding_errors = vec![];
    let mut decoded = vec![];

    for (d, e) in decoded_logs {
        decoded.extend(d);
        decoding_errors.extend(e);
    }

    let undecoded = contract_no_abi
        .par_iter()
        .map(|address| {
            let address= address.clone();
            let logs_of_address = logs_by_address.get(address).unwrap();
            Vec::from_iter(logs_of_address.clone())
        })
        .flatten()
        .collect::<Vec<Log>>();

    let decoded_logs: Vec<Log> = [decoded, undecoded].concat();

    Ok((decoded_logs, decoding_errors))
}