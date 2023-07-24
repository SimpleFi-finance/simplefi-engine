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
        let normalized =
            check_driver_and_normalize(address_logs.0, address_logs.1, &mut dragonfly_driver)
                .await?;

        match normalized {
            Some(data_to_process) => {
                // push only if is in filtered protocol status
                let name = data_to_process.1.get_driver_name();
                let matched_index = protocols_status.iter().position(|p| p.protocol_id == name);

                match matched_index {
                    Some(i) => logs_to_process.push(data_to_process),
                    _ => (),
                }
            }
            _ => (),
        }
    }

    // let mut protocol_latest_block_processed: HashMap<String, u64> = HashMap::new();

    // process dataframes
    // TODO: Switch to multithreading thread pool system to increase performance
    let mut new_volumes_to_store = vec![];
    for df_with_driver in logs_to_process {
        // process volumetrics
        let new_volumes = process_volumetrics(&df_with_driver.2, &df_with_driver.1).await;
        new_volumes_to_store.push((df_with_driver.0, new_volumes));

        // let name = df_with_driver.1.get_protocol_info().name;

        // let existing = protocol_latest_block_processed.get(&name);

        // match existing {
        //     Some(latest_block) => {
        //         // let block_series = df_with_driver.2.column("block_number").unwrap();
        //         // let latest_processed = block_series.u64().unwrap().max().unwrap();
        //         let latest_processed: u64 = df_with_driver.2["block_number"].max().unwrap();
        //         if latest_processed > latest_block.clone() {
        //             protocol_latest_block_processed.insert(name, latest_processed);
        //         };
        //     }
        //     _ => (),
        // }
        // process snapshots
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

    todo!();
}

fn bin_logs_by_address(logs: Vec<Log>) -> (HashMap<String, Vec<Log>>, u64) {
    let mut hmap: HashMap<String, Vec<Log>> = HashMap::new();
    let mut latest_ts_processed = 0;

    for log in logs {
        if log.timestamp > latest_ts_processed {
            latest_ts_processed = log.timestamp.clone()
        }
        match log.address {
            Some(address) => {
                let existing = hmap.get(&address);
                match existing {
                    Some(stored_logs) => stored_logs.push(log),
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
) -> Result<Option<(String, SupportedProtocolDrivers, DataFrame)>, Box<dyn std::error::Error>> {
    let protocol_driver = dragonfly_driver
        .match_protocol_from_market_address(&address)
        .await?;

    match protocol_driver {
        Some(driver) => {
            let normalized_logs = driver.normalize_logs(logs);
            Ok(Some((address, driver, normalized_logs)))
        }
        _ => Ok(None),
    }
}

/*
 Update protocols


   get all protocol status'
   find oldest last timestamp from protocol status' that is within the threshold (1 week/2 days??)

   get logs from mongo > that oldest last timestamp
   bin by address into a hashmap
   get hashmap entries for logs that have address from factory address list
     for each factor address dataframe
       get the matching protocol driver
       get new market addresses from logs (method for getting it from mongo logs?)
       save market address using redis driver

   after processing factory logs:
       for each address in hashmap
       check redis driver if address is in any of the sets
       if in set
       check protocol status for last timestamp checked, if older, do nothing
       if newer
             normalize logs
             check redis if snapshots/volumetrics exists for that address
               if they do, check if same period (5 min, hour, day)
                 if same periods, use as base for new snapshots/volumetrics
                 if not same periods, create new snapshots to use as (using previous figures for snapshots), store previous periods in mongo and clean redis
                 use matched driver to process logs and create snapshots/volumetrics
                 save most recent 5min,1hour,1day snapshot/volumetric in redis for that address, save all older in mongo
                 update protocol status last updated timestamp

*/

/*
   most of the logic here is reused from backfil logic.
   Only difference is this never checks parquet, doesn't iterate through days
   all processing logic should be reusable
*/
