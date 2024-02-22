use std::collections::HashMap;

use chains_types::SupportedChains;
use chrono::{DateTime, NaiveDateTime, Timelike, Utc};
use polars::{
    export::num::Integer,
    lazy::dsl::{col, lit},
    prelude::{DataFrame, IntoLazy, NamedFrom},
    series::Series,
};

use crate::{
    mongo::protocol_status::types::ProtocolStatus,
    processes::mocks::generate_mock_creations::generate_mock_dataframe,
    protocol_driver::driver_traits::{
        market_creation_methods::MarketCreation, protocol_info::GetProtocolInfo,
    },
    protocol_driver::protocol_driver::{
        get_factory_address_list, match_protocol_from_factory_address, SupportedProtocolDrivers,
    },
    types::{redis_driver::ProtocolRedisDriver, volumetrics::Volumetric},
    utils::{
        date::round_timestamp,
        volumetrics::{
            amalgamate_volumetrics, amalgamate_volumetrics_vecs,
            create_five_min_volumetrics::create_five_min_volumetrics,
        },
    },
};

pub async fn process_from_parquet(
    from: i64,
    to: i64,
    chain: SupportedChains,
    redis_driver: &mut ProtocolRedisDriver,
    protocols_status: &Vec<ProtocolStatus>,
) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: replace mock
    let df = generate_mock_dataframe(from, to).await;

    // process cretions from logs
    process_creations_from_logs(&df, redis_driver).await;

    // process other logs
    let partitions = df.partition_by(["address"]).unwrap();

    for partition in partitions {
        //match driver
        let address = &partition
            .column("address")
            .unwrap()
            .utf8()
            .unwrap()
            .into_iter()
            .next()
            .unwrap()
            .unwrap();

        let matched_driver = redis_driver
            .match_protocol_from_market_address(address)
            .await;

        match matched_driver {
            // skip if not driver found
            None => continue,
            Some(driver) => {
                let status = match_driver_status(&driver, protocols_status);

                match status {
                    None => continue,
                    Some(stat) => {
                        // process volumetrics
                        process_volumetrics(&partition, stat, redis_driver, address, &driver).await;
                        // process snapshots
                    }
                }
            }
        }
    }

    Ok(())
}

async fn process_volumetrics(
    events: &DataFrame,
    protocol_status: ProtocolStatus,
    redis_driver: &mut ProtocolRedisDriver,
    market_address: &str,
    driver: &SupportedProtocolDrivers,
) -> Result<(), Box<dyn std::error::Error>> {
    let first_timestamp = events
        .sort(["timestamp"], false)
        .unwrap()
        .column("timestamp")
        .unwrap()
        .u64()
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
        .unwrap();

    let first_period_ts = round_timestamp(5, &first_timestamp);

    let volumetrics = create_five_min_volumetrics(events.clone().lazy(), driver).await;

    let _ = redis_driver.set_volumes(market_address, volumetrics).await;

    Ok(())
}

async fn get_active_volume(
    redis_driver: &mut ProtocolRedisDriver,
    market_address: &str,
    period_ts: u64,
) -> Option<Volumetric> {
    let previous_period_volumes = redis_driver.get_active_volumes(market_address).await;
    match previous_period_volumes {
        Some(volumes) => {
            let matched_index = volumes.iter().position(|v| v.timestamp == period_ts);
            match matched_index {
                Some(x) => Some(volumes[x].clone()),
                _ => None,
            }
        }
        _ => None,
    }
}

fn match_driver_status(
    driver: &SupportedProtocolDrivers,
    protocols_status: &Vec<ProtocolStatus>,
) -> Option<ProtocolStatus> {
    let status = protocols_status
        .iter()
        .position(|status| status.protocol_id == driver.get_protocol_info().name);

    match status {
        Some(i) => {
            return Some(protocols_status[i].clone());
        }
        None => return None,
    }
}

async fn process_creations_from_logs(
    df: &DataFrame,
    redis_driver: &mut ProtocolRedisDriver,
) {
    let factories = Series::new("factories", get_factory_address_list());
    let filter_exp = col("address").is_in(lit(factories));
    let factory_df = df.clone().lazy().filter(filter_exp).collect().unwrap();

    let partitions = factory_df.partition_by_stable(["address"]).unwrap();

    for partition in partitions {
        let address = partition
            .column("address")
            .unwrap()
            .utf8()
            .unwrap()
            .into_iter()
            .next()
            .unwrap()
            .unwrap();

        let driver = match_protocol_from_factory_address(address);

        match driver {
            Some(d) => {
                let creation_addresses = d.get_created_market_addresses(df.clone());

                for new_market in creation_addresses {
                    redis_driver
                        .set_market_driver(new_market, &d.get_protocol_info().name)
                        .await
                        .unwrap();
                }
            }
            _ => continue,
        }
    }
}

/*
// from parquet (possible stream), await
  get logs for that starting day that are greater than lowest timestamp (same day as timestamp)
    filter for logs that have address from factory address list
    bin by address
    for each factor address dataframe
      get the matching protocol driver
      get new market addresses from logs
      save market address using redis driver

    after processing factory logs:
      bin original df by address
      for each address
        check redis driver if address is in any of the sets
        if in set
          check protocol status for last timestamp checked, if older, do nothing
          if newer
             check redis if snapshots/volumetrics exists for that address
              if they do, check if same period (5 min, hour, day)
                if same periods, use as base for new snapshots/volumetrics
                if not same periods, create new snapshots to use as (using previous figures for snapshots), store previous periods in mongo and clean redis
                use matched driver to process logs and create snapshots/volumetrics
                update protocol status last updated timestamp

      update last_day_processed + 1
*/
