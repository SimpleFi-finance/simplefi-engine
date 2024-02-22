use chrono::Utc;

use crate::{
    dragonfly::{dragonfly_driver::ProtocolDragonflyDriver, volumetrics::VolumetricsTrait},
    types::shared::Timeframe,
    utils::volumetrics::{aggregate_volumetrics::aggregate_volumetrics, split_by_period},
};

pub async fn hourly_aggregation(
    dragonfly_driver: &mut ProtocolDragonflyDriver
) -> Result<(), Box<dyn std::error::Error>> {
    let _ = hourly_aggregation_volumetrics(dragonfly_driver).await?;
    // TODO: hourly_aggregation_snapshots

    Ok(())
}

async fn hourly_aggregation_volumetrics(
    dragonfly_driver: &mut ProtocolDragonflyDriver
) -> Result<(), Box<dyn std::error::Error>> {
    let ts = Utc::now().timestamp_micros() as u64;
    let period_timeframe = Timeframe::Daily;
    let period_hourly_timeframe = Timeframe::Hourly;
    let previous_period_end = period_hourly_timeframe.round_down_timestamp(&ts);

    // get five minute volumes
    let stored_five_min_volumes = dragonfly_driver
        .get_all_volumes(&Timeframe::FiveMinute, Some(previous_period_end))
        .await?;

    // aggregate into hourly and create daily period arrays of hourly
    let mut hourly = vec![];

    for market_volumes in stored_five_min_volumes {
        let hourly_volumes = aggregate_volumetrics(&market_volumes.1, &Timeframe::Hourly);
        let periods = split_by_period(&period_timeframe, hourly_volumes);

        for daily_period in periods {
            hourly.push((market_volumes.0.clone(), daily_period));
        }
    }

    // store
    let _ = dragonfly_driver
        .set_multiple_volumes(hourly, &period_timeframe)
        .await?;

    // TODO: complete write five minute to parquet file
    let _ = write_to_parquet().await?;

    // delete processed volumes
    let _ = dragonfly_driver
        .delete_outdated_volumes(
            period_timeframe.round_down_timestamp(&ts),
            &period_timeframe,
        )
        .await?;

    Ok(())
}

async fn write_to_parquet() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: complete write to parquet file
    todo!();
}

/*

  process volumetrics
  process snapshots



  for each
    get all the five minute snapshots for the previous (or older)
    aggregate data, creating hourly
    save hourly in redis, with the key = market_address_periodTimestamp (period timestamp = day end, midnight)
    format five minute snaps/volumes into dataframes for parquet
    read period parquet file, update with new additions, save (overwrite) parquet file, or create new daily file? (check size)
    delete five minute from redis

*/
