use redis::{aio::Connection, RedisError};
use serde_json;
use simplefi_engine_settings::load_settings;
use simplefi_redis::{
    add_to_set, connect, delete_from_hset, delete_multiple_from_hset, get_complete_hset,
    get_from_hset, is_in_set, store_in_hset,
};

use crate::protocol_driver::driver_traits::protocol_info::GetProtocolInfo;
use crate::protocol_driver::protocol_driver::SupportedProtocolDrivers;
use crate::utils::volumetrics::amalgamate_volumetrics_vecs;

use super::volumetrics::Volumetric;
pub struct ProtocolRedisDriver {
    connection: Connection,
}

impl ProtocolRedisDriver {
    pub fn resolve_set_name(
        &self,
        name: &str,
    ) -> String {
        format!("gold_protocol_driver_{}", name)
    }

    pub async fn new() -> Self {
        let mysettings = load_settings().expect("Failed to load settings");
        let redis_connection = connect(&mysettings.redis_uri)
            .await
            .expect("Expect to connect to redis");

        Self {
            connection: redis_connection,
        }
    }

    pub async fn set_market_driver(
        &mut self,
        market_address: String,
        protocol_driver_id: &str,
    ) -> Result<(), RedisError> {
        let list_name = self.resolve_set_name(&protocol_driver_id).clone();
        add_to_set(&mut self.connection, &list_name, &market_address).await
    }

    pub async fn is_protocol_market(
        &mut self,
        market_address: &str,
        protocol_id: &str,
    ) -> Result<bool, RedisError> {
        let list_name = self.resolve_set_name(&protocol_id).clone();
        is_in_set(&mut self.connection, &list_name, market_address).await
    }

    pub async fn match_protocol_from_market_address(
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
    // TODO:

    // get volumetric
    pub async fn get_active_volumes(
        &mut self,
        market_address: &str,
    ) -> Option<Vec<Volumetric>> {
        let hmap_name = self.resolve_set_name("volumetrics");
        let response = get_from_hset(&mut self.connection, &hmap_name, market_address).await;

        match response {
            Ok(res) => {
                // parse
                let volumes: Vec<Volumetric> = serde_json::from_str(&res).unwrap();
                Some(volumes)
            }
            _ => None,
        }
    }

    // set volumetric
    pub async fn set_volumes(
        &mut self,
        market_address: &str,
        volumes: Vec<Volumetric>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let hmap_name = self.resolve_set_name("volumetrics");
        let current_volumes = self.get_active_volumes(market_address).await;

        let new_volume_set = match current_volumes {
            Some(x) => amalgamate_volumetrics_vecs(x, volumes),
            None => volumes,
        };

        let json_volumes = serde_json::to_string(&new_volume_set).unwrap();

        let _ = store_in_hset(
            &mut self.connection,
            &hmap_name,
            market_address,
            &json_volumes,
        )
        .await;

        Ok(())
    }

    // overwrite market volumes
    pub async fn overwrite_markets_volumes(
        &mut self,
        data: Vec<(String, Vec<Volumetric>)>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let hmap_name = self.resolve_set_name("volumetrics");
        for market in data {
            let json_volumes = serde_json::to_string(&market.1).unwrap();
            let _ =
                store_in_hset(&mut self.connection, &hmap_name, &market.0, &json_volumes).await?;
        }
        Ok(())
    }

    // returns a vec of tuples, (market address, Vec<volumetric>)
    pub async fn get_all_volumes(&mut self) -> Vec<(String, Vec<Volumetric>)> {
        let hmap_name = self.resolve_set_name("volumetrics");
        let hmap = get_complete_hset(&mut self.connection, &hmap_name).await;

        let mut res: Vec<(String, Vec<Volumetric>)> = vec![];

        match hmap {
            Ok(mut hm) => {
                while hm.len() > 0 {
                    let key = hm.remove(0);
                    let value = hm.remove(0);
                    let decode: Vec<Volumetric> = serde_json::from_str(&value).unwrap();

                    res.push((key, decode));
                }
            }
            _ => (),
        }

        res
    }

    pub async fn remove_market_volumes(
        &mut self,
        market_address: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let hmap_name = self.resolve_set_name("volumetrics");
        let _ = delete_from_hset(&mut self.connection, &hmap_name, market_address).await?;

        Ok(())
    }

    pub async fn delete_markets_volumes(
        &mut self,
        market_addresses: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let hmap_name = self.resolve_set_name("volumetrics");
        let _ =
            delete_multiple_from_hset(&mut self.connection, &hmap_name, market_addresses).await?;

        Ok(())
    }
    // pub async fn set_volume(
    //     &mut self,
    //     market_address: &str,
    //     volume: Volumetric,
    // ) -> Result<(), Box<dyn std::error::Error>> {
    //     let hmap_name = self.resolve_set_name("volumetrics");

    //     let current_volumes = self.get_active_volumes(market_address).await;

    //     match current_volumes {
    //         // active volumes stored in hmap
    //         Some(mut volumes) => {
    //             volumes.push(volume);
    //             let json_volumes = serde_json::to_string(&volumes).unwrap();
    //             let _ = store_in_hset(
    //                 &mut self.connection,
    //                 &hmap_name,
    //                 market_address,
    //                 &json_volumes,
    //             )
    //             .await;
    //             Ok(())
    //         }
    //         // no previous volumes stored
    //         _ => {
    //             let mut volumes_to_set: Vec<Volumetric> = vec![];
    //             volumes_to_set.push(volume);
    //             let json_volumes = serde_json::to_string(&volumes_to_set).unwrap();
    //             let _ = store_in_hset(
    //                 &mut self.connection,
    //                 &hmap_name,
    //                 market_address,
    //                 &json_volumes,
    //             )
    //             .await;
    //             Ok(())
    //         }
    //     }
    // }

    // set snapshot

    // get snapshot

    // get all snapshots

    // get all volumetrics
}

#[cfg(test)]
mod tests {

