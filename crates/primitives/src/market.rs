use rlp::{Encodable, Decodable};
use sip_codecs::{ main_codec, Compact};
use bytes::BufMut;
use crate::H256;

#[main_codec]
#[derive(Debug, Clone,PartialEq, Eq)]
pub struct Market {
    pub protocol_id: u64,
    pub input_tokens: Vec<H256>,
}

impl Default for Market {
    fn default () -> Self {
        Market {
            protocol_id: Default::default(),
            input_tokens: Default::default(),
        }
    }
}

impl Decodable for Market {
    fn decode(buf: &mut &[u8]) -> Result<Self, rlp::DecodeError> {

        let this = Self {
            protocol_id: Decodable::decode(buf)?,
            input_tokens: Decodable::decode(buf)?,
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

impl Encodable for Market {
    fn encode(&self, out: &mut dyn BufMut) {
        self.protocol_id.encode(out);
        self.input_tokens.encode(out);
    }

    fn length(&self) -> usize {

        let mut length: usize = 0;
        length += self.protocol_id.length();
        length +=self.input_tokens.length();
        length
    }
}


// token markets

#[main_codec]
#[derive(Debug, Clone,PartialEq, Eq)]
pub struct TokenMarkets {
    pub market_addresses: Vec<H256>,
}

impl Default for TokenMarkets {
    fn default () -> Self {
        TokenMarkets {
            market_addresses: Default::default(),
        }
    }
}

impl Decodable for TokenMarkets {
    fn decode(buf: &mut &[u8]) -> Result<Self, rlp::DecodeError> {

        let this = Self {
            market_addresses: Decodable::decode(buf)?,
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

impl Encodable for TokenMarkets {
    fn encode(&self, out: &mut dyn BufMut) {
        self.market_addresses.encode(out);
    }

    fn length(&self) -> usize {

        let mut length: usize = 0;
        length += self.market_addresses.length();
        length
    }
}


