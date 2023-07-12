use crate::protocol_driver::protocol_driver::SupportedProtocolDrivers;
use crate::protocol_driver::protocols::uniswap::uniswap_v2_mainnet;
use crate::types::volumetrics::Volumetric;

use polars::prelude::DataFrame;

pub trait VolumetricMethods {
    fn volumes_from_dataframe_slice(
        &self,
        df: &DataFrame,
    ) -> Volumetric;
}

impl VolumetricMethods for SupportedProtocolDrivers {
    fn volumes_from_dataframe_slice(
        &self,
        df: &DataFrame,
    ) -> Volumetric {
        match self {
            SupportedProtocolDrivers::UniswapV2Mainnet => {
                uniswap_v2_mainnet::volumes_from_dataframe_slice(df)
            }
        }
    }
}
