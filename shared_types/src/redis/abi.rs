use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractWithAbiRedis {
    pub timestamp: u64,
    pub abi: Vec<u8>,
}
