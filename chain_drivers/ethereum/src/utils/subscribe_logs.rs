use std::collections::HashMap;

use common::types::chain::Chain;
use log::info;
use serde::{Deserialize, Serialize};

use crate::types::raw::log::Log;

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

pub fn subscribe_logs(msg: String, logs_hm: &mut HashMap<i64, Vec<Log>>, chain: &Chain) {
    let log_data: NewLogEvent = chain.decode_message(&msg);

    let log = log_data.params;

    let data = match log {
        Some(log) => {
            log.result
        }
        None => {
            info!("No result data");
            None
        }
    };
    let mut last_bn = 0;

    match data {
        Some(data) => {
            if data.block_number > last_bn {
            // trigger reading of previous block and decoding
                let prev_block_data = logs_hm.get(&last_bn);
                match prev_block_data {
                    Some(prev_block_data) => {
                    //     let bn = last_bn.clone();
                    //     let logs = prev_block_data.clone();
                    //     logs_hm.remove(&bn);
                    //     let db = chain.db;
                    //     // move into chain specific methods
                    //     tokio::spawn(async move {
                    //         let now = std::time::Instant::now();
                    //         let last_bn = bn.clone();

                    //         let decoded = chain.decode_logs(logs, decode_callback).await.unwrap();
                    //         // todo remove hardcoded collection name
                    //         chain.save_to_db(decoded.0, "logs_bronze".to_string()).await;
                    //         save_decoding_error(&db, decoded.1).await.unwrap();

                    //         debug!("Prev block {:?} data decoded", &last_bn);
                    //         debug!("Decoding took {:?}", now.elapsed());
                    //     });
                        info!("Prev block {:?} data decoded", &last_bn)
                    }
                    None => {info!("No prev block data")}
                }
                
                last_bn = data.block_number;
            }
            logs_hm.entry(data.block_number).or_insert(Vec::new()).push(data.clone());
        }
        None => {
            info!("No log data")
        }
    }
}   