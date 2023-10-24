use rlp::{Encodable, Decodable};
use sip_codecs::{ main_codec, Compact};
use bytes::BufMut;
use crate::H256;

#[main_codec]
#[derive(Debug, Clone,PartialEq, Eq, Copy)]
pub struct ProtocolStatus {
    pub last_sync_block_timestamp: u64,
    pub should_update: bool,
    pub has_error: bool
}

impl Default for ProtocolStatus {
    fn default () -> Self {
        ProtocolStatus {
            last_sync_block_timestamp: Default::default(),
            should_update: Default::default(),
            has_error: Default::default()
        }
    }
}

impl Decodable for ProtocolStatus {
    fn decode(buf: &mut &[u8]) -> Result<Self, rlp::DecodeError> {

        let this = Self {
            last_sync_block_timestamp: Decodable::decode(buf)?,
            should_update: Decodable::decode(buf)?,
            has_error: Decodable::decode(buf)?,
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

impl Encodable for ProtocolStatus {
    fn encode(&self, out: &mut dyn BufMut) {
        self.last_sync_block_timestamp.encode(out);
        self.should_update.encode(out);
        self.has_error.encode(out);
    }

    fn length(&self) -> usize {

        let mut length: usize = 0;
        length += self.last_sync_block_timestamp.length();
        length +=self.should_update.length();
        length +=self.has_error.length();
        length
    }
}

#[main_codec]
#[derive(Debug, Clone,PartialEq, Eq, Copy)]
pub struct Protocol {
    pub protocol_id: u64,
    pub chain_id: u64,
    pub factory_address: H256,
    pub status : ProtocolStatus
}

impl Default for Protocol {
    fn default () -> Self {
        Protocol {
            protocol_id: Default::default(),
            chain_id: Default::default(),
            factory_address: Default::default(),
            status: Default::default()
        }
    }
}


impl Decodable for Protocol {
    fn decode(buf: &mut &[u8]) -> Result<Self, rlp::DecodeError> {

        let this = Self {
            protocol_id: Decodable::decode(buf)?,
            chain_id: Decodable::decode(buf)?,
            factory_address: Decodable::decode(buf)?,
            status: Decodable::decode(buf)?,
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


impl Encodable for Protocol {
    fn encode(&self, out: &mut dyn BufMut) {
        self.protocol_id.encode(out);
        self.chain_id.encode(out);
        self.factory_address.encode(out);
        self.status.encode(out);
    }

    fn length(&self) -> usize {

        let mut length: usize = 0;
        length += self.protocol_id.length();
        length +=self.chain_id.length();
        length +=self.factory_address.length();
        length +=self.status.length();
        length
    }
}

