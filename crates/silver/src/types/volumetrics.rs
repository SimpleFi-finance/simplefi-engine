use ethers::types::{U256, H256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AddressBalance {
  pub address: H256,
  pub balance: U256
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Volumetric {
    pub timestamp: u64,
    pub swaps_out: Vec<AddressBalance>,  
    pub swaps_in: Vec<AddressBalance>,   
    pub withdrawal: Vec<AddressBalance>, 
    pub mint: Vec<AddressBalance>,       
    pub transfer: U256,        
}

#[derive(Clone, Debug)]
pub struct Volumes {
    pub swaps_out: Vec<AddressBalance>,  
    pub swaps_in: Vec<AddressBalance>,   
    pub withdrawal: Vec<AddressBalance>, 
    pub mint: Vec<AddressBalance>,       
    pub transfer: U256,        
}

