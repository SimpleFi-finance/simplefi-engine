use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use bson::Binary;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbiEvent {
    pub timestamp: u64,
    pub signature: String,
    pub event: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbiEventDocument {
    pub timestamp: u64,
    pub index: u32,
    pub signature: String,
    pub sorted: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSignatureDocument {
    pub id: u32,
    pub timestamp: u64,
    pub text_signature: String,
    pub hex_signature: String,
    pub bytes_signature: String,
}
