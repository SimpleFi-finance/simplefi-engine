use std::collections::HashMap;
use serde_json::Value;
use simp_settings::load_settings;

pub fn rpc_methods() -> HashMap<String, Value> {

    let mut methods = HashMap::new();

    let rpc_methdos = vec![(
        "getLogs".to_string(),
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
        "getBlock".to_string(),
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_getBlockByNumber",
            "params": ["__insert_block_number__", false],
        })
    ),(
        "getBlockWithTxs".to_string(),
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_getBlockByNumber",
            "params": ["__insert_block_number__", true],
        })
    ),(
        "subscribeLogs".to_string(),
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "eth_subscribe",
            "params": ["logs", {}]
        })
    ), (
        "subscribeNewHeads".to_string(),
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

pub fn nodes() -> HashMap<(String, String), String> {

    let settings = load_settings().unwrap();

    let nodes = vec![];

    let mut nodes_hm = HashMap::new();

    for (name, connection_type, url) in nodes {
        nodes_hm.insert((name, connection_type), url);
    }

    nodes_hm
}