use serde::{Deserialize, Serialize};

use super::log::EthLog;

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
    pub result: Option<EthLog>,
    pub subscription: String,
}