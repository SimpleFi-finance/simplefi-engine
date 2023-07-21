use chrono::Utc;

use crate::{
    mongo::volumetrics::utils::shared::get_month_year_day,
    types::{redis_driver::ProtocolRedisDriver, volumetrics::Volumetric},
    utils::date::round_down_timestamp,
};

pub async fn migrate_to_mongo() -> Result<(), Box<dyn std::error::Error>> {
    let mut redis_driver = ProtocolRedisDriver::new().await;
    let stored_market_volumes = redis_driver.get_all_volumes().await;
    let period_cut_off =
        round_down_timestamp(5, &Utc::now().timestamp_millis().try_into().unwrap());

    let mut markets_with_no_remaining_volumes = vec![];
    let mut active_market_volumes = vec![];
    let mut volumes_to_migrate = vec![];

    for (market_address, volumes) in stored_market_volumes {
        // iterate through, creating

        let market_volumes_filtered: (Vec<Volumetric>, Vec<Volumetric>) = volumes
            .iter()
            .cloned()
            .fold((vec![], vec![]), |mut acc, v| {
                if v.timestamp < period_cut_off {
                    acc.0.push(v);
                } else {
                    acc.1.push(v);
                }

                acc
            });

        if market_volumes_filtered.1.len() == 0 {
            markets_with_no_remaining_volumes.push(market_address.clone())
        } else {
            active_market_volumes.push((market_address.clone(), market_volumes_filtered.1))
        }

        if market_volumes_filtered.0.len() > 0 {
            volumes_to_migrate.push((market_address, market_volumes_filtered.0))
        }
    }

    // remove empty markets from cache
    redis_driver
        .delete_markets_volumes(markets_with_no_remaining_volumes)
        .await?;

    // set remaining active volumes in cache
    redis_driver
        .overwrite_markets_volumes(active_market_volumes)
        .await?;

    // set complete volumes in mongo

    /*
        For each market and each volume (map/fold)
            get the year,month,day (to find the doc)
            define the update logic (push into mapping, filters to find doc, upsert logic if not found)

        bulk save
            error handling


    */
    // let updates = vec![];

    // for market in volumes_to_migrate {
    //     for volume in market.1 {
    //         let (month, year, day) = get_month_year_day(volume.timestamp);

    //     }
    // }

    todo!();
}
