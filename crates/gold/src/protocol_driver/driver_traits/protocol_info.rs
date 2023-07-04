use crate::types::protocols::ProtocolInfo;

// protocol crates
use crate::protocol_driver::protocol_driver::SupportedProtocolDrivers;
use crate::protocol_driver::protocols::uniswap::uniswap_v2_mainnet;

pub trait GetProtocolInfo {
    fn get_protocol_info(&self) -> ProtocolInfo;
}

impl GetProtocolInfo for SupportedProtocolDrivers {
    fn get_protocol_info(&self) -> ProtocolInfo {
        match self {
            SupportedProtocolDrivers::UniswapV2Mainnet => uniswap_v2_mainnet::get_protocol_info(),
        }
    }
}
