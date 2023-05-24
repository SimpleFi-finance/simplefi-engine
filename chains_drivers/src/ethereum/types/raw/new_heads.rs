use serde::{Deserialize, Serialize};

use super::{log::Log, block::Block};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewLogEvent {
    pub jsonrpc: String,
    pub method: Option<String>,
    pub result: Option<String>,
    pub params: Option<NewLogsEventParams>,
    pub id: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewLogsEventParams {
    pub result: Option<Log>,
    pub subscription: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewHeadsEvent {
    pub jsonrpc: String,
    pub method: Option<String>,
    pub result: Option<String>,
    pub params: Option<NewHeadsEventParams>,
    pub id: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewHeadsEventParams {
    pub result: Option<Block>,
    pub subscription: String,
}