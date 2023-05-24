use std::{collections::HashMap, sync::Arc};

use serde_json::Value;
use log::{ debug, info };
use third_parties::mongo::{
    MongoConfig, 
    Mongo,
    lib::bronze::{logs::types::Log, decoding_error::types::DecodingError},
};
use std::clone::Clone;
use tungstenite::{connect, Message};
use crate::ethereum::{
    types::raw::{ new_heads::NewLogEvent, log::EthLog},
    utils::decode_logs_mainnet,
};
use super::base_chain::{Chain, DecodeLogs, SubscribeLogs, SupportedMethods, ConnectionType, SubscribeBlocks, NativeCurrency, Engine};
use shared_utils::logger::init_logging;

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
            ).await
        }
    }
}

impl DecodeLogs for EvmChain {
    fn decode_logs<T,R>(&self, items: Vec<T>) -> Vec<R> {
        // todo add logs decoding functionality
        vec![]
    }
}

impl SubscribeLogs for EvmChain {
    fn subscribe_logs<T,R>(&self) {
        init_logging();

        let wss_connection = self.chain.get_node(&"infura".to_string(), &ConnectionType::WSS).expect("No WSS connection found for requested provider");
        println!("{:?}", wss_connection);

        let method = self.chain.get_method(&SupportedMethods::SubscribeLogs).unwrap();
       
        let request_str = serde_json::to_string(method).unwrap();

        let (mut socket, _response) = connect(wss_connection).expect("Can't connect");
        socket.write_message(Message::Text(request_str)).unwrap();

        let decode_message = match self.chain.chain_id.as_str() {
            "1" => |msg_str: String| {
                match serde_json::from_str::<NewLogEvent>(&msg_str) {
                    Ok(v) => v,
                    Err(e) => panic!("{:?}", e)
                }
            },
            _ => panic!("Chain not supported"),
        };

        let decode_logs = match self.chain.chain_id.as_str() {
            "1" => |logs: Vec<EthLog>, db: Mongo| async move {
                let decoded = decode_logs_mainnet::decode_logs(logs, &db).await.unwrap();
                decoded
            },
            _ => panic!("Chain not supported"),
        };

        let mut last_bn = 0;
        let mut logs_hm: HashMap<i64, Vec<EthLog>> = HashMap::new();

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
                                                .save_to_db::<Log>(decoded.0, "logs_bronze_test".to_string()).await;

                                            chain
                                                .save_to_db::<DecodingError>( decoded.1, "logs_decoding_error".to_string()).await;
    
                                            debug!("Prev block {:?} data decoded", &last_bn);
                                            debug!("Decoding took {:?}", now.elapsed());
                                        });
                                    }
                                    None => {info!("No prev block data")}
                                }
                                
                            }
                            last_bn = data.block_number;
                            logs_hm.entry(data.block_number).or_insert(Vec::new()).push(data.clone());
                        }
                        None => {info!("No log data")}
                    }
                }
                None => {info!("No result data")}
            }
        }
    }
}

impl SubscribeBlocks for EvmChain {
    fn subscribe_blocks<T,R>(&self) {
        let wss_connection = self.chain.get_node(&"infura".to_string(), &ConnectionType::WSS).expect("No WSS connection found for requested provider");
        let method = self.chain.get_method(&SupportedMethods::SubscribeBlocks).unwrap();
       
        let request_str = serde_json::to_string(method).unwrap();

        let (mut socket, _response) = connect(wss_connection).expect("Can't connect");
        socket.write_message(Message::Text(request_str)).unwrap();

        loop {
            let msg = socket.read_message().expect("Error reading message");
            let msg_str = msg.into_text().unwrap();

            // let msg_data = // use match to parse message into correct format
            // save logic to save in db using chain data
            println!("Message: {}", msg_str);
        }
    }
}
