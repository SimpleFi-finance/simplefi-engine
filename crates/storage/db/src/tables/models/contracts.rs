//! Block related models and types.

use sip_codecs::{main_codec, Compact};
use primitives::{Address, BlockNumber};

use super::AbiData;

#[main_codec]
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct ContractData {
    pub abi_id: u64,
    pub block_number: Option<u64>,
    pub verified: bool,
}

impl ContractData {
    pub fn new(abi_id: u64, block_number: Option<u64>, verified: bool) -> Self {
        ContractData {
            abi_id,
            block_number,
            verified,
        }
    }
}

#[main_codec]
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct ProxyImplementations {
    pub address: Address,
    pub block_number: BlockNumber
}
/// It has the pointer to the transaction Number of the first
#[main_codec]
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct StoredContract {
    pub abi_id: u64,
    pub verified: bool,
    pub implementations: Vec<ProxyImplementations>
}


impl StoredContract {
    pub fn new(abi_id: u64, verified: bool, implementations: Option<Vec<ProxyImplementations>>) -> Self {
        let implementations = implementations.unwrap_or(Vec::new());
        StoredContract { abi_id, verified, implementations }
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct AbiContract {
    pub address: Address,
    pub abi: AbiData,
    pub contract_type: String
}
