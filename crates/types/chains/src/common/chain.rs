use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use data_lake_types::{SupportedDataTypes, SupportedDataLevels};
use mongo_types::MongoConfig;
use std::clone::Clone;
use std::fmt::Debug;
// use std::io::Result;
use std::{collections::HashMap, fmt};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum ConnectionType {
    RPC,
    WSS,
}

impl fmt::Display for ConnectionType {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            ConnectionType::RPC => write!(f, "rpc"),
            ConnectionType::WSS => write!(f, "wss"),
        }
    }
}

impl std::str::FromStr for ConnectionType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rpc" => Ok(ConnectionType::RPC),
            "wss" => Ok(ConnectionType::WSS),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Engine {
    EVM,
}

impl fmt::Display for Engine {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        match *self {
            Engine::EVM => write!(f, "EVM"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NativeCurrency {
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
    pub address: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SupportedMethods {
    GetLogs,
    GetBlock,
    GetBlockWithTxs,
    SubscribeLogs,
    SubscribeBlocks,
    SubscribeNewHeads,
    SubscribeTransactions,
}


impl std::str::FromStr for SupportedMethods {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "getLogs" => Ok(SupportedMethods::GetLogs),
            "getBlock" => Ok(SupportedMethods::GetBlock),
            "getBlockWithTxs" => Ok(SupportedMethods::GetBlockWithTxs),
            "subscribeLogs" => Ok(SupportedMethods::SubscribeLogs),
            "subscribeBlocks" => Ok(SupportedMethods::SubscribeBlocks),
            "subscribeNewHeads" => Ok(SupportedMethods::SubscribeNewHeads),
            "subscribeTransactions" => Ok(SupportedMethods::SubscribeTransactions),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChainDetails {
    pub chain_id: String,
    pub name: String,
    pub network: String,
    pub symbol: String,
    pub engine_type: Engine,
    pub native_currency: Vec<NativeCurrency>,
    pub confirmation_time: u64,
    pub db: MongoConfig,
    pub nodes: HashMap<(String, ConnectionType), String>,
    pub rpc_methods: HashMap<SupportedMethods, Value>,
}

pub trait Info {
    fn info(&self) -> ChainDetails;
    fn get_node(&self, provider: &str, connection: &ConnectionType) -> Option<String>;
    fn get_method(&self, method: &SupportedMethods) -> Option<Value>;
    fn get_db(&self) -> MongoConfig;
    fn resolve_collection_name(&self, collection_type: &SupportedDataTypes, collection_level: &SupportedDataLevels) -> String;
}
// subscribe to selected node, listens to new heads and pushes to redis stream
#[async_trait::async_trait]
pub trait SubscribeBlocks {
    async fn subscribe_blocks
    // <T: DeserializeOwned + Unpin + Sync + Send + Serialize + 'static + std::default::Default + Clone>
    (
        &self, 
        redis_uri: String
    )-> std::io::Result<()>;
}

#[async_trait::async_trait]
pub trait IndexFullBlocks {
    async fn index_full_blocks(
        &self,
        confirmed: bool,
        from_block_number: u64,
        to_block_number: Option<u64>,
    ) -> std::io::Result<(Vec<Value>, Vec<Value>, Vec<Value>)>;
}

#[async_trait::async_trait]
pub trait IndexBlocks {
    async fn index_blocks<T: DeserializeOwned + Unpin + Sync + Send + Serialize + 'static + std::default::Default + Clone>(
        &self,
        with_txs: bool,
        from_block_number: u64,
        to_block_number: Option<u64>,
    ) -> std::io::Result<Vec<T>>;
}


#[async_trait::async_trait]
pub trait IndexLogs {
    async fn index_logs<T: DeserializeOwned + Unpin + Sync + Send + Serialize + 'static + std::default::Default + Clone>(
        &self,
        from_block_number: u64,
        to_block_number: Option<u64>,
    ) -> std::io::Result<Vec<T>>;
}

#[async_trait::async_trait]
pub trait DecodeLogs {
    async fn decode_logs(
        &self,
        logs: Vec<Value>,
    ) -> std::io::Result<(Vec<Value>, Vec<Value>)>;
}