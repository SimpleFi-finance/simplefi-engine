use db::tables::{StoredContract, models::{AbiContract, ProxyImplementations}, ContractData, AbiData};
use interfaces::Result;
use primitives::{Address, StoredLog, DecodedData, BlockNumber};
use auto_impl::auto_impl;

#[auto_impl(&, Arc, Box)]
pub trait AbiProvider: Send + Sync {
    fn get_abi_by_id(&self, id: u64) -> Result<Option<AbiData>>;

    fn get_abis_by_address(&self, address: Address) -> Result<Option<Vec<AbiContract>>>;

    fn has_proxy(&self, address: Address) -> Result<Option<Address>>;

    fn get_contract_data(&self, address: Address) -> Result<Option<ContractData>>;

    fn decode_logs(&self, logs: Vec<StoredLog>, abi: &Vec<AbiContract>) -> Result<Vec<Option<Vec<DecodedData>>>>;

    fn get_proxy_data(&self, address: Address) -> Result<Option<StoredContract>>;

    fn get_latest_abi(&self) -> Result<Option<u64>>;

    fn address_without_abi(&self, address: Address) -> Result<Option<(Address, u32)>>;
}

#[auto_impl(&, Arc, Box)]
pub trait AbiWriter: Send + Sync {
    fn insert_abi(&self, abi: AbiData) -> Result<Option<u64>>;

    fn insert_contract(&self, address: Address, abi_id: u64, verified: bool, block_number: Option<BlockNumber>) -> Result<Address>;

    fn upsert_proxy(&self, address: Address, abi_id: u64, verified: bool, implementations: Vec<ProxyImplementations>) -> Result<Address>;

    fn insert_contract_proxy_index(&self, address: Address, proxy: Address, force_upsert: bool) -> Result<Address>;

    fn insert_unknown_contract(&self, address: Address, timestamp: u32) -> Result<()>;
}