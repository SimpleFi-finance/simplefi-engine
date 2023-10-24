//! Implements data structures specific to the database
use crate::table::{Decode, Encode};
use interfaces::db::DatabaseError;

use primitives::{
    Address, H256,
};

pub mod integer_list;
pub mod sharded_key;
mod abi;
pub use abi::*;

mod contracts;
pub use contracts::*;
// pub mod storage_sharded_key;
mod blocks;
pub use blocks::*;

mod logs;
pub mod volumetric;
pub use volumetric::*;

pub use logs::*;
// pub use sharded_key::ShardedKey;

/// Macro that implements [`Encode`] and [`Decode`] for uint types.
macro_rules! impl_uints {
    ($($name:tt),+) => {
        $(
            impl Encode for $name
            {
                type Encoded = [u8; std::mem::size_of::<$name>()];

                fn encode(self) -> Self::Encoded {
                    self.to_be_bytes()
                }
            }

            impl Decode for $name
            {
                fn decode<B: AsRef<[u8]>>(value: B) -> Result<Self, DatabaseError> {
                    Ok(
                        $name::from_be_bytes(
                            value.as_ref().try_into().map_err(|_| DatabaseError::DecodeError)?
                        )
                    )
                }
            }
        )+
    };
}

impl_uints!(u64, u32, u16, u8);

impl Encode for Vec<u8> {
    type Encoded = Vec<u8>;
    fn encode(self) -> Self::Encoded {
        self
    }
}

impl Decode for Vec<u8> {
    fn decode<B: AsRef<[u8]>>(value: B) -> Result<Self, DatabaseError> {
        Ok(value.as_ref().to_vec())
    }
}

impl Encode for Address {
    type Encoded = [u8; 20];
    fn encode(self) -> Self::Encoded {
        self.to_fixed_bytes()
    }
}

impl Decode for Address {
    fn decode<B: AsRef<[u8]>>(value: B) -> Result<Self, DatabaseError> {
        Ok(Address::from_slice(value.as_ref()))
    }
}

impl Encode for H256 {
    type Encoded = [u8; 32];
    fn encode(self) -> Self::Encoded {
        self.to_fixed_bytes()
    }
}

impl Decode for H256 {
    fn decode<B: AsRef<[u8]>>(value: B) -> Result<Self, DatabaseError> {
        Ok(H256::from_slice(value.as_ref()))
    }
}

impl Encode for String {
    type Encoded = Vec<u8>;
    fn encode(self) -> Self::Encoded {
        self.into_bytes()
    }
}

impl Decode for String {
    fn decode<B: AsRef<[u8]>>(value: B) -> Result<Self, DatabaseError> {
        String::from_utf8(value.as_ref().to_vec()).map_err(|_| DatabaseError::DecodeError)
    }
}

// impl Encode for PrunePart {
//     type Encoded = [u8; 1];

//     fn encode(self) -> Self::Encoded {
//         let mut buf = [0u8];
//         self.to_compact(&mut buf.as_mut());
//         buf
//     }
// }

// impl Decode for PrunePart {
//     fn decode<B: AsRef<[u8]>>(value: B) -> Result<Self, DatabaseError> {
//         let buf = value.as_ref();
//         Ok(Self::from_compact(buf, buf.len()).0)
//     }
// }
