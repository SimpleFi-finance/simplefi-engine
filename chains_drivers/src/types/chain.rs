use futures::{TryStreamExt};
use mongodb::bson::{doc, Document};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use shared_types::data_lake::{SupportedDataLevels, SupportedDataTypes};
use std::clone::Clone;
use std::fmt::Debug;
use std::io::Result;
use std::{collections::HashMap, fmt};
use third_parties::mongo::{Mongo, MongoConfig};

use super::base::{EntityBlockNumber, EntityContractAddress, RawToValue, EntityTimestamp};


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

#[derive(Debug, Clone)]
pub struct ChainDetails {
    pub chain_id: String,
    pub name: String,
    pub network: String,
    pub symbol: String,
    pub engine_type: Engine,
    pub native_currency: Vec<NativeCurrency>,
    pub confirmation_time: u64,
    // pub db: Mongo,
    pub nodes: HashMap<(String, ConnectionType), String>,
    pub rpc_methods: HashMap<SupportedMethods, Value>,
}

pub trait Info {
    fn info(&self) -> ChainDetails;
}

pub trait ChainNodes {
    fn get_node(&self, provider: &str, connection: &ConnectionType) -> Option<String>;
}

pub trait ChainMethods {
    fn get_method(&self, method: &SupportedMethods) -> Option<Value>;
}

pub trait ChainDB {
    fn db(&self) -> Mongo;
}

// subscribe to selected node, listens to new heads and pushes to redis stream
pub trait SubscribeBlocks {
    fn subscribe_blocks
    // <T: DeserializeOwned + Unpin + Sync + Send + Serialize + 'static + std::default::Default + Clone>
    (
        &self, 
        redis_uri: String
    );
}

pub trait IndexFullBlocks {
    fn index_full_blocks(
        &self,
        confirmed: bool,
        from_block_number: u64,
        to_block_number: Option<u64>,
    ) -> Result<(Vec<Value>, Vec<Value>, Vec<Value>)>;
}

pub trait IndexBlocks {
    fn index_blocks<T: DeserializeOwned + Unpin + Sync + Send + Serialize + 'static + std::default::Default + Clone>(
        &self,
        with_txs: bool,
        from_block_number: u64,
        to_block_number: Option<u64>,
    ) -> Result<Vec<T>>;
}

pub trait IndexLogs {
    fn index_logs<T: DeserializeOwned + Unpin + Sync + Send + Serialize + 'static + std::default::Default + Clone>(
        &self,
        from_block_number: u64,
        to_block_number: Option<u64>,
    ) -> Result<Vec<T>>;
}


