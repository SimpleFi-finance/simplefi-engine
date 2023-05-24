use std::{fmt, collections::HashMap, future::IntoFuture};
use futures::{TryStreamExt, Future};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use third_parties::mongo::{Mongo, MongoConfig};
use std::clone::Clone;
use mongodb::{
    bson::doc,
};
use tungstenite::{connect, Message};

pub enum SupportedChains {
    EthereumMainnet,
}

impl fmt::Display for SupportedChains {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SupportedChains::EthereumMainnet => write!(f, "ethereum_mainnet"),
        }
    }
}

impl fmt::Debug for SupportedChains {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Engine::EVM => write!(f, "EVM"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SupportedMethods {
    GetLogs,
    GetBlock,
    SubscribeLogs,
    SubscribeBlocks,
    SubscribeNewHeads,
    GetTransactions,
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
    ) -> Self {

        let nodes = nodes.iter().map(|(provider, connection, url)| {
            let provider = provider;
            let connection = ConnectionType::from(connection.clone()); // SupportedChains::from_str(chain).unwrap();
            let url = url.to_string();
            ((provider.clone(), connection), url)
        }).collect();

        let methods = rpc_methods.iter().map(|(method, value)| {
            (method.clone(), value.clone())
        }).collect();

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
        }
    }

    pub fn get_node(&self, provider: &String, connection: &ConnectionType) -> Option<&String> {
        self.nodes.get(&(provider.clone(), connection.clone()))
    }

    pub fn get_method(&self, method: &SupportedMethods) -> Option<&Value> {
        self.rpc_methods.get(method)
    }

    // pub fn decode_message<T: DeserializeOwned>(&self, message: &String) -> T
    // {
    //     let data: T = serde_json::from_str(message).unwrap();

    //     data
    // }

    pub async fn save_to_db<R>(&self, items: Vec<R>, collection_name: String)
    where 
        for<'a> R: Deserialize<'a> + Serialize
    {
        let collection = self.db.collection::<R>(&collection_name);

        collection.insert_many(items, None).await.unwrap();
    }

    pub async fn get_items<R>(&self, collection_name: String, filter: Option<HashMap<String, String>>) -> Vec<R>
    where 
        R: DeserializeOwned + Unpin + Sync + Send + Serialize
    {
        let collection = self.db.collection::<R>(&collection_name);
        // todo implement filters
        let filter = doc!{};

        let mut results = vec![];
        let mut items = collection.find(filter, None).await.unwrap();

        while let Some(item) = items.try_next().await.unwrap() {
            results.push(item);
        }

        results
    }
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Chain: {} {} {} {}", self.chain_id, self.name, self.symbol, self.engine_type)
    }
}

// type Callback<T,R> = fn(Vec<T>) -> Vec<R>;
pub trait DecodeLogs {
    fn decode_logs<T,R>(&self, items: Vec<T>) -> Vec<R>;
}

pub trait DecodeBlocks {
    fn decode_blocks<T,R>(&self, items: Vec<T>) -> Vec<R>;
}

pub trait DecodeTransactions {
    fn decode_transactions<T,R>(&self, items: Vec<T>) -> Vec<R>;
}

pub trait SubscribeBlocks {
    fn subscribe_blocks<T,R>(&self);
}

pub trait SubscribeLogs {
    fn subscribe_logs<T,R>(&self);
}

pub trait GetTransactions {
    fn get_transactions<T,R>(&self, from_block_number: u64, to_block_number: u64);
}

pub trait GetLogs {
    fn get_logs<T,R>(&self, from_block_number: u64, to_block_number: u64);
}


// type CallbackWSS<T,R> = fn(String, &HashMap<i64, Vec<T>>) -> Future<>;

// type CallbackWSS<T> = fn(String, &HashMap<i64, Vec<T>>);

// #[async_trait]
// pub trait WSSLogProcessor {
//     async fn subscribe_logs<T>(self, cb: CallbackWSS<T>);
// }

// #[async_trait]
// impl WSSLogProcessor for Chain {
//     async fn subscribe_logs<T>(self, cb: CallbackWSS<T>) {
//         let wss_connection = self.get_node(&"infura".to_string(), &ConnectionType::WSS).expect("No WSS connection found for requested provider");
//         let method = self.get_method(&SupportedMethods::SubscribeLogs).unwrap();
       
//         let request_str = serde_json::to_string(method).unwrap();

//         let (mut socket, _response) = connect(wss_connection).expect("Can't connect");
//         socket.write_message(Message::Text(request_str)).unwrap();

//         // save logs in hashmap


//         let mut logs_hm: HashMap<i64, Vec<T>> = HashMap::new();

//         loop {
//             let msg = socket.read_message().expect("Error reading message");
//             let msg_str = msg.into_text().unwrap();
//             // todo implement await for callback

//             cb(msg_str.clone(), &logs_hm);
//             println!("Message: {}", msg_str);
//         }
//     }
// }
// impl RawToBronze for Chain {
//     // import types from common
//     fn logs_to_bronze<T>(&self, logs: Vec<T>) -> Vec<serde_json::Value> {
        
//     }
// }

// impl<'a, P, R> Stream for SubscriptionStream<'a, P, R>
// where
//     P: PubsubClient,
//     R: DeserializeOwned,
// {
//     type Item = R;

//     fn poll_next(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Option<Self::Item>> {
//         if !self.loaded_elements.is_empty() {
//             let next_element = self.get_mut().loaded_elements.pop_front();
//             return Poll::Ready(next_element)
//         }

//         let mut this = self.project();
//         loop {
//             return match futures_util::ready!(this.rx.as_mut().poll_next(ctx)) {
//                 Some(item) => match serde_json::from_str(item.get()) {
//                     Ok(res) => Poll::Ready(Some(res)),
//                     Err(err) => {
//                         error!("failed to deserialize item {:?}", err);
//                         continue
//                     }
//                 },
//                 None => Poll::Ready(None),
//             }
//         }
//     }
// }
