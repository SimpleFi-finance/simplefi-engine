use crate::protocols_driver::protocols_driver::SupportedProtocolDrivers;
use crate::types::ProtocolInfo;

// protocol crates
use crate::protocols::uniswap::uniswap_v2_mainnet;

trait GetProtocolInfo {
    fn get_protocol_info(&self) -> ProtocolInfo;
}

impl GetProtocolInfo for SupportedProtocolDrivers {
    fn get_protocol_info(&self) -> ProtocolInfo {
        match self {
            SupportedProtocolDrivers::UniswapV2Mainnet => uniswap_v2_mainnet::get_protocol_info(),
        }
    }
}
