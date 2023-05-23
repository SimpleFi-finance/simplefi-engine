use std::collections::HashMap;

use serde_json::Value;
use third_parties::mongo::MongoConfig;
use tungstenite::{connect, Message};

use super::base_chain::{Chain, DecodeLogs, SubscribeLogs, SupportedMethods, ConnectionType, SubscribeBlocks, NativeCurrency, Engine};

#[derive(Debug, Clone)]
pub struct EvmChain {
    chain: Chain,
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
    fn decode_logs<T,R>(self, items: Vec<T>) -> Vec<R> {
        // todo add logs decoding functionality
        vec![]
    }
}

impl SubscribeLogs for EvmChain {
    fn subscribe_logs<T,R>(self) {
        // todo add logs subscription functionality
        let wss_connection = self.chain.get_node(&"infura".to_string(), &ConnectionType::WSS).expect("No WSS connection found for requested provider");
        let method = self.chain.get_method(&SupportedMethods::SubscribeLogs).unwrap();
       
        let request_str = serde_json::to_string(method).unwrap();

        let (mut socket, _response) = connect(wss_connection).expect("Can't connect");
        socket.write_message(Message::Text(request_str)).unwrap();

        let mut logs_hm: HashMap<i64, Vec<T>> = HashMap::new();

        loop {
            let msg = socket.read_message().expect("Error reading message");
            let msg_str = msg.into_text().unwrap();

            // let msg_data = // use match to parse message into correct format
            // todo implement await for callback
            // save logic to save in db
            println!("Message: {}", msg_str);
        }
    }
}

impl SubscribeBlocks for EvmChain {
    fn subscribe_blocks<T,R>(self) {
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