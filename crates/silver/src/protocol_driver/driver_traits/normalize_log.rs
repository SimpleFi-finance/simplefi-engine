use bronze::mongo::evm::data_sets::logs::Log;
use polars::prelude::DataFrame;

use crate::{
    protocol_driver::protocol_driver::SupportedProtocolDrivers,
    protocol_driver::protocols::uniswap::uniswap_v2_mainnet,
};

pub trait NormalizeLogs {
    fn normalize_logs(
        &self,
        logs: Vec<Log>,
    ) -> DataFrame;
}

impl NormalizeLogs for SupportedProtocolDrivers {
    fn normalize_logs(
        &self,
        logs: Vec<Log>,
    ) -> DataFrame {
        match self {
            SupportedProtocolDrivers::UniswapV2Mainnet => uniswap_v2_mainnet::normalize_logs(logs),
        }
    }
}
