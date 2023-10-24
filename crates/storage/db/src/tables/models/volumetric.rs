// use primitives::rpc_utils::rl;
use sip_codecs::{main_codec, Compact};
use primitives::VolumeKey;


/// The storage of the volumetric index keys
#[main_codec]
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VolumeKeys {
    pub volume_keys: Vec<VolumeKey>,
}



impl Default for VolumeKeys {
    fn default() -> Self {
        VolumeKeys {
            volume_keys: Default::default()
        }
    }
}

#[main_codec]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct VolumeKeyWithData {
    pub timestamp: u64,
    pub key: VolumeKey
}

impl Default for VolumeKeyWithData {
    fn default() -> Self {
        VolumeKeyWithData {
            timestamp:  Default::default(),
            key:  Default::default()
        }
    }
}



#[main_codec]
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct VolumeKeysWithData {
    pub volume_keys: Vec<VolumeKeyWithData>
}

impl Default for VolumeKeysWithData {
    fn default() -> Self {
        VolumeKeysWithData {
            volume_keys: Default::default()
        }
    }
}

