use chrono::Utc;

use crate::{
    dragonfly::{
        daily_aggregation::DailyAggregation, dragonfly_driver::ProtocolDragonflyDriver,
        volumetrics::VolumetricsTrait,
    },
    types::shared::Timeframe,
    utils::volumetrics::{aggregate_volumetrics::aggregate_volumetrics, split_by_period},
};

pub async fn daily_aggregation(
    dragonfly_driver: &mut ProtocolDragonflyDriver
) -> Result<(), Box<dyn std::error::Error>> {
    let ts = Utc::now().timestamp_micros() as u64;
    let period_timeframe = Timeframe::Daily;
    let previous_period_end = period_timeframe.round_down_timestamp(&ts);

    let _ = aggregate_daily_volumetrics(previous_period_end, dragonfly_driver, &period_timeframe)
        .await?;

    // set latest_update ts
    dragonfly_driver
        .set_latest_aggregation_ts(&previous_period_end)
        .await?;

    Ok(())
}

async fn aggregate_daily_volumetrics(
    previous_period_end: u64,
    dragonfly_driver: &mut ProtocolDragonflyDriver,
    period_timeframe: &Timeframe,
) -> Result<(), Box<dyn std::error::Error>> {
    // get five minute volumes
    let stored_hourly_volumes = dragonfly_driver
        .get_all_volumes(&Timeframe::Hourly, Some(previous_period_end))
        .await?;

    // aggregate into hourly and create daily period arrays of hourly
    let mut daily = vec![];

    for market_volumes in stored_hourly_volumes {
        let daily_volumes = aggregate_volumetrics(&market_volumes.1, &period_timeframe);
        let periods = split_by_period(&period_timeframe, daily_volumes);

        for daily_period in periods {
            daily.push((market_volumes.0.clone(), daily_period));
        }
    }

    // TODO: complete write hourly/daily to Long term storage
    let _ = write_to_storage().await?;

    // delete processed volumes
    let _ = dragonfly_driver
        .delete_outdated_volumes(previous_period_end, &period_timeframe)
        .await?;

    Ok(())
}

async fn write_to_storage() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: complete write to parquet file
    todo!();
}
