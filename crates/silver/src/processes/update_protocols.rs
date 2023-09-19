use crate::dragonfly::volumetrics::VolumetricsTrait;
use crate::protocol_driver::driver_traits::market_creation_methods::MarketCreation;
use crate::protocol_driver::driver_traits::protocol_info::GetProtocolInfo;
use crate::protocol_driver::protocol_driver::match_protocol_from_factory_address;
use crate::types::shared::Timeframe;
use crate::utils::volumetrics::process_volumetrics::process_volumetrics;
use crate::{
    dragonfly::protocol_driver::ProtocolsDriver,
    protocol_driver::protocol_driver::SupportedProtocolDrivers,
};
use std::{collections::HashMap, env};

use bronze::mongo::evm::data_sets::logs::Log;
use chains_types::get_chain;
use chrono::Utc;

use crate::protocol_driver::driver_traits::normalize_log::NormalizeLogs;
use crate::{
    dragonfly::{dragonfly_driver::ProtocolDragonflyDriver, protocol_status::ProtocolStatusTrait},
    processes::mocks::generate_mock_mongo_creations::generate_mock_mongo_creations_update,
    types::protocols::ProtocolStatus,
    utils::date::round_down_timestamp,
};
use polars::prelude::{ChunkAgg, DataFrame};

async fn update_protocols() -> Result<(), Box<dyn std::error::Error>> {
    let chain_id = env::var("CHAIN_ID").unwrap();
    let chain = get_chain(&chain_id).unwrap().to_string();
    let mut dragonfly_driver = ProtocolDragonflyDriver::new(&chain).await;
    let protocols_status = dragonfly_driver.get_all_protocols().await?;
    let now = Utc::now().timestamp_millis() as u64;
    let ms_and_hour_ago = &now - 3600000;
    let threshold = round_down_timestamp(60, &ms_and_hour_ago);

    let protocols_to_update: Vec<&ProtocolStatus> = protocols_status
        .iter()
        .filter(|p| {
            if p.last_sync_block_timestamp >= threshold && p.should_update {
                return true;
            }
            return false;
        })
        .collect();

    let oldest_update = protocols_to_update.iter().fold(now, |mut acc, p| {
        if p.last_sync_block_timestamp < acc {
            acc = p.last_sync_block_timestamp;
        }
        acc
    });

    // TODO: replace with real data
    let new_events = generate_mock_mongo_creations_update(oldest_update).await;
    let (binned_logs, latest_ts_processed) = bin_logs_by_address(new_events);

    let mut logs_to_process: Vec<(String, SupportedProtocolDrivers, DataFrame)> = vec![];

    // find drivers, bin dataframes
    for address_logs in binned_logs {
        // check for factory address (creation log) and add new markets to protocol drivers
        match match_protocol_from_factory_address(&address_logs.0) {
            Some(driver) => {
                let normalized = driver.normalize_logs(address_logs.1);
                let creation_addresses = driver.get_created_market_addresses(normalized);
                for new_market in creation_addresses {
                    dragonfly_driver
                        .set_market_driver(new_market, &driver.get_protocol_info().name)
                        .await
                        .unwrap();
                }
                continue;
            }
            _ => (),
        }

        // check if we have a driver for this address
        let normalized = check_driver_and_normalize(
            address_logs.0,
            address_logs.1,
            &mut dragonfly_driver,
            &protocols_to_update,
        )
        .await?;

        match normalized {
            Some(data_to_process) => logs_to_process.push(data_to_process),
            _ => (),
        }
    }

    // TODO: Switch to multithreading thread pool system to increase performance
    let mut new_volumes_to_store = vec![];
    for df_with_driver in logs_to_process {
        // process volumetrics
        let new_volumes = process_volumetrics(&df_with_driver.2, &df_with_driver.1).await;
        new_volumes_to_store.push((df_with_driver.0, new_volumes));
    }

    // store new volumes in redis
    dragonfly_driver
        .set_multiple_volumes(new_volumes_to_store, &Timeframe::FiveMinute)
        .await?;

    // delete logs from redis.
    // TODO: implement
    

    // update protocol drivers with latest blocks processed
    let mut updated_protocols: Vec<ProtocolStatus> = vec![];

    for proto in protocols_to_update {
        let mut new = proto.clone();
        new.last_sync_block_timestamp = latest_ts_processed;
        updated_protocols.push(new);
    }
    dragonfly_driver
        .update_many_protocol_status(updated_protocols)
        .await?;

    Ok(())
}

fn bin_logs_by_address(logs: Vec<Log>) -> (HashMap<String, Vec<Log>>, u64) {
    let mut hmap: HashMap<String, Vec<Log>> = HashMap::new();
    let mut latest_ts_processed = 0;

    for log in logs {
        if log.timestamp > latest_ts_processed {
            latest_ts_processed = log.timestamp.clone()
        }

        let market_address = log.address.clone();
        match market_address {
            Some(address) => {
                let existing = hmap.get(&address);
                match existing {
                    Some(stored_logs) => {
                        let mut new = stored_logs.clone();
                        new.push(log);
                        hmap.insert(address, new);
                    }
                    _ => {
                        hmap.insert(address, vec![log]);
                    }
                }
            }
            _ => (),
        }
    }
    (hmap, latest_ts_processed as u64)
}

async fn check_driver_and_normalize(
    address: String,
    logs: Vec<Log>,
    dragonfly_driver: &mut ProtocolDragonflyDriver,
    protocols_status: &Vec<&ProtocolStatus>,
) -> Result<Option<(String, SupportedProtocolDrivers, DataFrame)>, Box<dyn std::error::Error>> {
    let protocol_driver = dragonfly_driver
        .match_protocol_from_market_address(&address)
        .await?;

    match protocol_driver {
        Some(driver) => {
            let matched_status_i = protocols_status
                .iter()
                .position(|p| p.protocol_id == driver.get_protocol_info().name);

            match matched_status_i {
                Some(i) => {
                    let mut filtered_logs: Vec<Log> = vec![];
                    let protocol_status = protocols_status[i];

                    for log in logs {
                        if log.timestamp as u64 >= protocol_status.last_sync_block_timestamp {
                            filtered_logs.push(log);
                        }
                    }

                    let normalized_logs = driver.normalize_logs(filtered_logs);
                    Ok(Some((address, driver, normalized_logs)))
                }
                _ => Ok(None),
            }
        }
        _ => Ok(None),
    }
}
