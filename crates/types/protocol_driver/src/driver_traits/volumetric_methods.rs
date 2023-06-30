use crate::protocols::uniswap::uniswap_v2_mainnet;
use crate::protocols_driver::protocols_driver::SupportedProtocolDrivers;
use polars::prelude::DataFrame;
use shared_types::gold::volumetrics::Volumes;

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
