use futures::{TryStreamExt, Future};
use mongodb::bson::doc;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use shared_types::data_lake::{SupportedDataLevels, SupportedDataTypes};
use std::clone::Clone;
use std::fmt::Debug;
use std::io::Result;
use std::{collections::HashMap, fmt};
use third_parties::mongo::{Mongo, MongoConfig};

pub enum SupportedChains {
    EthereumMainnet,
}

impl fmt::Display for SupportedChains {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            SupportedChains::EthereumMainnet => write!(f, "ethereum_mainnet"),
        }
    }
}

impl fmt::Debug for SupportedChains {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            SupportedChains::EthereumMainnet => write!(f, "ethereum_mainnet"),
        }
    }
}

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

#[derive(Debug, Clone, PartialEq)]
pub struct NativeCurrency {
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
    pub address: String,
}

#[derive(Debug, Clone)]
pub struct Chain {
    pub chain_id: String,
    pub name: String,
    pub network: String,
    pub symbol: String,
    pub engine_type: Engine,
    pub native_currency: Vec<NativeCurrency>,
    pub db: Mongo,
    pub confirmation_time: u64,
    nodes: HashMap<(String, ConnectionType), String>,
    rpc_methods: HashMap<SupportedMethods, Value>,
}

impl Chain {
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
        let nodes = nodes
            .iter()
            .map(|(provider, connection, url)| {
                let provider = provider;
                let connection = ConnectionType::from(connection.clone()); // SupportedChains::from_str(chain).unwrap();
                let url = url.to_string();
                ((provider.clone(), connection), url)
            })
            .collect();

        let methods = rpc_methods
            .iter()
            .map(|(method, value)| (method.clone(), value.clone()))
            .collect();

        let mongo = Mongo::new(&db_config).await.unwrap();

        Self {
            chain_id,
            name,
            symbol,
            network,
            engine_type,
            native_currency,
            nodes,
            rpc_methods: methods,
            db: mongo,
            confirmation_time,
        }
    }

    pub fn get_node(
        &self,
        provider: &String,
        connection: &ConnectionType,
    ) -> Option<&String> {
        self.nodes.get(&(provider.clone(), connection.clone()))
    }

    pub fn get_method(
        &self,
        method: &SupportedMethods,
    ) -> Option<&Value> {
        self.rpc_methods.get(method)
    }

    // pub fn decode_message<T: DeserializeOwned>(&self, message: &String) -> T
    // {
    //     let data: T = serde_json::from_str(message).unwrap();

    //     data
    // }

    fn resolve_collection_name(&self, 
        data_type: &SupportedDataTypes,
        data_level: &SupportedDataLevels,
    ) -> String {

        format!("{}_{}_{}", self.symbol, data_type.to_string(), data_level.to_string())
    }

    pub async fn save_to_db<R>(
        &self,
        items: Vec<R>,
        collection_name: &SupportedDataTypes,
        data_level: &SupportedDataLevels
    ) where
        for<'a> R: Deserialize<'a> + Serialize,
    {   
        let collection_name = self.resolve_collection_name(collection_name, &data_level);

        let collection = self.db.collection::<R>(&collection_name);

        collection.insert_many(items, None).await.unwrap();
    }

    pub async fn get_items<R>(
        &self,
        collection_name: &SupportedDataTypes,
        data_level: &SupportedDataLevels,
        filter: Option<HashMap<String, String>>,
    ) -> Vec<R>
    where
        R: DeserializeOwned + Unpin + Sync + Send + Serialize,
    {
        let collection_name = self.resolve_collection_name(collection_name, data_level);

        let collection = self.db.collection::<R>(&collection_name);
        // todo implement filters
        let filter = doc! {};

        let mut results = vec![];
        let mut items = collection.find(filter, None).await.unwrap();

        while let Some(item) = items.try_next().await.unwrap() {
            results.push(item);
        }

        results
    }
}

impl fmt::Display for Chain {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(
            f,
            "Chain: {} {} {} {}",
            self.chain_id, self.name, self.symbol, self.engine_type
        )
    }
}

pub trait DecodeLogs {
    fn decode_logs<T, R>(
        &self,
        items: Vec<T>,
    ) -> Vec<R>;
}

pub trait DecodeBlocks {
    fn decode_blocks<T, R>(
        &self,
        items: Vec<T>,
    ) -> Vec<R>;
}

pub trait DecodeTransactions {
    fn decode_transactions<T, R>(
        &self,
        items: Vec<T>,
    ) -> Vec<R>;
}

pub trait SubscribeBlocks {
    fn subscribe_blocks<T: DeserializeOwned + Unpin + Sync + Send + Serialize + 'static + std::default::Default, R: DeserializeOwned + Unpin + Sync + Send + Serialize>(&self);
}

pub trait SubscribeLogs {
    fn subscribe_logs<T, R>(&self);
}

pub trait GetLogs {
    fn get_logs<T: DeserializeOwned + Unpin + Sync + Send + Debug + Serialize>(
        &self,
        from_block_number: u64,
        to_block_number: u64,
    ) -> Result<Vec<T>>;
}
pub trait GetBlocks {
    fn get_blocks<Y: DeserializeOwned + Unpin + Sync + Send + Serialize, T: DeserializeOwned + Unpin + Sync + Send + Serialize, R>(
        &self,
        from_block_number: u64,
        to_block_number: u64,
        with_txs: bool,
    ) -> Result<Vec<T>>;
}

pub trait GetConfirmedBlocks {
    fn get_blocks<Y: DeserializeOwned + Unpin + Sync + Send + Serialize, T: DeserializeOwned + Unpin + Sync + Send + Serialize, R>(
        &self,
        from_block_number: u64,
        to_block_number: u64,
        with_txs: bool,
    ) -> Result<Vec<T>>;
}
