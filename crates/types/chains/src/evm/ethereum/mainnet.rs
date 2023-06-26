use std::collections::HashMap;
use serde_json::Value;
use simplefi_engine_settings::load_settings;

use crate::common::chain::{SupportedMethods, ConnectionType};

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