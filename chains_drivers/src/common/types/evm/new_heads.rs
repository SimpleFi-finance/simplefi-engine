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
pub struct NewHeadsEvent<R> {
    pub jsonrpc: String,
    pub method: Option<String>,
    pub result: Option<String>,
    #[serde(default)]
    pub params: Option<NewHeadsEventParams<R>>,
    pub id: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct NewHeadsEventParams<R> {
    #[serde(default)]
    pub result: Option<Block<R>>,
    pub subscription: String,
}