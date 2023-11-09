use super::error::RpcProviderError;

pub trait ChainRpcProvider: Send + Sync {
    fn get_block_header<T>(&self, block_number: u64) -> Result<T, RpcProviderError>;
    fn get_block_txs<T>(&self, block_number: u64) -> Result<Vec<T>, RpcProviderError>;
    fn get_block_logs<T>(&self, block_number: u64) -> Result<Vec<T>, RpcProviderError>;
    fn get_block_traces<T>(&self, block_number: u64) -> Result<Vec<T>, RpcProviderError>;
    fn get_blocks_headers<T>(&self, from: u64, to: u64) -> Result<Vec<T>, RpcProviderError>;
    fn get_blocks_txs<T>(&self, from: u64, to: u64) -> Result<Vec<T>, RpcProviderError>;
    fn get_blocks_logs<T>(&self, from: u64, to: u64) -> Result<Vec<T>, RpcProviderError>;
    // TODO: find way to declare iterator in trait return type
    // fn get_blocks_headers_iter<T>(&self, from: u64, to: u64) -> impl Iterator<Item = T> + '_;
    // fn get_blocks_txs_iter<T>(&self, from: u64, to: u64) -> impl Iterator<Item = T> + '_;
    // fn get_blocks_logs_iter<T>(&self, from: u64, to: u64) -> impl Iterator<Item = T> + '_;
    fn subscribe_block<T>(&self) -> Result<T, RpcProviderError>;
}