    use ethers::types::U256;
    // use simplefi_redis::delete_set;

    use super::*;

    #[tokio::test]
    async fn test_set_get_volumes() {
        let mut redis_driver = ProtocolRedisDriver::new().await;

        let test_volume = Volumetric {
            timestamp: 1,
            swaps_in: vec![],
            swaps_out: vec![],
            withdrawal: vec![],
            transfer: U256::from(1),
            mint: vec![],
        };

        let ts = test_volume.timestamp;

        let _ = redis_driver.set_volumes("test", vec![test_volume]).await;

        let res = redis_driver.get_active_volumes("test").await.unwrap();

        assert_eq!(res[0].timestamp, ts);

        let _ = redis_driver.remove_market_volumes("test").await;
    }

    #[tokio::test]
    async fn test_set_multiple_volumes() {
        let mut redis_driver = ProtocolRedisDriver::new().await;

        let test_volume = Volumetric {
            timestamp: 1,
            swaps_in: vec![],
            swaps_out: vec![],
            withdrawal: vec![],
            transfer: U256::from(1),
            mint: vec![],
        };

        let ts = test_volume.timestamp;

        let _ = redis_driver
            .set_volumes("test", vec![test_volume.clone()])
            .await;
        let _ = redis_driver.set_volumes("test", vec![test_volume]).await;

        let res = redis_driver.get_active_volumes("test").await.unwrap();

        assert_eq!(res[0].timestamp, ts);
        assert_eq!(res[0].transfer, U256::from(1));
        let _ = redis_driver.remove_market_volumes("test").await;
    }

    #[tokio::test]
    async fn test_get_all_volumes() {
        let mut redis_driver = ProtocolRedisDriver::new().await;

        let test_volume = Volumetric {
            timestamp: 1,
            swaps_in: vec![],
            swaps_out: vec![],
            withdrawal: vec![],
            transfer: U256::from(1),
            mint: vec![],
        };
        let test_volume2 = Volumetric {
            timestamp: 2,
            swaps_in: vec![],
            swaps_out: vec![],
            withdrawal: vec![],
            transfer: U256::from(1),
            mint: vec![],
        };
        let _ = redis_driver
            .set_volumes("test", vec![test_volume.clone()])
            .await;
        let _ = redis_driver.set_volumes("test", vec![test_volume2]).await;
        let _ = redis_driver
            .set_volumes("test2", vec![test_volume.clone()])
            .await;

        let res = redis_driver.get_all_volumes().await;

        assert_eq!(res[0].0, "test");
        assert_eq!(res[1].0, "test2");
        assert_eq!(res[0].1.len(), 2);

        let _ = redis_driver.remove_market_volumes("test").await;
        let _ = redis_driver.remove_market_volumes("test2").await;
    }
}
