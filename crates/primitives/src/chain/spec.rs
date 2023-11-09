use crate::{constants::{
    EIP1559_DEFAULT_BASE_FEE_MAX_CHANGE_DENOMINATOR, EIP1559_DEFAULT_ELASTICITY_MULTIPLIER
}, Chain};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::{ChainRpcProvider, error::RpcProviderError};

/// The Ethereum mainnet spec
pub static MAINNET: Lazy<Arc<ChainSpec>> = Lazy::new(|| {
    ChainSpec {
        chain: Chain::mainnet(),
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
    EVMCompatible
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
    // /// Get an iterator of all hardforks with their respective activation conditions.
    // pub fn forks_iter(&self) -> impl Iterator<Item = (Hardfork, ForkCondition)> + '_ {
    //     self.hardforks.iter().map(|(f, b)| (*f, *b))
    // }

    /// Build a chainspec using [`ChainSpecBuilder`]
    pub fn builder() -> ChainSpecBuilder {
        ChainSpecBuilder::default()
    }
}

impl ChainRpcProvider for ChainSpec {
    fn get_block_header<T>(&self, block_number: u64) -> Result<T, RpcProviderError> {
        match self.chain_type() {
            ComputationEngine::EVM | ComputationEngine::EVMCompatible => {
                unimplemented!()
            },
            _ => {
                Err(RpcProviderError::InvalidRequest("Invalid chain type".to_string()))
            }
        }
    }

    fn get_block_logs<T>(&self, block_number: u64) -> Result<Vec<T>, RpcProviderError> {
        match self.chain_type() {
            ComputationEngine::EVM | ComputationEngine::EVMCompatible => {
                unimplemented!()
            },
            _ => {
                Err(RpcProviderError::InvalidRequest("Invalid chain type".to_string()))
            }
        }
    }

    fn get_block_traces<T>(&self, block_number: u64) -> Result<Vec<T>, RpcProviderError> {
        match self.chain_type() {
            ComputationEngine::EVM | ComputationEngine::EVMCompatible => {
                unimplemented!()
            },
            _ => {
                Err(RpcProviderError::InvalidRequest("Invalid chain type".to_string()))
            }
        }
    }

    fn get_block_txs<T>(&self, block_number: u64) -> Result<Vec<T>, RpcProviderError> {
        match self.chain_type() {
            ComputationEngine::EVM | ComputationEngine::EVMCompatible => {
                unimplemented!()
            },
            _ => {
                Err(RpcProviderError::InvalidRequest("Invalid chain type".to_string()))
            }
        }
    }

    fn get_blocks_headers<T>(&self, from: u64, to: u64) -> Result<Vec<T>, RpcProviderError> {
        match self.chain_type() {
            ComputationEngine::EVM | ComputationEngine::EVMCompatible => {
                unimplemented!()
            },
            _ => {
                Err(RpcProviderError::InvalidRequest("Invalid chain type".to_string()))
            }
        }
    }

    fn get_blocks_logs<T>(&self, from: u64, to: u64) -> Result<Vec<T>, RpcProviderError> {
        match self.chain_type() {
            ComputationEngine::EVM | ComputationEngine::EVMCompatible => {
                unimplemented!()
            },
            _ => {
                Err(RpcProviderError::InvalidRequest("Invalid chain type".to_string()))
            }
        }
    }

    fn get_blocks_txs<T>(&self, from: u64, to: u64) -> Result<Vec<T>, RpcProviderError> {
        match self.chain_type() {
            ComputationEngine::EVM | ComputationEngine::EVMCompatible => {
                unimplemented!()
            },
            _ => {
                Err(RpcProviderError::InvalidRequest("Invalid chain type".to_string()))
            }
        }
    }

    fn subscribe_block<T>(&self) -> Result<T, RpcProviderError> {
        match self.chain_type() {
            ComputationEngine::EVM | ComputationEngine::EVMCompatible => {
                unimplemented!()
            },
            _ => {
                Err(RpcProviderError::InvalidRequest("Invalid chain type".to_string()))
            }
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
    pub fn chain(mut self, chain: Chain) -> Self {
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