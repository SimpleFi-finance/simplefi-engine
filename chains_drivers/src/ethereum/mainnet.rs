use std::collections::HashMap;
use log::info;
use serde_json::Value;
use settings::load_settings;
use third_parties::redis::{publish_message, connect as redis_connect};
use tungstenite::connect;

use crate::{types::{chain::{SupportedMethods, ConnectionType, Info}, evm::{new_heads::NewHeadsEvent, transaction::Tx}}, chains::get_chain};


pub fn rpc_methods() -> HashMap<SupportedMethods, Value> {

    let mut methods = HashMap::new();

    let rpc_methdos = vec![(
        SupportedMethods::GetLogs,
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_getLogs",
            "params": [{
                "fromBlock": "__insert_from_block_number__",
                "toBlock": "__insert_to_block_number__",
            }],
        })
    ), (
        SupportedMethods::GetBlock,
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_getBlockByNumber",
            "params": ["__insert_block_number__", false],
        })
    ),(
        SupportedMethods::GetBlockWithTxs,
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_getBlockByNumber",
            "params": ["__insert_block_number__", true],
        })
    ),(
        SupportedMethods::SubscribeLogs,
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_subscribe",
            "params": ["logs", {}]
        })
    ), (
        SupportedMethods::SubscribeNewHeads,
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_subscribe",
            "params": ["newHeads", {}]
        })
    )];

    for (method, value) in rpc_methdos {
        methods.insert(method, value);
    }

    methods

}

pub fn nodes() -> HashMap<(String, ConnectionType), String> {

    let settings = load_settings().unwrap();

    let nodes = vec![
        ("infura".to_string(), ConnectionType::RPC, format!("{}{}", settings.infura_mainnet_rpc, settings.infura_token)),
        ("infura".to_string(), ConnectionType::WSS, format!("{}{}", settings.infura_mainnet_ws, settings.infura_token)),
    ];

    let mut nodes_hm = HashMap::new();

    for (name, connection_type, url) in nodes {
        nodes_hm.insert((name, connection_type), url);
    }

    nodes_hm
}

pub async fn subscribe_blocks(redis_uri: String, rpc_method: Value, rpc_node: String) {
    // todo save to mongo db
    let request_str = serde_json::to_string(&rpc_method).unwrap();

    let (mut socket, _response) = connect(&rpc_node)
        .expect("can't connect to wss node");
    socket.write_message(tungstenite::Message::Text(request_str)).unwrap();

    let chain = get_chain("1")
        .unwrap();

    loop {
        let msg = socket.read_message().unwrap();
        let msg_str = msg.into_text().unwrap();
        let decoded_msg = match serde_json::from_str::<NewHeadsEvent<Tx>>(&msg_str) {
            Ok(decoded) => decoded,
            Err(e) => panic!("{:?}", e),
        };

        match decoded_msg.params {
            Some(data) => match data.result {
                Some(block) => {
                    let mut redis_conn = redis_connect(&redis_uri).await.unwrap();
                    let bn = block.number.clone().to_string();

                    let redis_channel = format!(
                        "{}_{}",
                        &chain.info().symbol.to_lowercase(),
                        "blocks".to_string()
                    );
                    // todo convert from pub/sub to redis stream
                    publish_message(&mut redis_conn, &redis_channel, &bn)
                        .await
                        .unwrap();
                }
                None => info!("No block data")
            },
            None => info!("No result data")
        }
    }
}

pub async fn index_blocks() {}

pub async fn index_logs() {}