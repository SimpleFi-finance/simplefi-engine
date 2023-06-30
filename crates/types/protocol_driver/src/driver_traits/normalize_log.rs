use polars::prelude::DataFrame;
use shared_types::mongo::bronze::evm::logs::Log;

use crate::{
    protocols::uniswap::uniswap_v2_mainnet,
    protocols_driver::protocols_driver::SupportedProtocolDrivers,
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
