use crate::types::protocols::{ProtocolInfo, ProtocolStatus};

// protocol crates
use crate::protocol_driver::protocol_driver::SupportedProtocolDrivers;
use crate::protocol_driver::protocols::uniswap::uniswap_v2_mainnet;

pub trait GetProtocolInfo {
    fn get_protocol_info(&self) -> ProtocolInfo;
    // fn match_protocol_status(
    //     &self,
    //     protocols_status: Vec<ProtocolStatus>,
    // ) -> Option<ProtocolStatus>;
}

impl GetProtocolInfo for SupportedProtocolDrivers {
    fn get_protocol_info(&self) -> ProtocolInfo {
        match self {
            SupportedProtocolDrivers::UniswapV2Mainnet => uniswap_v2_mainnet::get_protocol_info(),
        }
    }

    // fn match_protocol_status(
    //     &self,
    //     protocols_status: Vec<ProtocolStatus>,
    // ) -> Option<ProtocolStatus> {
    //     let info = self.get_protocol_info();
    //     let matched_index = protocols_status
    //         .iter()
    //         .position(|p| p.protocol_id == info.name);

    //     match matched_index {
    //         Some(i) => Some(protocols_status[i]),
    //         _ => None,
    //     }
    // }
}
