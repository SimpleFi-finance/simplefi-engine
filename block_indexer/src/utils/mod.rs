use chrono::{NaiveDateTime, Datelike};
use ethers::types::*;
use rayon::prelude::IntoParallelRefIterator;
use serde_json::json;
use third_parties::mongo::lib::bronze::blocks::types::Block;
use third_parties::mongo::lib::bronze::logs::types::Log;
use third_parties::mongo::lib::bronze::txs::types::Tx;
use rayon::iter::ParallelIterator;

pub async fn get_block_with_txs(provider_uri: String, block_number: &u64 ) -> (Option<Block>, Option<Vec<Tx>>) {
     let client = reqwest::Client::new();
    let num = format!("0x{:x}", &block_number);
    let request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "eth_getBlockByNumber",
        "params": [num, true]
    });

    let string = request.to_string();

    let request = client
        .post(provider_uri)
        .body(string)
        .send()
        .await
        .unwrap();

    let response = request.json::<serde_json::Value>().await.unwrap();
    let block: Block = serde_json::from_value(response["result"].clone()).unwrap();
    let txs: Vec<Tx> = serde_json::from_value(response["result"]["transactions"].clone()).unwrap();

    let txs = txs.par_iter().map(|tx| {
        let mut tx = tx.clone();
        let ts = block.timestamp.clone();
        let datetime = NaiveDateTime::from_timestamp_opt(ts, 0).unwrap();

        tx.timestamp = Some(datetime.timestamp_micros());
        tx.year = Some(datetime.year() as i16);
        tx.month = Some(datetime.month() as i8);
        tx.day = Some(datetime.day() as i8);
        tx
    }).collect::<Vec<Tx>>();

    return (Some(block), Some(txs));
}

pub async fn get_block_logs(uri: String, from_block: &u64, to_block: &u64) -> Option<Vec<Log>> {   
    let filter = Filter {
        address: None,
        topics: [None, None, None, None],
        block_option: {
            FilterBlockOption::Range { from_block: Some(BlockNumber::Number(Into::into(*from_block))), to_block: Some(BlockNumber::Number(Into::into(*to_block))) }
        }
    };

    let client = reqwest::Client::new();
    
    let request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "eth_getLogs",
        "params": [filter]
    });

    let string = request.to_string();

    let request = client
        .post(uri)
        .body(string)
        .send()
        .await
        .unwrap();

    let response = request.json::<serde_json::Value>().await.unwrap();

    let logs: Vec<Log> = serde_json::from_value(response["result"].clone()).unwrap();

    return Some(logs);
}