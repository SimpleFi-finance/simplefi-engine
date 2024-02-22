use super::{error::RpcProviderError, ChainRpcProvider};
use crate::{chain::GenericNodeResponse, Chain};
use once_cell::sync::Lazy;
use reqwest;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::sync::Arc;

/// The Ethereum mainnet spec
pub static MAINNET: Lazy<Arc<ChainSpec>> = Lazy::new(|| {
    ChainSpec {
        chain: Chain::mainnet(),
        rpc_connection: Default::default(),
        computation_engine: ComputationEngine::EVM,
        mint_time: 12000,
        confirmation_block_time: 12,
    }
    .into()
});

/// The Goerli spec
pub static GOERLI: Lazy<Arc<ChainSpec>> = Lazy::new(|| {
    ChainSpec {
        chain: Chain::goerli(),
        rpc_connection: Default::default(),
        computation_engine: ComputationEngine::EVM,
        mint_time: 4000,
        confirmation_block_time: 15,
    }
    .into()
});

/// The Sepolia spec
pub static SEPOLIA: Lazy<Arc<ChainSpec>> = Lazy::new(|| {
    ChainSpec {
        chain: Chain::sepolia(),
        rpc_connection: Default::default(),
        computation_engine: ComputationEngine::EVM,
        mint_time: 4000,
        confirmation_block_time: 15,
    }
    .into()
});

pub static DEV: Lazy<Arc<ChainSpec>> = Lazy::new(|| {
    ChainSpec {
        chain: Chain::dev(),
        ..Default::default()
    }
    .into()
});

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
pub enum ComputationEngine {
    EVM,
    EVMCompatible,
    COSMOS,
}

impl Default for ComputationEngine {
    fn default() -> Self {
        ComputationEngine::EVM
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChainSpec {
    /// The chain ID
    pub chain: Chain,

    pub rpc_connection: String,
    // type of chain computation engine
    pub computation_engine: ComputationEngine,
    /// Approximate mint time of a new block in millisecond
    pub mint_time: u64,
    /// Number of blocks to confirm a given block
    pub confirmation_block_time: u64,
}

impl Default for ChainSpec {
    fn default() -> ChainSpec {
        ChainSpec {
            chain: Default::default(),
            rpc_connection: Default::default(),
            mint_time: Default::default(),
            computation_engine: Default::default(),
            confirmation_block_time: Default::default(),
        }
    }
}

impl ChainSpec {
    /// Get information about the chain itself
    pub fn chain(&self) -> Chain {
        self.chain
    }

    pub fn mint_time(&self) -> u64 {
        self.mint_time
    }

    pub fn confirmation_block_time(&self) -> u64 {
        self.confirmation_block_time
    }

    pub fn is_evm(&self) -> bool {
        self.computation_engine == ComputationEngine::EVM
    }

    pub fn chain_type(&self) -> ComputationEngine {
        self.computation_engine
    }

    pub fn rpc_connection(&self) -> String {
        self.rpc_connection.clone()
    }
    // /// Get an iterator of all hardforks with their respective activation conditions.
    // pub fn forks_iter(&self) -> impl Iterator<Item = (Hardfork, ForkCondition)> + '_ {
    //     self.hardforks.iter().map(|(f, b)| (*f, *b))
    // }

    /// Build a chainspec using [`ChainSpecBuilder`]
    pub fn builder() -> ChainSpecBuilder {
        ChainSpecBuilder::default()
    }
}

#[async_trait::async_trait]
impl ChainRpcProvider for ChainSpec {
    async fn get_block_header<T: Serialize + DeserializeOwned + Send>(
        &self,
        block_number: u64,
    ) -> Result<GenericNodeResponse<T>, RpcProviderError> {
        let client = reqwest::Client::new();

        match self.chain_type() {
            ComputationEngine::EVM | ComputationEngine::EVMCompatible => {
                let query = serde_json::to_string(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "eth_getBlockByNumber",
                    "params": [&format!("0x{:x}", block_number), false],
                }))
                .unwrap();

                let request = client
                    .post(self.rpc_connection())
                    .body(query)
                    .send()
                    .await
                    .unwrap();

                let response = request.text().await.unwrap();
                let data: GenericNodeResponse<T> = serde_json::from_str(&response).unwrap();
                Ok(data)
            }
            _ => Err(RpcProviderError::InvalidRequest(
                "Invalid chain type".to_string(),
            )),
        }
    }

