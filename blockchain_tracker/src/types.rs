use third_parties::mongo::lib::bronze::blocks::types::Block;
use serde::{Deserialize, Serialize};

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
