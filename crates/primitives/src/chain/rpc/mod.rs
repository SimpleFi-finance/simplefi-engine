pub mod error;

use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericNodeResponse<T> {
    pub jsonrpc: String,
    pub id: u64,
    pub result: T,
}