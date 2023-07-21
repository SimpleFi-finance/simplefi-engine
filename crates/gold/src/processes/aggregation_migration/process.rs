use core::num::flt2dec::strategy::dragon;
use std::env;

use chains_types::get_chain;
use chrono::Utc;

use crate::{
    dragonfly::{daily_aggregation::DailyAggregation, dragonfly_driver::ProtocolDragonflyDriver},
    processes::aggregation_migration::daily::daily_aggregation,
};

use super::hourly::hourly_aggregation;

pub async fn aggregation_migration() -> Result<(), Box<dyn std::error::Error>> {
    let chain_id = env::var("CHAIN_ID").unwrap();
    let chain = get_chain(&chain_id).unwrap().to_string();
    let mut dragonfly_driver = ProtocolDragonflyDriver::new(&chain).await;

    let _ = hourly_aggregation(&mut dragonfly_driver).await?;

    // if it's now a new day and should process previous days hourly....
    let latest_daily_aggregation = dragonfly_driver.get_latest_aggregation_ts().await?;
    let now = Utc::now().timestamp_micros() as u64;

    if now >= latest_daily_aggregation + 86400000 {
        daily_aggregation(&mut dragonfly_driver).await?;
    }

    Ok(())
}

/*
  process hourly

  if it's the final hour of the day (timestamp for hourly to process is midnight), process daily

*/
