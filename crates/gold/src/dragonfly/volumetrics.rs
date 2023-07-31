use std::collections::HashMap;

use async_trait::async_trait;
use simplefi_redis::{
    delete_multiple_from_hset, get_complete_hset, get_from_hset, get_hset_keys, store_in_hset,
    store_multiple_in_hset,
};

use crate::{
    types::{shared::Timeframe, volumetrics::Volumetric},
    utils::volumetrics::amalgamate_volumetrics_vecs,
};

use super::dragonfly_driver::ProtocolDragonflyDriver;

#[async_trait]
pub trait VolumetricsTrait {
    async fn get_active_volumes(
        &mut self,
        market_address: &str,
        timeframe: &Timeframe,
    ) -> Result<Vec<Volumetric>, Box<dyn std::error::Error>>;
    async fn set_volumes(
        &mut self,
        market_address: &str,
        volumes: Vec<Volumetric>,
        timeframe: &Timeframe,
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn set_multiple_volumes(
        &mut self,
        market_volumes: Vec<(String, Vec<Volumetric>)>,
        timeframe: &Timeframe,
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn delete_markets_volumes(
        &mut self,
        market_addresses: Vec<String>,
        timeframe: &Timeframe,
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn delete_outdated_volumes(
        &mut self,
        ts_cutoff: u64,
        timeframe: &Timeframe,
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn get_all_volumes(
        &mut self,
        timeframe: &Timeframe,
        ts_cutoff: Option<u64>,
    ) -> Result<Vec<(String, Vec<Volumetric>)>, Box<dyn std::error::Error>>;

    async fn overwrite_markets_volumes(
        &mut self,
        data: Vec<(String, Vec<Volumetric>)>,
        timeframe: &Timeframe,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

#[async_trait]
impl VolumetricsTrait for ProtocolDragonflyDriver {
    async fn get_active_volumes(
        &mut self,
        market_address: &str,
        timeframe: &Timeframe,
    ) -> Result<Vec<Volumetric>, Box<dyn std::error::Error>> {
        let hmap_name = self
            .resolve_collection_name(data_lake_types::SupportedDataTypes::Volumetric, &timeframe);

        let response = get_from_hset(&mut self.connection, &hmap_name, market_address).await?;

        let volumes: Vec<Volumetric> = serde_json::from_str(&response).unwrap();

        Ok(volumes)
    }

    async fn set_volumes(
        &mut self,
        market_address: &str,
        volumes: Vec<Volumetric>,
        timeframe: &Timeframe,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let hmap_name = self
            .resolve_collection_name(data_lake_types::SupportedDataTypes::Volumetric, timeframe);

        let current_volumes = self.get_active_volumes(market_address, timeframe).await?;

        let new_volume_set = if current_volumes.len() > 0 {
            amalgamate_volumetrics_vecs(current_volumes, volumes)
        } else {
            volumes
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

    // Caution, each key will overwrite any existing values
    // TODO: add logic to ensure all volumes in vec are for the same period
    async fn set_multiple_volumes(
        &mut self,
        market_volumes: Vec<(String, Vec<Volumetric>)>,
        period_timeframe: &Timeframe,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let formatted_keys: Vec<(String, String)> = market_volumes
            .into_iter()
            .map(|mv| {
                let formatted_key = format!(
                    "{}_{}",
                    mv.0,
                    period_timeframe.round_timestamp(&mv.1[0].timestamp)
                );

                let formatted_data: String = serde_json::to_string(&mv.1).unwrap();
                (formatted_key, formatted_data)
            })
            .collect();

        let hmap_name = self.resolve_collection_name(
            data_lake_types::SupportedDataTypes::Volumetric,
            period_timeframe,
        );

        store_multiple_in_hset(&mut self.connection, &hmap_name, formatted_keys);
        Ok(())
    }

    async fn delete_markets_volumes(
        &mut self,
        market_addresses: Vec<String>,
        timeframe: &Timeframe,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let hmap_name = self
            .resolve_collection_name(data_lake_types::SupportedDataTypes::Volumetric, timeframe);
        let _ =
            delete_multiple_from_hset(&mut self.connection, &hmap_name, market_addresses).await?;

        Ok(())
    }

    async fn delete_outdated_volumes(
        &mut self,
        ts_cutoff: u64,
        timeframe: &Timeframe,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let hmap_name = self
            .resolve_collection_name(data_lake_types::SupportedDataTypes::Volumetric, timeframe);

        let all_keys = get_hset_keys(&mut self.connection, &hmap_name).await?;

        let outdated_keys: Vec<String> = all_keys
            .iter()
            .cloned()
            .filter(|key| {
                let (_market_name, timestamp) = self.split_field_key(key);
                if timestamp < ts_cutoff {
                    return true;
                }
                return false;
            })
            .collect();

        delete_multiple_from_hset(&mut self.connection, &hmap_name, outdated_keys).await?;

        Ok(())
    }

    // gets all period partitions for all markets in hmap, volumetrics periods are combined for each market
    // to produce a single vector of volumes for each market
    // An optional timestamp cutoff is allowed to filter only volumes older than said timestamp
    async fn get_all_volumes(
        &mut self,
        timeframe: &Timeframe,
        ts_cutoff: Option<u64>,
    ) -> Result<Vec<(String, Vec<Volumetric>)>, Box<dyn std::error::Error>> {
        let hmap_name = self
            .resolve_collection_name(data_lake_types::SupportedDataTypes::Volumetric, timeframe);
        let mut hmap = get_complete_hset(&mut self.connection, &hmap_name).await?;

        let mut result_hash: HashMap<String, Vec<Volumetric>> = HashMap::new();

        while hmap.len() > 0 {
            let key = hmap.remove(0);
            let value = hmap.remove(0);

            match ts_cutoff {
                Some(ts) => {
                    let (market_address, timestamp) = self.split_field_key(&key);

                    if timestamp <= ts {
                        let mut decode: Vec<Volumetric> = serde_json::from_str(&value).unwrap();
                        let existing = result_hash.get(&market_address);

                        match existing {
                            Some(ex) => {
                                // ex.append(&mut decode);
                                decode.extend(ex.into_iter().cloned());
                                result_hash.insert(market_address, decode.to_vec());
                            }
                            _ => {
                                result_hash.insert(market_address, decode);
                            }
                        }
                    }
                }
                _ => (),
            }
        }

        let x = result_hash
            .drain()
            .map(|x| (x.0.to_string(), x.1))
            .collect();

        Ok(x)
    }

    async fn overwrite_markets_volumes(
        &mut self,
        data: Vec<(String, Vec<Volumetric>)>,
        timeframe: &Timeframe,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let hmap_name = self
            .resolve_collection_name(data_lake_types::SupportedDataTypes::Volumetric, timeframe);
        for market in data {
            let json_volumes = serde_json::to_string(&market.1).unwrap();
            let _ =
                store_in_hset(&mut self.connection, &hmap_name, &market.0, &json_volumes).await?;
        }
        Ok(())
    }
}
