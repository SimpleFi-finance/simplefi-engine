use primitives::H256;
use sip_codecs::{main_codec, Compact};

#[main_codec]
/// transaction in the block and the total number of transactions
#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub struct AbiData {
    pub hash: H256,
    pub body: Vec<u8>,
}