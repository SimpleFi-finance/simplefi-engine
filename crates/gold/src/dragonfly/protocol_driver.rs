use async_trait::async_trait;
use redis::RedisError;
use simplefi_redis::{add_to_set, is_in_set};

use crate::protocol_driver::driver_traits::protocol_info::GetProtocolInfo;
use crate::protocol_driver::protocol_driver::SupportedProtocolDrivers;

use super::dragonfly_driver::ProtocolDragonflyDriver;

#[async_trait]
pub trait ProtocolsDriver {
    async fn set_market_driver(
        &mut self,
        market_address: String,
        protocol_driver_id: &str,
    ) -> Result<(), RedisError>;

    async fn is_protocol_market(
        &mut self,
        market_address: &str,
        protocol_id: &str,
    ) -> Result<bool, RedisError>;

    async fn match_protocol_from_market_address(
        &mut self,
        address: &str,
    ) -> Option<SupportedProtocolDrivers>;
}

#[async_trait]
impl ProtocolsDriver for ProtocolDragonflyDriver {
    async fn set_market_driver(
        &mut self,
        market_address: String,
        protocol_id: &str,
    ) -> Result<(), RedisError> {
        let list_name = format!("{}_gold_{}", &self.chain, protocol_id);
        add_to_set(&mut self.connection, &list_name, &market_address).await
    }

    async fn is_protocol_market(
        &mut self,
        market_address: &str,
        protocol_id: &str,
    ) -> Result<bool, RedisError> {
        let list_name = format!("{}_gold_{}", &self.chain, protocol_id);
        is_in_set(&mut self.connection, &list_name, market_address).await
    }

    async fn match_protocol_from_market_address(
        &mut self,
        address: &str,
    ) -> Option<SupportedProtocolDrivers> {
        // uniswap v2 mainnet
        let proto_name = &SupportedProtocolDrivers::UniswapV2Mainnet
            .get_protocol_info()
            .name;
        let uni_v2_mainnet_check = self.is_protocol_market(address, proto_name).await.unwrap();

        if uni_v2_mainnet_check {
            return Some(SupportedProtocolDrivers::UniswapV2Mainnet);
        }

        None
    }
}
