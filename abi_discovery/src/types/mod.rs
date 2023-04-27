use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct RequestBody {
    pub address: String,
    // pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractAbiJson {
    #[serde(rename = "ABI")]
    pub abi: String,
    #[serde(rename = "CompilerVersion")]
    pub compiler_version: String,
    #[serde(rename = "ConstructorArguments")]
    pub constructor_arguments: String,
    #[serde(rename = "ContractName")]
    pub contract_name: String,
    #[serde(rename = "ContractAddress")]
    pub contract_address: String,
    #[serde(rename = "OptimizationUsed")]
    pub optimization_used: String,
    #[serde(rename = "Runs")]
    pub runs: String,
    #[serde(rename = "SourceCode")]
    pub source_code: String,
    #[serde(rename = "Library")]
    pub library: String,
    #[serde(rename = "LicenseType")]
    pub license_type: String,
    #[serde(rename = "Proxy")]
    pub proxy: String,
    #[serde(rename = "Implementation")]
    pub implementation: String,
    #[serde(rename = "SwarmSource")]
    pub swarm_source: String,
    #[serde(rename = "EVMVersion")]
    pub evm_version: String,
    #[serde(rename = "addressImport")]
    pub address_import: bool,
    #[serde(rename = "chainId")]
    pub chainid: u32,
}

/* #[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody {
    pub contracts: Vec<String>,
    pub imports: Vec<ContractAbiJson>,
} */

#[derive(Debug, Serialize, Deserialize)]
pub struct TokensCollection {
    pub address: String,
    pub chain: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractSerial {
    pub abi: String,
    #[serde(rename = "importedAddress")]
    pub contract_address: String,
    #[serde(rename = "importedChainId")]
    pub chain_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportSerial {
    #[serde(rename = "chainId")]
    pub chain_id: u32,
    #[serde(rename = "Implementation")]
    pub contract_address: String,
    #[serde(rename = "ABI")]
    pub abi: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBody {
    pub contracts: Vec<ContractSerial>,
    pub imports: Vec<ImportSerial>,
}
