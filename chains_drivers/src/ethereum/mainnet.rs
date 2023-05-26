use settings::load_settings;
use third_parties::mongo::MongoConfig;

use crate::common::{base_chain::{ConnectionType, SupportedMethods, NativeCurrency, Engine}, evm::EvmChain};

pub async fn ethereum_mainnet() -> Result<EvmChain, Box<dyn std::error::Error>> {

    // todo load settings specific for ethereum
    let settings = load_settings().unwrap();

    let nodes =  vec![
        ("infura".to_string(), ConnectionType::RPC, format!("{}{}", settings.infura_mainnet_rpc, settings.infura_token)),
        ("infura".to_string(), ConnectionType::WSS, format!("{}{}", settings.infura_mainnet_ws, settings.infura_token)),
    ];

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

    let eth = NativeCurrency {
        name: "Ether".to_string(),
        symbol: "ETH".to_string(),
        decimals: 18,
        address: "0x0000000000000000000000000000000000000000".to_string(),
    };

    let db = MongoConfig {
        uri: settings.mongodb_uri,
        database: settings.mongodb_database_name,
    };

    Ok(
        EvmChain::new(
            "1".to_string(), 
            "Ethereum Mainnet".to_string(), 
            "mainnet".to_string(),
            "ETH".to_string(), 
            Engine::EVM, 
            vec![eth],
            nodes,
            rpc_methods,
            db,
        ).await
    )
}