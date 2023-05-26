use std::{collections::HashMap, error::Error, pin::Pin};
use ethabi::ethereum_types::H256;
use futures::Future;
use serde::{de::DeserializeOwned, Serialize};

use super::{
    base_chain::{
        Chain, 
        ConnectionType, 
        DecodeLogs, 
        Engine, 
        NativeCurrency, 
        SubscribeBlocks, 
        SubscribeLogs, 
        GetBlocks,
        SupportedMethods,
    },
    types::{
        evm::{
            log::Log, 
            new_heads::{
                NewLogEvent, 
                NewHeadsEvent
            }, 
            block::Block, transaction::Tx,
            generic::GenericNodeResponse
        },
    },
};

use crate::ethereum::{
    utils::{decode_logs_mainnet, decode_block_mainnet},
};
use log::{debug, info};
use serde_json::Value;
use shared_utils::logger::init_logging;
use std::clone::Clone;
use third_parties::mongo::{lib::bronze::decoding_error::types::DecodingError, Mongo, MongoConfig};
use tungstenite::{connect, Message};

use third_parties::mongo::lib::bronze::{
    logs::types::Log as MongoLog,
    blocks::types::Block as MongoBlock
};

#[derive(Debug, Clone)]
pub struct EvmChain {
    pub chain: Chain,
}

impl EvmChain {
    pub async fn new(
        chain_id: String,
        name: String,
        network: String,
        symbol: String,
        engine_type: Engine,
        native_currency: Vec<NativeCurrency>,
        nodes: Vec<(String, ConnectionType, String)>,
        rpc_methods: Vec<(SupportedMethods, Value)>,
        db_config: MongoConfig,
    ) -> Self {
        EvmChain {
            chain: Chain::new(
                chain_id,
                name,
                network,
                symbol,
                engine_type,
                native_currency,
                nodes,
                rpc_methods,
                db_config,
            )
            .await,
        }
    }
}

impl SubscribeLogs for EvmChain {
    fn subscribe_logs<T, R>(&self) {
        let wss_connection = self
            .chain
            .get_node(&"infura".to_string(), &ConnectionType::WSS)
            .expect("No WSS connection found for requested provider");
        println!("{:?}", wss_connection);

        let method = self
            .chain
            .get_method(&SupportedMethods::SubscribeLogs)
            .unwrap();

        let request_str = serde_json::to_string(method).unwrap();

        let (mut socket, _response) = connect(wss_connection).expect("Can't connect");
        socket.write_message(Message::Text(request_str)).unwrap();

        let decode_message = match self.chain.chain_id.as_str() {
            "1" => |msg_str: String| match serde_json::from_str::<NewLogEvent>(&msg_str) {
                Ok(v) => v,
                Err(e) => panic!("{:?}", e),
            },
            _ => panic!("Chain not supported"),
        };

        let decode_logs = match self.chain.chain_id.as_str() {
            "1" => |logs: Vec<Log>, db: Mongo| async move {
                let decoded = decode_logs_mainnet::decode_logs(logs, &db).await.unwrap();
                decoded
            },
            _ => panic!("Chain not supported"),
        };

        let mut last_bn = 0;
        let mut logs_hm: HashMap<i64, Vec<Log>> = HashMap::new();

        loop {
            let db = self.chain.db.clone();
            let msg = socket.read_message().expect("Error reading message");
            let msg_str = msg.into_text().unwrap();

            let data = decode_message(msg_str);
            let log = data.params;

            match log {
                Some(log) => {
                    let data = log.result;
                    match data {
                        Some(data) => {
                            if data.block_number > last_bn {
                                // todo add recursive reading of previous blocks (check keys)
                                let prev_block_data = logs_hm.get(&last_bn);
                                match prev_block_data {
                                    Some(prev_block_data) => {
                                        let bn = last_bn.clone();
                                        let logs = prev_block_data.clone();
                                        logs_hm.remove(&bn);
                                        let chain = self.chain.clone();

                                        tokio::spawn(async move {
                                            let now = std::time::Instant::now();
                                            let decoded = decode_logs(logs, db).await;

                                            // todo dynamic type and collection name
                                            chain
                                                .save_to_db::<MongoLog>(
                                                    decoded.0,
                                                    "logs_bronze_test".to_string(),
                                                )
                                                .await;

                                            chain
                                                .save_to_db::<DecodingError>(
                                                    decoded.1,
                                                    "logs_decoding_error".to_string(),
                                                )
                                                .await;

                                            debug!("Prev block {:?} data decoded", &last_bn);
                                            debug!("Decoding took {:?}", now.elapsed());
                                        });
                                    }
                                    None => {
                                        info!("No prev block data")
                                    }
                                }
                            }
                            last_bn = data.block_number;
                            logs_hm
                                .entry(data.block_number)
                                .or_insert(Vec::new())
                                .push(data.clone());
                        }
                        None => {
                            info!("No log data")
                        }
                    }
                }
                None => {
                    info!("No result data")
                }
            }
        }
    }
}