    async fn get_block_logs<T: Serialize + DeserializeOwned + Send>(
        &self,
        block_number: u64,
    ) -> Result<GenericNodeResponse<Vec<T>>, RpcProviderError> {
        let client = reqwest::Client::new();

        match self.chain_type() {
            ComputationEngine::EVM | ComputationEngine::EVMCompatible => {
                let query = serde_json::to_string(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "eth_getLogs",
                    "params": [{
                        "fromBlock": &format!("0x{:x}", block_number),
                        "toBlock": &format!("0x{:x}", block_number),
                    }],
                }))
                .unwrap();

                let request = client
                    .post(self.rpc_connection())
                    .body(query)
                    .send()
                    .await
                    .unwrap();

                let response = request.text().await.unwrap();
                let data: GenericNodeResponse<Vec<T>> = serde_json::from_str(&response).unwrap();
                Ok(data)
            }
            _ => Err(RpcProviderError::InvalidRequest(
                "Invalid chain type".to_string(),
            )),
        }
    }

    async fn get_block_traces<T: Serialize + DeserializeOwned + Send>(
        &self,
        block_number: u64,
    ) -> Result<GenericNodeResponse<Vec<T>>, RpcProviderError> {
        let client = reqwest::Client::new();
        match self.chain_type() {
            ComputationEngine::EVM | ComputationEngine::EVMCompatible => {
                let query = serde_json::to_string(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "trace_block",
                    "params": [{
                        "fromBlock": &format!("0x{:x}", block_number),
                        "toBlock": &format!("0x{:x}", block_number),
                    }],
                }))
                .unwrap();

                let request = client
                    .post(self.rpc_connection())
                    .body(query)
                    .send()
                    .await
                    .unwrap();

                let response = request.text().await.unwrap();
                let data: GenericNodeResponse<Vec<T>> = serde_json::from_str(&response).unwrap();
                Ok(data)
            }
            _ => Err(RpcProviderError::InvalidRequest(
                "Invalid chain type".to_string(),
            )),
        }
    }

    async fn get_block_txs<T: Serialize + DeserializeOwned + Send>(
        &self,
        block_number: u64,
    ) -> Result<GenericNodeResponse<Vec<T>>, RpcProviderError> {
        let client = reqwest::Client::new();
        match self.chain_type() {
            ComputationEngine::EVM | ComputationEngine::EVMCompatible => {
                let query = serde_json::to_string(&serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "eth_getBlockByNumber",
                    "params": [&format!("0x{:x}", block_number), true],
                }))
                .unwrap();

                let request = client
                    .post(self.rpc_connection())
                    .body(query)
                    .send()
                    .await
                    .unwrap();

                let response = request.text().await.unwrap();
                let data: GenericNodeResponse<Vec<T>> = serde_json::from_str(&response).unwrap();
                Ok(data)
            }
            _ => Err(RpcProviderError::InvalidRequest(
                "Invalid chain type".to_string(),
            )),
        }
    }

    async fn get_blocks_headers<T: Serialize + DeserializeOwned + Send>(
        &self,
        from: u64,
        to: u64,
    ) -> Result<Vec<T>, RpcProviderError> {
        match self.chain_type() {
            ComputationEngine::EVM | ComputationEngine::EVMCompatible => {
                let mut headers = vec![];

                for bn in from..=to {
                    let header = self.get_block_header(bn).await.unwrap();
                    headers.push(header.result);
                }
                Ok(headers)
            }
            _ => Err(RpcProviderError::InvalidRequest(
                "Invalid chain type".to_string(),
            )),
        }
    }

    async fn get_blocks_logs<T: Serialize + DeserializeOwned + Send>(
        &self,
        from: u64,
        to: u64,
    ) -> Result<Vec<T>, RpcProviderError> {
        match self.chain_type() {
            ComputationEngine::EVM | ComputationEngine::EVMCompatible => {
                // by block for now and find better way to get more blocks per call
                let mut logs = vec![];

                for bn in from..=to {
                    let log = self.get_block_logs(bn).await.unwrap();
                    logs.extend(log.result);
                }

                Ok(logs)
            }
            _ => Err(RpcProviderError::InvalidRequest(
                "Invalid chain type".to_string(),
            )),
        }
    }

    async fn get_blocks_txs<T: Serialize + DeserializeOwned + Send>(
        &self,
        from: u64,
        to: u64,
    ) -> Result<Vec<T>, RpcProviderError> {
        match self.chain_type() {
            ComputationEngine::EVM | ComputationEngine::EVMCompatible => {
                let mut txs = vec![];

                for bn in from..=to {
                    let tx = self.get_block_txs(bn).await.unwrap();
                    txs.extend(tx.result);
                }

                Ok(txs)
            }
            _ => Err(RpcProviderError::InvalidRequest(
                "Invalid chain type".to_string(),
            )),
        }
    }

    async fn subscribe_block<T: Serialize + DeserializeOwned + Send>(
        &self
    ) -> Result<T, RpcProviderError> {
        match self.chain_type() {
            ComputationEngine::EVM | ComputationEngine::EVMCompatible => {
                // (
                //     "subscribeNewHeads".to_string(),
                //     serde_json::json!({
                //         "jsonrpc": "2.0",
                //         "id": 1,
                //         "method": "eth_subscribe",
                //         "params": ["newHeads", {}]
                //     })
                // )
                unimplemented!()
            }
            _ => Err(RpcProviderError::InvalidRequest(
                "Invalid chain type".to_string(),
            )),
        }
    }
}

/// A helper to build custom chain specs
#[derive(Debug, Default)]
pub struct ChainSpecBuilder {
    chain: Option<Chain>,
}

impl ChainSpecBuilder {
    /// Construct a new builder from the mainnet chain spec.
    pub fn mainnet() -> Self {
        Self {
            chain: Some(MAINNET.chain),
        }
    }

    /// Set the chain ID
    pub fn chain(
        mut self,
        chain: Chain,
    ) -> Self {
        self.chain = Some(chain);
        self
    }

    /// This function panics if the chain ID and genesis is not set ([`Self::chain`] and
    /// [`Self::genesis`])
    pub fn build(self) -> ChainSpec {
        ChainSpec {
            chain: self.chain.expect("The chain is required"),
            ..Default::default()
        }
    }
}

impl From<&Arc<ChainSpec>> for ChainSpecBuilder {
    fn from(value: &Arc<ChainSpec>) -> Self {
        Self {
            chain: Some(value.chain),
        }
    }
}
