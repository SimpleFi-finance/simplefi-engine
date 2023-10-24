use bytes::BufMut;
use crate::{H256, U256};
use rlp::{Encodable, Decodable};
use sip_codecs::{ main_codec, Compact};


#[main_codec]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddressBalance {
    pub address: H256,
    pub balance: U256
}

impl Default for AddressBalance {
    fn default() -> Self {
        AddressBalance {
            address: Default::default(),
            balance: Default::default(),
        }
    }
}


// encode
impl Encodable for AddressBalance {
    fn encode(&self, out: &mut dyn BufMut) {
        self.address.encode(out);
        self.balance.encode(out);
    }

    fn length(&self) -> usize {

        let mut length: usize = 0;
        length += self.address.length();
        length +=self.balance.length();
        length
    }
}
// encode
impl Decodable for AddressBalance {
    fn decode(buf: &mut &[u8]) -> Result<Self, rlp::DecodeError> {

        let this = Self {
            address: Decodable::decode(buf)?,
            balance: Decodable::decode(buf)?,
        };

        if buf.len() != 0 {
            return Err(rlp::DecodeError::ListLengthMismatch {
                expected: 0,
                got: buf.len(),
            })
        }
        Ok(this)
    }
}