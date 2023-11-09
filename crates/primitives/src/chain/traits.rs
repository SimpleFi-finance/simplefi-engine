use crate::ProcessorError;

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


pub trait ChainDataProcessor: Send + Sync {
    fn process_block_header<T>(&self, block_header: T) -> Result<(), ProcessorError>;
    fn process_block_txs<T>(&self, block_txs: Vec<T>) -> Result<(), ProcessorError>;
    fn process_block_logs<T>(&self, block_logs: Vec<T>) -> Result<(), ProcessorError>;
    fn process_block_traces<T>(&self, block_traces: Vec<T>) -> Result<(), ProcessorError>;
}