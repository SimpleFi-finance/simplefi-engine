use std::fmt::{Display, Formatter, Result};

use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum AbiDiscoveryError {
    RedisConnectionError,
    RedisError,
    MongoDBError,
    SettingsError,
    AddressError,
    OtherError,
    SerializationError,
    ProviderNotFoundError,
    ProviderAllExpiredError,
    ProviderError,
    ProviderRateLimitInvalidError,
    ProviderRateLimitExceededError,
}

// implement Display
impl Display for AbiDiscoveryError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            AbiDiscoveryError::RedisConnectionError => write!(f, "Redis connection error"),
            AbiDiscoveryError::RedisError => write!(f, "Redis error"),
            AbiDiscoveryError::MongoDBError => write!(f, "MongoDB error"),
            AbiDiscoveryError::SettingsError => write!(f, "Settings error"),
            AbiDiscoveryError::AddressError => write!(f, "Address error"),
            AbiDiscoveryError::OtherError => write!(f, "Other error"),
            AbiDiscoveryError::SerializationError => write!(f, "SerializationError error"),
            AbiDiscoveryError::ProviderNotFoundError => write!(f, "Provider not found error"),
            AbiDiscoveryError::ProviderAllExpiredError => write!(f, "All providers have their rate limits expired"),
            AbiDiscoveryError::ProviderError => write!(f, "Error while calling Provider"),
            AbiDiscoveryError::ProviderRateLimitInvalidError => write!(f, "Provider Rate Limit is invalid"),
            AbiDiscoveryError::ProviderRateLimitExceededError => write!(f, "Provider Rate Limit exceeded"),
        }
    }
}


#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AbiStandards {
    ERC20 = 0,
    ERC721 = 1,
    ERC777 = 2,
    ERC1155 = 3,
    ERCProxy = 4,
    Custom = 99,
}

impl AbiStandards {
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    pub fn from_u32(value: u32) -> AbiStandards {
        match value {
            0 => AbiStandards::ERC20,
            1 => AbiStandards::ERC721,
            2 => AbiStandards::ERC777,
            3 => AbiStandards::ERC1155,
            4 => AbiStandards::ERCProxy,
            99 => AbiStandards::Custom,
            _ => AbiStandards::Custom,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Abi {
    pub id: ObjectId,
    pub abi: String,
    pub abi_hash: String,
    pub is_proxy: bool,
    pub standard: AbiStandards,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationContractAbi {
    pub order: u8,
    pub name: String,
    pub address: String,
    pub abi: Option<Abi>,
    pub creation_block: Option<u64>,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractAbi {
    pub id: ObjectId,
    pub name: String,
    pub address: String,
    pub abi: Option<Abi>,
    pub creation_block: Option<u64>,
    pub is_proxy: bool,
    pub implementations: Vec<ImplementationContractAbi>,
    pub verified: bool,
}
