use std::{collections::HashMap};
use logs_subscriber::{settings::load_settings, utils::decode_logs::decode_logs};
use serde::{Deserialize, Serialize};
use serde_json::json;
use settings::load_settings as load_global_settings;
use shared_types::chains::evm::log::Log;
use shared_utils::logger::init_logging;
use log::{ debug, error };
use third_parties::mongo::{lib::bronze::logs::setters::save_logs, MongoConfig, Mongo};
use tungstenite::{connect, Message};

#[tokio::main]
async fn main() {
    let global_settings = load_global_settings().unwrap();
    let local_settings = load_settings().unwrap();
    init_logging();

    let mut logs_hm: HashMap<i64, Vec<Log>> = HashMap::new();

    let wss_url = format!("{}{}", global_settings.infura_mainnet_ws, global_settings.infura_token);

    let request_method = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "eth_subscribe",
        "params": ["logs", {}]
    });

    let request_str = serde_json::to_string(&request_method).unwrap();

    let (mut socket2, _response) = connect(&wss_url).expect("Can't connect");
    socket2.write_message(Message::Text(request_str)).unwrap();
    let mut last_bn = 0;
    let db_config = MongoConfig {
        uri: local_settings.mongodb_uri.clone(),
        database: local_settings.mongodb_database_name.clone(),
    };

    let db = Mongo::new(&db_config).await.unwrap();

    loop {
        let msg = socket2.read_message().expect("Error reading message");
        let msg = msg.into_text().unwrap();
        let log_data: NewLogEvent = serde_json::from_str(&msg).unwrap();
        let log = log_data.params;
        match log {
            Some(log) => {
                let data = log.result;
                match data {
                    Some(data) => {
                        if data.block_number > last_bn {
                            // trigger reading of previous block and decoding
                            debug!("");
                            let prev_block_data = logs_hm.get(&last_bn);
                            match prev_block_data {
                                Some(prev_block_data) => {
                                    let bn = last_bn.clone();
                                    let logs = prev_block_data.clone();
                                    logs_hm.remove(&bn);
                                    let db = db.clone();

                                    tokio::spawn(async move {
                                        let now = std::time::Instant::now();
                                        let last_bn = bn.clone();
                                        let decoded = decode_logs(logs, &db).await.unwrap();
                                        // save to mongodb
                                        save_logs(&db, decoded).await.unwrap();
                                        debug!("Prev block {:?} data decoded", &last_bn);
                                        debug!("Decoding took {:?}", now.elapsed());
                                    });
                                }
                                None => {error!("No prev block data")}
                            }
                            
                            last_bn = data.block_number;
                        }
                        logs_hm.entry(data.block_number).or_insert(Vec::new()).push(data.clone());
                    }
                    None => {error!("No log data")}
                }
            }
            None => {error!("No result data")}
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewLogEvent {
    pub jsonrpc: String,
    pub method: Option<String>,
    pub result: Option<String>,
    pub params: Option<NewHeadsEventParams>,
    pub id: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewHeadsEventParams {
    pub result: Option<Log>,
    pub subscription: String,
}