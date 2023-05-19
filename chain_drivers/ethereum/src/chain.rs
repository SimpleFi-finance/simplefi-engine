use common::types::chain::{Chain, NativeCurrency, ConnectionType, SupportedMethods};
use settings::load_settings;
use third_parties::mongo::MongoConfig;

#[tokio::main]
pub async fn ethereum_mainnet() -> Chain {

    // load settings for ethereum
    // let settings = load_settings().unwrap_or_default();
    let settings = load_settings().unwrap();


    let nodes =  vec![
        ("infura".to_string(), ConnectionType::RPC, format!("{}/{}", settings.infura_mainnet_rpc, settings.infura_token)),
        ("infura".to_string(), ConnectionType::WSS, format!("{}/{}", settings.infura_mainnet_ws, settings.infura_token)),
    ];
    println!("nodes: {:?}", nodes);
    let rpc_methods = vec![(
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

    let eth = NativeCurrency {
        name: "Ether".to_string(),
        symbol: "ETH".to_string(),
        decimals: 18,
        address: "0x0000000000000000000000000000000000000000".to_string(),
    };

    // todo remove hardcoding
    let db = MongoConfig {
        uri: "mongodb://localhost:27017".to_string(),
        database: "test".to_string(),
    };

    Chain::new(
        "1".to_string(), 
        "Ethereum Mainnet".to_string(), 
        "mainnet".to_string(),
        "ETH".to_string(), 
        common::types::chain::Engine::EVM, 
        vec![eth],
        nodes,
        rpc_methods,
        db,
    ).await
}

// decoding methods (evm?)

// db methods

// datalake methods