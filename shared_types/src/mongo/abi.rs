use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use bson::Binary;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbiCollection {
    pub timestamp: i64,
    pub index: u32,
    pub abi: Binary,
}

#[derive(Deserialize, Serialize)]
pub struct PartialIndexDoc {
    pub index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ValueEnum)]
pub enum ContractAbiFlag {
    Verified,
    Unverified,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractAbiCollection {
    pub timestamp: i64,
    pub address: String,
    pub index: u32,
    pub flag: ContractAbiFlag,
}

pub struct FactoryAbiCollection {
    pub timestamp: i64,
    pub name: String,
    pub address: String,
    pub index: u32,
}

pub struct FactoryContractsCollection {
    pub address: String,
    pub factory_address: u32,
}

pub struct AddressResult {
    pub address: String,
}
