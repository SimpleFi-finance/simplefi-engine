use crate::protocol_driver::driver_traits::market_creation_methods::MarketCreation;
use crate::protocol_driver::driver_traits::protocol_info::GetProtocolInfo;
use crate::protocol_driver::protocol_driver::match_protocol_from_factory_address;
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
use polars::prelude::DataFrame;

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
    let binned_logs = bin_logs_by_address(new_events);

    let mut logs_to_process: Vec<(SupportedProtocolDrivers, DataFrame)> = vec![];

    // find drivers, bin dataframes
    for address_logs in binned_logs {
        // check for factory address (creation log)
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

        // check if we need to process this address
        let normalized =
            check_driver_and_normalize(address_logs.0, address_logs.1, &mut dragonfly_driver)
                .await?;

        match normalized {
            Some(data_to_process) => logs_to_process.push(data_to_process),
            _ => (),
        }
    }

    // process dataframes

    todo!();
}

fn bin_logs_by_address(logs: Vec<Log>) -> HashMap<String, Vec<Log>> {
    let mut hmap: HashMap<String, Vec<Log>> = HashMap::new();

    for log in logs {
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
    hmap
}

async fn check_driver_and_normalize(
    address: String,
    logs: Vec<Log>,
    dragonfly_driver: &mut ProtocolDragonflyDriver,
) -> Result<Option<(SupportedProtocolDrivers, DataFrame)>, Box<dyn std::error::Error>> {
    let protocol_driver = dragonfly_driver
        .match_protocol_from_market_address(&address)
        .await?;

    match protocol_driver {
        Some(driver) => {
            let normalized_logs = driver.normalize_logs(logs);
            Ok(Some((driver, normalized_logs)))
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
