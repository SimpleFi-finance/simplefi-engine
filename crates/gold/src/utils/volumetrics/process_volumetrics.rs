use polars::prelude::{DataFrame, IntoLazy};

use crate::{
    protocol_driver::protocol_driver::SupportedProtocolDrivers, types::volumetrics::Volumetric,
    utils::date::round_timestamp,
};

use super::create_five_min_volumetrics::create_five_min_volumetrics;

pub async fn process_volumetrics(
    events: &DataFrame,
    // protocol_status: ProtocolStatus,
    // redis_driver: &mut ProtocolRedisDriver,
    // market_address: &str,
    driver: &SupportedProtocolDrivers,
) -> Vec<Volumetric> {
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

    create_five_min_volumetrics(events.clone().lazy(), driver).await

    // let _ = redis_driver.set_volumes(market_address, volumetrics).await;
}
