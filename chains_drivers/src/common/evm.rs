use super::{
    base_chain::{
        Chain, ConnectionType, Engine, GetBlocks, GetLogs, NativeCurrency, SubscribeBlocks,
        SubscribeLogs, SupportedMethods,
    },
    types::evm::{
        block::Block,
        generic::GenericNodeResponse,
        log::{LogBlockNumber, LogContractAddress, RawToMongo},
        new_heads::{NewHeadsEvent, NewLogEvent},
    },
    utils::decoding::logs::evm::evm_logs_decoder,
};
use grpc_server::client::AbiDiscoveryClient;
use mongodb::bson::doc;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde::{de::DeserializeOwned, Serialize};
use shared_types::data_lake::{SupportedDataLevels, SupportedDataTypes};
use std::{collections::HashMap, fmt::Debug};

use log::{debug, info};
use serde_json::Value;
use std::clone::Clone;

use bronze::mongo::{
    evm::{
        types::logs::Log as MongoLog,
        types::blocks::Block as MongoBlock,
    },
    common::types::decoding_errors::DecodingError
};

use third_parties::{
    mongo::MongoConfig,
    redis::{connect as redis_connect, publish_message},
};
use tungstenite::{connect, Message};

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
        confirmation_time: u64,
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
                confirmation_time,
            )
            .await,
        }
    }

    async fn get_abis(
        &self,
        contract_addresses: Vec<String>,
    ) -> Result<Vec<grpc_server::abi_discovery_proto::ContractInfo>, Box<dyn std::error::Error>>
    {
        let mut abi_discovery_client =
            AbiDiscoveryClient::new("http://[::1]:50051".to_string()).await;

        // TODO: Add chain as parameter
        let chain = "ethereum".to_string();

        let response = abi_discovery_client.get_contracts_info_handler(chain, contract_addresses).await;

        let response_data = response.into_inner();

        Ok(response_data.contracts_info)
    }

    pub async fn decode_logs<T: LogBlockNumber + LogContractAddress + RawToMongo + Sync>(
        &self,
        logs: Vec<T>,
        timestamp: i64,
    ) -> Result<(Vec<MongoLog>, Vec<DecodingError>), Box<dyn std::error::Error>> {
        let logs_by_address = logs
            .par_iter()
            .fold(
                || HashMap::new(),
                |mut acc, log| {
                    let log = log.raw_to_mongo(timestamp);

                    acc.entry(log.address.clone().unwrap())
                        .or_insert(vec![])
                        .push(log.clone());

                    acc
                },
            )
            .reduce(
                || HashMap::new(),
                |mut acc, hm| {
                    for (key, value) in hm {
                        acc.entry(key).or_insert(vec![]).extend(value);
                    }
                    acc
                },
            );

        let unique_addresses: Vec<String> = logs_by_address.keys().cloned().collect();

        let abis = self.get_abis(unique_addresses).await?;

        let decoded_logs = evm_logs_decoder(logs_by_address, abis).unwrap();
        Ok(decoded_logs)
    }
}

