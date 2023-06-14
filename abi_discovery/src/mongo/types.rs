use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbiCollection {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub abi: String,
    pub abi_hash: String,
    pub is_proxy: bool,
    pub standard: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationContractAbiCollection {
    pub order: u8,
    pub name: String,
    pub address: String,
    pub abi_id: Option<ObjectId>,
    pub creation_block: Option<u64>,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractAbiCollection {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abi_id: Option<ObjectId>,
    pub creation_block: Option<u64>,
    pub is_proxy: bool,
    pub implementations: Vec<ImplementationContractAbiCollection>,
    pub verified: bool,
}

