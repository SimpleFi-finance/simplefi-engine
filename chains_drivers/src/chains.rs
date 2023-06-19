use std::fmt;
use log::info;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use redis::{Client, cmd};
use serde_json::Value;
// use third_parties::{mongo::lib::bronze::logs};
use tungstenite::connect;

use crate::{
    ethereum::mainnet::{
        rpc_methods as ethereum_rpc_methods, 
        nodes as ethereum_nodes
    },
    types::{
        chain::{
            ChainDetails, ChainMethods, ChainNodes, Engine, Info, NativeCurrency, SupportedMethods, ConnectionType, SubscribeBlocks, IndexBlocks, IndexLogs, IndexFullBlocks,
        },
        evm::{
            chain_log::Log, generic::GenericNodeResponse, block::Block, transaction::Tx, new_heads::NewHeadsEvent
        }, base::RawToValue
    }
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

#[async_trait::async_trait]
impl IndexBlocks for SupportedChains {
    async fn index_blocks<T: serde::de::DeserializeOwned + Unpin + Sync + Send + serde::Serialize + 'static + std::default::Default + Clone>(
        &self,
        with_txs: bool,
        from_block_number: u64,
        to_block_number: Option<u64>,
    ) -> std::io::Result<Vec<T>> {
        let client = reqwest::Client::new();
        // should return a flavour of a block with or without txs:  Block<Vec<Value>> where Value is json!(tx_with_ts) or None 
        match self {
            SupportedChains::EthereumMainnet => {   
                let node = self.get_node("infura", &ConnectionType::RPC).unwrap();

                let mut blocks_data = vec![];

                for block_number in from_block_number..=to_block_number.unwrap_or(from_block_number) {
                    let method = match with_txs {
                        true => self.get_method(&SupportedMethods::GetBlockWithTxs).unwrap(),
                        false => self.get_method(&SupportedMethods::GetBlock).unwrap(),
                    };
                    
                    let method = serde_json::to_string(&method).unwrap();

                    let query = method.replace("__insert_block_number__", &format!("0x{:x}", block_number));

                    let request = client.post(node.clone()).body(query).send().await.unwrap();

                    let response = request.text().await.unwrap();
                    let data: GenericNodeResponse<T> = serde_json::from_str(&response).unwrap();

                    blocks_data.push(data.result);
                }
            
                Ok(blocks_data)
            }
        }
    }
}

#[async_trait::async_trait]
impl IndexLogs for SupportedChains {
    async fn index_logs<T: serde::de::DeserializeOwned + Unpin + Sync + Send + serde::Serialize + 'static + std::default::Default + Clone>(
        &self,
        from_block_number: u64,
        to_block_number: Option<u64>,
    ) -> std::io::Result<Vec<T>> {
        let client = reqwest::Client::new();
        // todo add filters
        match self {
            SupportedChains::EthereumMainnet => {
                let node = self.get_node("infura", &ConnectionType::RPC).unwrap();

                let mut logs_data: Vec<T> = vec![];

                for bn in from_block_number..=to_block_number.unwrap_or(from_block_number) {
                    let get_logs_method = serde_json::to_string(&self.get_method(&SupportedMethods::GetLogs)
                        .unwrap())
                    .unwrap();

                    let get_logs = get_logs_method
                        .replace("__insert_from_block_number__", &format!("0x{:x}", bn))
                        .replace("__insert_to_block_number__", &format!("0x{:x}", bn));

                    // todo handle errors
                    let request = client.post(node.clone()).body(get_logs).send().await.unwrap();
                    let data = request.text().await.unwrap();

                    let logs: GenericNodeResponse<Vec<T>> = serde_json::from_str(&data).unwrap();

                    logs_data.extend(logs.result);
                }
                Ok(logs_data)
            }
        }
    }
}

#[async_trait::async_trait]
impl SubscribeBlocks for SupportedChains {
    async fn subscribe_blocks(
        &self, 
        redis_uri: String
    )-> std::io::Result<()> {
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
                        let request_str = serde_json::to_string(&rpc_method).unwrap();

                        let (mut socket, _response) = connect(&rpc_node)
                            .expect("can't connect to wss node");

                        socket.write_message(tungstenite::Message::Text(request_str)).unwrap();

                        let chain = get_chain("1")
                            .unwrap();

                        let stream_name = format!("{}_blocks", &chain.info().symbol.to_lowercase());

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
                                        let redis_cli = Client::open(redis_uri.clone())
                                            .unwrap();
                                        let mut redis_conn = redis_cli.get_connection().unwrap();

                                        let bn = block.number.clone().to_string();
                                        
                                        let kv = [("block_number", bn.clone().to_string())];
                                        
                                        cmd("XADD")
                                            .arg(stream_name.clone())
                                            .arg("*")
                                            .arg(&kv)
                                            .execute(&mut redis_conn);
                                    }
                                    None => {
                                        info!("No block data");
                                    }
                        
                                },
                                None => {info!("No result data");
                                }
                            }
                        }
                    },
                    None => {
                        panic!("No wss node found for chain: {}", self.info().name);
                    }
                }
            }
        }
    }
}


#[async_trait::async_trait]
impl IndexFullBlocks for SupportedChains {
    async fn index_full_blocks(
        &self,
        confirmed: bool,
        from_block_number: u64,
        to_block_number: Option<u64>,
    ) -> std::io::Result<(Vec<Value>, Vec<Value>, Vec<Value>)> {
        match self {
            SupportedChains::EthereumMainnet => {
                if to_block_number.is_some() {
                    if to_block_number.unwrap() < from_block_number {
                        panic!("to_block_number must be greater than from_block_number");
                    }
                }

                let mut start_bn = from_block_number;
                let mut end_bn = to_block_number;

                if confirmed {
                    start_bn = from_block_number - self.info().confirmation_time;
                    end_bn = match to_block_number {
                        Some(bn) => Some(bn - self.info().confirmation_time),
                        None => None
                    }
                }

                let (blocks, logs) = tokio::join!(
                    self.index_blocks::<Block<Tx>>(true, start_bn, end_bn),
                    self.index_logs::<Log>(start_bn, end_bn)
                );

                let mut final_logs = vec![];
                let mut final_txs = vec![];
                let mut final_blocks = vec![];

                let logs = logs.unwrap();

                for bn in blocks.unwrap() {
                    let block = bn.clone().raw_to_value(0);

                    let logs_in_bn = logs.par_iter().filter(|log| log.block_number == bn.number).map(|l| {
                        l.raw_to_value(bn.timestamp)
                    }).collect::<Vec<Value>>();

                    let txs = bn.transactions.unwrap().par_iter().map(|tx| {
                        tx.raw_to_value(bn.timestamp)
                    }).collect::<Vec<Value>>();

                    final_blocks.push(block);
                    final_txs.extend(txs);
                    final_logs.extend(logs_in_bn);
                }

                Ok((final_blocks, final_txs, final_logs))
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
