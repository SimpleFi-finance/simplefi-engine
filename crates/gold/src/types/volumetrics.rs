use ethers::types::H256;
use serde::{Deserialize, Serialize};

pub struct AddressBalance {
  pub address: H256,
  pub balance: H256
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Volumetric {
    pub timestamp: u64,
    pub swaps_out: Vec<AddressBalance>,  
    pub swaps_in: Vec<AddressBalance>,   
    pub withdrawal: Vec<AddressBalance>, 
    pub mint: Vec<AddressBalance>,       
    pub transfer: H256,        
}

#[derive(Clone, Debug)]
pub struct Volumes {
    pub swaps_out: Vec<AddressBalance>,  
    pub swaps_in: Vec<AddressBalance>,   
    pub withdrawal: Vec<AddressBalance>, 
    pub mint: Vec<AddressBalance>,       
    pub transfer: H256,        
}


/*
 {
   protocol_id: 1,
   timestamp: 1687362739524,
   market_address: "0x3041cbd36888becc7bbcbc0045e3b1f144466f5f",
   swaps_out: ["0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48|32886000000", 0xdAC17F958D2ee523a2206206994597C13D831ec7|292874000000"],
   swaps_out: ["0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48|24557000000", 0xdAC17F958D2ee523a2206206994597C13D831ec7|31886000000"],
   mint: ["0x3041cbd36888becc7bbcbc0045e3b1f144466f5f|13000000000000000000"],
   burn: ["0x3041cbd36888becc7bbcbc0045e3b1f144466f5f|4000000000000000000"],
   transfer: ["0x3041cbd36888becc7bbcbc0045e3b1f144466f5f|241000000000000000000", ]
 }
*/
