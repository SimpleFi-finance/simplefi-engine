use serde::{Serialize,de::DeserializeOwned};

use super::{error::RpcProviderError, GenericNodeResponse};

#[async_trait::async_trait]
pub trait ChainRpcProvider: Send + Sync {
    async fn get_block_header<T: Serialize + DeserializeOwned + Send>(&self, block_number: u64) -> Result<GenericNodeResponse<T>, RpcProviderError>;
    async fn get_block_txs<T: Serialize + DeserializeOwned + Send>(&self, block_number: u64) -> Result<GenericNodeResponse<Vec<T>>, RpcProviderError>;
    async fn get_block_logs<T: Serialize + DeserializeOwned + Send>(&self, block_number: u64) -> Result<GenericNodeResponse<Vec<T>>, RpcProviderError>;
    async fn get_block_traces<T: Serialize + DeserializeOwned + Send>(&self, block_number: u64) -> Result<GenericNodeResponse<Vec<T>>, RpcProviderError>;
    async fn get_blocks_headers<T: Serialize + DeserializeOwned + Send>(&self, from: u64, to: u64) -> Result<Vec<T>, RpcProviderError>;
    async fn get_blocks_txs<T: Serialize + DeserializeOwned + Send>(&self, from: u64, to: u64) -> Result<Vec<T>, RpcProviderError>;
    async fn get_blocks_logs<T: Serialize + DeserializeOwned + Send>(&self, from: u64, to: u64) -> Result<Vec<T>, RpcProviderError>;
    // TODO: find way to declare iterator in trait return type
    // fn get_blocks_headers_iter<T>(&self, from: u64, to: u64) -> impl Iterator<Item = T> + '_;
    // fn get_blocks_txs_iter<T>(&self, from: u64, to: u64) -> impl Iterator<Item = T> + '_;
    // fn get_blocks_logs_iter<T>(&self, from: u64, to: u64) -> impl Iterator<Item = T> + '_;
    async fn subscribe_block<T: Serialize + DeserializeOwned + Send>(&self) -> Result<T, RpcProviderError>;
}