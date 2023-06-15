use std::fmt;

use futures::executor::block_on;
use serde_json::Value;

use crate::{
    ethereum::mainnet::{
        rpc_methods as ethereum_rpc_methods, 
        nodes as ethereum_nodes, subscribe_blocks
    },
    types::chain::{
        ChainDetails, ChainMethods, ChainNodes, Engine, Info, NativeCurrency, SupportedMethods, ConnectionType, SubscribeBlocks, IndexBlocks,
    },
};

pub enum SupportedChains {
    EthereumMainnet,
}

impl std::fmt::Display for SupportedChains {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            SupportedChains::EthereumMainnet => write!(f, "ethereum_mainnet"),
        }
    }
}

impl Info for SupportedChains {
    fn info(&self) -> ChainDetails {
        match self {
            SupportedChains::EthereumMainnet => ChainDetails {
                chain_id: "1".to_string(),
                name: "Ethereum Mainnet".to_string(),
                symbol: "ETH".to_string(),
                confirmation_time: 13,
                native_currency: vec![NativeCurrency {
                    name: "Ether".to_string(),
                    symbol: "ETH".to_string(),
                    decimals: 18,
                    address: "0x0000000000".to_string(),
                }],
                network: "mainnet".to_string(),
                engine_type: Engine::EVM,
                nodes: ethereum_nodes(),
                rpc_methods: ethereum_rpc_methods(),
            },
        }
    }
}

impl ChainNodes for SupportedChains {
    fn get_node(
        &self,
        provider: &str,
        connection: &ConnectionType,
    ) -> Option<String> {
        self.info().nodes.get(&(provider.to_string(), connection.clone())).cloned()
    }
}

impl ChainMethods for SupportedChains {
    fn get_method(
        &self,
        method: &SupportedMethods,
    ) -> Option<Value> {
        self.info().rpc_methods.get(&method).cloned()
    }
}

impl SubscribeBlocks for SupportedChains {
    fn subscribe_blocks<T: serde::de::DeserializeOwned + Unpin + Sync + Send + serde::Serialize + 'static + std::default::Default + Clone>(
        &self, 
        redis_uri: String
    ) {
        match self {
            SupportedChains::EthereumMainnet => {
                let rpc_method = self.get_method(&SupportedMethods::SubscribeNewHeads);
                let rpc_method = match rpc_method {
                    Some(rpc_method) => {
                        rpc_method    
                    },
                    _ => {
                        panic!("No rpc method found for chain to listen to new heads: {}", self.info().name);
                    }
                };

                // todo load provider name from config

                let rpc_node = self.get_node("infura", &ConnectionType::WSS);
                match rpc_node {
                    Some(rpc_node) => {
                        block_on(async {
                            subscribe_blocks(redis_uri, rpc_method, rpc_node).await;
                        });
                    },
                    None => {
                        panic!("No wss node found for chain: {}", self.info().name);
                    }
                }
            }
        }
    }
}

impl IndexBlocks for SupportedChains {
    fn index_blocks<
        T: serde::de::DeserializeOwned + Unpin + Sync + Send + serde::Serialize + 'static + std::default::Default + Clone
    >(
        &self,
        from_block_number: u64,
        to_block_number: u64,
        with_txs: bool,
    ) -> std::io::Result<Vec<T>> {
        match self {
            SupportedChains::EthereumMainnet => {
                // returns struct of block {logs, txs}
                Ok(vec![])
            }
        }
    }
}

// decode logs

// decode txs

// decode blocks

pub fn get_chain(
    chain_id: &str,
) -> Option<SupportedChains> {
    match chain_id {
        "1" => Some(SupportedChains::EthereumMainnet),
        _ => None,
    }
}
