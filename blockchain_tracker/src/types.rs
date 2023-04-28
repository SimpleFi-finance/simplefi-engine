use serde::{Deserialize, Serialize};
use shared_types::chains::evm::block::Block;

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
