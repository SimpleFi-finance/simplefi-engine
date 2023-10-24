pub mod address_balance;
use address_balance::AddressBalance;

pub mod keys;
pub use keys::*;

use crate::{H256, U256};
use rlp::{RlpDecodable, RlpEncodable};
use sip_codecs::{ main_codec, Compact};


// Volumetric
#[main_codec]
#[derive(Debug, Clone, PartialEq, Eq,RlpDecodable, RlpEncodable)]
pub struct Volumetric {
    pub timestamp: u64,
    pub market_address: H256,
    pub swaps_out : Vec<AddressBalance>,
    pub swaps_in: Vec<AddressBalance>,
    pub withdrawal: Vec<AddressBalance>,
    pub mint: Vec<AddressBalance>,
    pub transfer: U256,
}

impl Default for Volumetric {
    fn default() -> Self {
        Volumetric {
            timestamp: Default::default(),
            market_address: Default::default(),
            swaps_out: Default::default(),
            swaps_in: Default::default(),
            withdrawal: Default::default(),
            mint: Default::default(),
            transfer: Default::default(),
        }
    }
}


// Volumetric
#[main_codec]
#[derive(Debug, Clone, PartialEq, Eq,RlpDecodable, RlpEncodable)]
pub struct PeriodVolumes {
    pub volumes: Vec<Volumetric>
}

impl Default for PeriodVolumes {
    fn default() -> Self {
        PeriodVolumes {
            volumes: Default::default()
        }
    }
}



// impl Encodable for Volumetric {
//     fn encode(&self, out: &mut dyn BufMut) {
//         self.timestamp.encode(out);
//         self.market_address.encode(out);
//         self.swaps_out.encode(out);
//         self.swaps_in.encode(out);
//         self.withdrawal.encode(out);
//         self.mint.encode(out);
//         self.transfer.encode(out);
//     }

//     fn length(&self) -> usize {


//         let mut length: usize = 0;


//         length += self.timestamp.length();
//         length += self.market_address.length();
//         length +=self.swaps_out.length();
//         length +=self.swaps_in.length();
//         length +=self.withdrawal.length();
//         length +=self.mint.length();
//         length +=self.transfer.length();


//         length
//     }
// }

// impl Decodable for Volumetric {
//     fn decode(buf: &mut &[u8]) -> Result<Self, rlp::DecodeError> {

//         let this = Self {
//             timestamp: Decodable::decode(buf)?,
//             market_address: Decodable::decode(buf)?,
//             swaps_out: Decodable::decode(buf)?,
//             swaps_in:Decodable::decode(buf)?,
//             withdrawal:Decodable::decode(buf)?,
//             mint:Decodable::decode(buf)?,
//             transfer:Decodable::decode(buf)?,
//         };

//         if buf.len() != 0 {
//             return Err(rlp::DecodeError::ListLengthMismatch {
//                 expected: 0,
//                 got: buf.len(),
//             })
//         }
//         Ok(this)
//     }
// }




#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rlp::{  Encodable, Decodable};
    use crate::{H256,U256, volumetric::AddressBalance};

    use ethers_core::utils::hex::{self};
    use super::Volumetric;
    // use std::str::FromStr;
    #[test]
    fn test_encode_block_header() {
        let expected = hex::decode("01f842a08da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceeda00000000000000000000000000000000000000000000000000000000000000000f842a08da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceeda00000000000000000000000000000000000000000000000000000000000000000f842a08da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceeda00000000000000000000000000000000000000000000000000000000000000000f842a08da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceeda00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let volume = Volumetric {
            timestamp: 1,
            market_address: H256::zero(),
            swaps_out: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance: U256::from_str("0").unwrap()}],
            swaps_in: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:   U256::from_str("0").unwrap()}],
            withdrawal: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:   U256::from_str("0").unwrap()}],
            mint: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:   U256::from_str("0").unwrap()}],
            transfer:   U256::from_str("0").unwrap(),
        };


        let mut data = vec![];
        let volumetric: Volumetric = volume.into();
        volumetric.encode(&mut data);
        assert_eq!(hex::encode(&data), hex::encode(expected));
        assert_eq!(volumetric.length(), data.len());
    }

    // Test vector from: https://eips.ethereum.org/EIPS/eip-2481
    #[test]
    fn test_decode_block_header() {
        let data = hex::decode("01f842a08da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceeda00000000000000000000000000000000000000000000000000000000000000000f842a08da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceeda00000000000000000000000000000000000000000000000000000000000000000f842a08da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceeda00000000000000000000000000000000000000000000000000000000000000000f842a08da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceeda00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000").unwrap();
        let expected = Volumetric {
            timestamp: 1,
            market_address: H256::zero(),
            swaps_out: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            swaps_in: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            withdrawal: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            mint: vec![AddressBalance{address: H256::from_str("8da82c576707872041a1237062d7c04ceaceda98a318c00bd80385d764d5ceed").unwrap(), balance:  U256::from_str("0").unwrap()}],
            transfer:  U256::from_str("0").unwrap(),
        };

        let volume = <Volumetric as Decodable>::decode(&mut data.as_slice()).unwrap();
        assert_eq!(volume, expected);
    }

}