impl SubscribeLogs for EvmChain {
    fn subscribe_logs<
        T: LogBlockNumber
            + LogContractAddress
            + RawToMongo
            + DeserializeOwned
            + Serialize
            + Clone
            + Send
            + Sync
            + 'static,
    >(
        &self
    ) {
        let wss_connection = self
            .chain
            .get_node(&"infura".to_string(), &ConnectionType::WSS)
            .expect("No WSS connection found for requested provider");

        let method = self
            .chain
            .get_method(&SupportedMethods::SubscribeLogs)
            .unwrap();

        let request_str = serde_json::to_string(method).unwrap();

        let (mut socket, _response) = connect(wss_connection).expect("Can't connect");
        socket.write_message(Message::Text(request_str)).unwrap();

        let decode_message = match self.chain.chain_id.as_str() {
            "1" => |msg_str: String| match serde_json::from_str::<NewLogEvent<T>>(&msg_str) {
                Ok(v) => v,
                Err(e) => panic!("{:?}", e),
            },
            _ => panic!("Chain not supported"),
        };

        let decode_logs = match self.chain.chain_id.as_str() {
            "1" => |logs: Vec<T>, chain: EvmChain| async move {
                // get ts from block

                let filter = doc! {"number": logs[0].block_number()};
                let block = chain
                    .chain
                    .get_items::<MongoBlock>(
                        &SupportedDataTypes::Blocks,
                        &SupportedDataLevels::Bronze,
                        Some(filter),
                    )
                    .await;
                let timestamp = match block.len() > 0 {
                    true => block[0].timestamp,
                    false => 0 as i64,
                };

                let decoded = chain.decode_logs(logs, timestamp).await.unwrap();
                decoded
            },
            _ => panic!("Chain not supported"),
        };

        let mut last_bn = 0;
        let mut logs_hm: HashMap<i64, Vec<T>> = HashMap::new();

        loop {
            let evm_chain = self.clone();
            let chain = self.chain.clone();

            let msg = socket.read_message().expect("Error reading message");
            let msg_str = msg.into_text().unwrap();

            let data = decode_message(msg_str);
            let log = data.params;

            match log {
                Some(log) => {
                    let data = log.result;
                    match data {
                        Some(data) => {
                            if data.block_number() > last_bn {
                                // todo add recursive reading of previous blocks (check keys)
                                let prev_block_data = logs_hm.get(&last_bn);
                                match prev_block_data {
                                    Some(prev_block_data) => {
                                        let bn = last_bn.clone();
                                        let logs = prev_block_data.clone();
                                        logs_hm.remove(&bn);

                                        tokio::spawn(async move {
                                            let now = std::time::Instant::now();
                                            let decoded = decode_logs(logs, evm_chain).await;

                                            chain
                                                .save_to_db::<MongoLog>(
                                                    decoded.0,
                                                    &SupportedDataTypes::Logs,
                                                    &SupportedDataLevels::Bronze,
                                                )
                                                .await;

                                            chain
                                                .save_to_db::<DecodingError>(
                                                    decoded.1,
                                                    &SupportedDataTypes::DecodingError,
                                                    &SupportedDataLevels::Bronze,
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
                            last_bn = data.block_number();
                            logs_hm
                                .entry(data.block_number())
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
    fn subscribe_blocks<
        T: DeserializeOwned + Unpin + Sync + Send + Serialize + 'static + std::default::Default,
    >(
        &self,
        redis_uri: String,
    ) {
        let wss_connection = self
            .chain
            .get_node(&"infura".to_string(), &ConnectionType::WSS)
            .expect("No WSS connection found for requested provider");
        let method = self
            .chain
            .get_method(&SupportedMethods::SubscribeNewHeads)
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
                blocks
                    .par_iter()
                    .map(|bl| bl.raw_to_mongo())
                    .collect::<Vec<MongoBlock>>()
            },
            _ => panic!("Chain not supported"),
        };

        loop {
            let msg = socket.read_message().expect("Error reading message");
            let msg_str = msg.into_text().unwrap();
            let data = decode_message(msg_str);
            let redis_uri = redis_uri.clone();

            match data.params {
                Some(data) => match data.result {
                    Some(block) => {
                        let chain = self.chain.clone();
                        tokio::spawn(async move {
                            let mut redis_conn = redis_connect(&redis_uri).await.unwrap();
                            let decoded = decode_blocks(vec![block]);

                            let bn = decoded[0].number.clone().to_string();

                            chain
                                .save_to_db::<MongoBlock>(
                                    decoded,
                                    &SupportedDataTypes::Blocks,
                                    &SupportedDataLevels::Bronze,
                                )
                                .await;

                            let redis_channel = format!(
                                "{}_{}",
                                &chain.symbol.to_lowercase(),
                                "blocks".to_string()
                            );

                            publish_message(&mut redis_conn, &redis_channel, &bn)
                                .await
                                .unwrap();
                        });
                    }
                    None => {
                        info!("No block data")
                    }
                },
                None => {
                    info!("No result data")
                }
            }
        }
    }
}

impl GetBlocks for EvmChain {
    fn get_blocks<Y: DeserializeOwned + Unpin + Sync + Send + Serialize>(
        &self,
        from_block_number: u64,
        to_block_number: u64,
        with_txs: bool,
    ) -> std::io::Result<Vec<Y>> {
        let client = reqwest::Client::new();

        let connection = self
            .chain
            .get_node(&"infura".to_string(), &ConnectionType::RPC)
            .unwrap();

        let method = match with_txs {
            true => self
                .chain
                .get_method(&SupportedMethods::GetBlockWithTxs)
                .unwrap(),
            false => self.chain.get_method(&SupportedMethods::GetBlock).unwrap(),
        };

        futures::executor::block_on(async move {
            let mut blocks_data: Vec<Y> = vec![];
            for block_number in from_block_number..=to_block_number {
                let block_hex = format!("0x{:x}", block_number);
                let method = serde_json::to_string(method).unwrap();
                let query = method.replace("__insert_block_number__", &block_hex);

                let request = client.post(connection).body(query).send().await.unwrap();

                let response = request.text().await.unwrap();

                match with_txs {
                    true => {
                        let data: GenericNodeResponse<Y> = serde_json::from_str(&response).unwrap();
                        blocks_data.push(data.result);
                    }
                    false => {
                        let data: GenericNodeResponse<Y> = serde_json::from_str(&response).unwrap();
                        blocks_data.push(data.result);
                    }
                }
            }
            Ok(blocks_data)
        })
    }
}

impl GetLogs for EvmChain {
    fn get_logs<T: DeserializeOwned + Unpin + Sync + Send + Debug + Serialize>(
        &self,
        from_block_number: u64,
        to_block_number: u64,
    ) -> std::io::Result<Vec<T>> {
        let client = reqwest::Client::new();

        let connection = self
            .chain
            .get_node(&"infura".to_string(), &ConnectionType::RPC)
            .unwrap();

        let method = self.chain.get_method(&SupportedMethods::GetLogs).unwrap();

        futures::executor::block_on(async move {
            let mut logs_data: Vec<T> = vec![];

            for block_number in from_block_number..=to_block_number {
                let block_hex = format!("0x{:x}", block_number);
                let method = serde_json::to_string(method).unwrap();
                let query = method
                    .replace("__insert_from_block_number__", &block_hex)
                    .replace("__insert_to_block_number__", &block_hex);

                let request = client.post(connection).body(query).send().await.unwrap();

                let response = request.text().await.unwrap();
                let data: GenericNodeResponse<Vec<T>> = serde_json::from_str(&response).unwrap();

                logs_data.extend(data.result);
            }
            Ok(logs_data)
        })
    }
}
