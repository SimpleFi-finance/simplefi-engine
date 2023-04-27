use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use bson::Binary;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbiCollection {
    pub timestamp: i64,
    pub index: u32,
    pub abi: Binary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbiJSONCollection {
    pub timestamp: i64,
    pub index: u32,
    pub abi: String,
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

#[derive(Debug, Deserialize, Clone)]
pub struct ContractWithAbi {
    pub timestamp: u64,
    pub address: String,
    pub abi: Vec<u8>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ContractWithAbiJSON {
    pub timestamp: u64,
    pub address: String,
    pub abi: String,
}

#[derive(Debug, Deserialize)]
pub struct ContractWithAbiDocument {
    pub timestamp: u64,
    pub address: String,
    pub abi: Binary,
}

#[derive(Debug, Deserialize)]
pub struct ContractWithAbiJSONDocument {
    pub timestamp: u64,
    pub address: String,
    pub abi: String,
}


pub struct FactoryAbiCollection {
    pub timestamp: i64,
    pub name: String,
    pub address: String,
    pub index: u32,
}

#[derive(Serialize, Deserialize)]
pub struct FactoryContractsCollection {
    pub address: String,
    pub factory_address: String,
}


pub struct AddressResult {
    pub address: String,
}
