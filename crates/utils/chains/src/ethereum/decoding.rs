use std::{error::Error, collections::HashMap};
use serde_json::Value;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use ethabi::ethereum_types::H256;
use ethabi::Token;

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

// TODO: update to replace grpc ContractInfo

pub fn evm_logs_decoder(logs_by_address: HashMap<String, Vec<Value>>, abis: Vec<String>) -> Result<(Vec<Value>, Vec<Value>), Box<dyn Error>>{

    // let mut eventhm = HashMap::new();

    let contracts_with_abi = abis.iter().map(|a| {
        let abi: Option<String> = None;

        if abi.is_none() {
            return H256::zero().to_string();
        }
        
        return H256::zero().to_string();
    })
    .collect::<Vec<String>>();

    let unique_addresses = Vec::from_iter(logs_by_address.keys().cloned());

    let all_addresses = unique_addresses.clone();

    let contract_no_abi = all_addresses
        .par_iter()
        .filter(|a| !contracts_with_abi.contains(a))
        .collect::<Vec<&String>>();

    let mut decoding_errors = vec![];
    let mut decoded = vec![];

    let undecoded = contract_no_abi
        .par_iter()
        .map(|address| {
            let address= address.clone();
            let logs_of_address = logs_by_address.get(address).unwrap();
            Vec::from_iter(logs_of_address.clone())
        })
        .flatten()
        .collect::<Vec<Value>>();

    let decoded_logs: Vec<Value> = [decoded, undecoded].concat();

    Ok((decoded_logs, decoding_errors))
}