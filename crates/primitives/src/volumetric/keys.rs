use bytes::BufMut;
use crate::VolumeKey;
use rlp::{Encodable, Decodable};
use sip_codecs::{ main_codec, Compact};

#[main_codec]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct VolumeKeyData {
    pub timestamp: u64,
    pub key: VolumeKey
}

impl Default for VolumeKeyData {
    fn default() -> Self {
        VolumeKeyData {
            timestamp: Default::default(),
            key: Default::default(),
        }
    }
}



impl Encodable for VolumeKeyData {
    fn encode(&self, out: &mut dyn BufMut) {
        self.key.encode(out);
        self.timestamp.encode(out);
    }

    fn length(&self) -> usize {
        let mut length: usize = 0;
        length += self.key.length();
        length += self.timestamp.length();
        length
    }
}

impl Decodable for VolumeKeyData {
    fn decode(buf: &mut &[u8]) -> Result<Self, rlp::DecodeError> {

        let this = Self {
            timestamp: Decodable::decode(buf)?,
            key: Decodable::decode(buf)?,

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
