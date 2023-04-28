use ethers::types::*;
use serde_json::json;
use shared_types::chains::evm::{tx::Tx, block::Block, log::Log};

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
        .post(provider_uri.clone())
        .body(string)
        .send()
        .await
        .unwrap();

    let response = request.text().await.unwrap();

    let response: serde_json::Value = serde_json::from_str(&response).unwrap();
    // let response = request.json::<serde_json::Value>().await.unwrap();
    let block: Block = serde_json::from_value(response["result"].clone()).unwrap();
    let txs: Vec<Tx> = serde_json::from_value(response["result"]["transactions"].clone()).unwrap();

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

    let response = request.text().await.unwrap();
    let response: serde_json::Value = serde_json::from_str(&response).unwrap();

    let logs: Vec<Log> = serde_json::from_value(response["result"].clone()).unwrap();

    return Some(logs);
}