impl SubscribeBlocks for EvmChain {
    fn subscribe_blocks<T: DeserializeOwned + Unpin + Sync + Send + Serialize + 'static + std::default::Default, R: DeserializeOwned + Unpin + Sync + Send + Serialize>(&self) {
        let wss_connection = self
            .chain
            .get_node(&"infura".to_string(), &ConnectionType::WSS)
            .expect("No WSS connection found for requested provider");
        let method = self
            .chain
            .get_method(&SupportedMethods::SubscribeBlocks)
            .unwrap();

        let request_str = serde_json::to_string(method).unwrap();

        let (mut socket, _response) = connect(wss_connection).expect("Can't connect");
        socket.write_message(Message::Text(request_str)).unwrap();

        let decode_message = match self.chain.chain_id.as_str() {
            "1" => |msg_str: String| match serde_json::from_str::<NewHeadsEvent<T>>(&msg_str) {
                Ok(v) => v,
                Err(e) => panic!("{:?}", e),
            },
            _ => panic!("Chain not supported"),
        };

        let decode_blocks = match self.chain.chain_id.as_str() {
            "1" => |blocks: Vec<Block<T>>| {
                let decoded = decode_block_mainnet::decode_blocks::<T>(blocks).unwrap();
                decoded
            },
            _ => panic!("Chain not supported"),
        };

        loop {
            let msg = socket.read_message().expect("Error reading message");
            let msg_str = msg.into_text().unwrap();
            let data = decode_message(msg_str);
    
            match data.params {
                Some(data) => {
                    match data.result {
                        Some(block) => {
                            let chain = self.chain.clone();
                            tokio::spawn(async move {
                                let decoded = decode_blocks(vec![block]);
                                // todo add to queue in redis
                                chain
                                    .save_to_db::<MongoBlock>(decoded, "blocks_bronze".to_string()).await;
                            });
                        },
                        None => {
                            info!("No block data")
                        }
                    }
                },  
                None => {
                    info!("No result data")
                }
            }
        }
    }
}


// R: Raw Tx from response
// T: Block expected 
// Y: Raw Block with/out txs
impl GetBlocks for EvmChain {
    fn get_blocks<Y: DeserializeOwned + Unpin + Sync + Send + Serialize , T: DeserializeOwned + Unpin + Sync + Send + Serialize, R>(
        &self, 
        from_block_number: u64, 
        to_block_number: u64, 
        with_txs: bool
    ) -> std::io::Result<Vec<T>> {
        
        let client = reqwest::Client::new();

        let connection = self.chain.get_node(&"infura".to_string(), &ConnectionType::RPC).unwrap();
        
        let method = match with_txs {
            true => self.chain.get_method(&SupportedMethods::GetBlockWithTxs).unwrap(),
            false => self.chain.get_method(&SupportedMethods::GetBlock).unwrap(),
        };

        futures::executor::block_on(async move {
            let mut blocks_data: Vec<T> = vec![];
            for block_number in from_block_number..=to_block_number {
                let block_hex = format!("0x{:x}", block_number);
                let method = serde_json::to_string(method).unwrap();
                let query = method.replace("__insert_block_number__", &block_hex);

                let request = client
                    .post(connection)
                    .body(query)
                    .send()
                    .await
                    .unwrap();

                let response = request.text().await.unwrap();

                match with_txs {
                    true => {
                        let data: GenericNodeResponse<T> = serde_json::from_str(&response).unwrap();
                        blocks_data.push(data.result);
                    },
                    false => {
                        let data: GenericNodeResponse<T> = serde_json::from_str(&response).unwrap();
                        blocks_data.push(data.result);
                    }
                }
            }
            Ok(blocks_data)
        })
    }
}