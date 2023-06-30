use polars::prelude::DataFrame;
use shared_types::mongo::bronze::evm::logs::Log;

use crate::protocols_driver::factory_addresses::match_protocol_from_factory_address;
use crate::protocols_driver::protocols_driver::SupportedProtocolDrivers;

// protocol crates
use crate::protocols::uniswap::uniswap_v2_mainnet;

pub trait MarketCreation {
    fn get_created_market_addresses(
        &self,
        df: DataFrame,
    ) -> Vec<String>;

    fn get_created_market_address(
        &self,
        log: Log,
    ) -> String;
}

impl MarketCreation for SupportedProtocolDrivers {
    fn get_created_market_addresses(
        &self,
        df: DataFrame,
    ) -> Vec<String> {
        match self {
            SupportedProtocolDrivers::UniswapV2Mainnet => {
                uniswap_v2_mainnet::get_created_market_addresses(df)
            }
        }
    }

    fn get_created_market_address(
        &self,
        log: Log,
    ) -> String {
        match self {
            SupportedProtocolDrivers::UniswapV2Mainnet => {
                uniswap_v2_mainnet::get_created_market_address(log)
            }
        }
    }
}
