use crate::protocol_driver::protocol_driver::SupportedProtocolDrivers;
use crate::protocol_driver::protocols::uniswap::uniswap_v2_mainnet;
use crate::types::volumetrics::Volumes;

use polars::prelude::DataFrame;

pub trait VolumetricMethods {
    fn volumes_from_dataframe_slice(
        &self,
        df: DataFrame,
    ) -> Volumes;
}

impl VolumetricMethods for SupportedProtocolDrivers {
    fn volumes_from_dataframe_slice(
        &self,
        df: DataFrame,
    ) -> Volumes {
        match self {
            SupportedProtocolDrivers::UniswapV2Mainnet => {
                uniswap_v2_mainnet::volumes_from_dataframe_slice(df)
            }
        }
    }
}
