use crate::types::{shared::Timeframe, volumetrics::Volumetric};
use chrono::{Datelike, TimeZone, Utc};

use crate::mongo::volumetrics::types::{
    DailyMappingItem, FiveMinMappingItem, HourlyMappingItem, VolumetricPeriodDaily,
    VolumetricPeriodFiveMin, VolumetricPeriodHourly,
};

use super::shared::split_by_period;

pub fn prep_daily_insert(
    volumes: Vec<Volumetric>,
    address: &str,
) -> VolumetricPeriodDaily {
    let ts = volumes[0].timestamp * 1000;
    let utc = Utc.timestamp_millis_opt(ts as i64).unwrap();
    let year = utc.year();
    let month = utc.month();

    let mapping = volumes
        .iter()
        .map(|v| {
            let utcv = Utc
                .timestamp_millis_opt((v.timestamp * 1000) as i64)
                .unwrap();
            let day = utcv.day();
            DailyMappingItem {
                day,
                volume: v.clone(),
                latest: false,
            }
        })
        .collect();

    VolumetricPeriodDaily {
        year: year as u32,
        month: month as u32,
        address: address.to_string(),
        mapping,
    }
}

pub fn prep_hourly_insert(
    volumes: Vec<Volumetric>,
    address: &str,
) -> VolumetricPeriodHourly {
    let ts = volumes[0].timestamp * 1000;
    let utc = Utc.timestamp_millis_opt(ts as i64).unwrap();
    let year = utc.year();
    let month = utc.month();

    // split into daily vecs
    let daily_periods = split_by_period(volumes, &Timeframe::FiveMinute);

    let mapping = daily_periods
        .iter()
        .map(|v| {
            let utcv = Utc
                .timestamp_millis_opt((v[0].timestamp * 1000) as i64)
                .unwrap();
            let day = utcv.day();
            HourlyMappingItem {
                day,
                mapping: v.clone(),
                latest: false,
            }
        })
        .collect();

    VolumetricPeriodHourly {
        year: year as u32,
        month: month as u32,
        address: address.to_string(),
        mapping,
    }
}
pub fn prep_five_min_insert(
    volumes: Vec<Volumetric>,
    address: &str,
) -> VolumetricPeriodFiveMin {
    let ts = volumes[0].timestamp * 1000;
    let utc = Utc.timestamp_millis_opt(ts as i64).unwrap();
    let year = utc.year();
    let month = utc.month();
    let day = utc.day();

    // split into daily vecs
    let mapping = volumes
        .iter()
        .map(|v| FiveMinMappingItem {
            volume: v.clone(),
            latest: false,
        })
        .collect();

    VolumetricPeriodFiveMin {
        year: year as u32,
        month: month as u32,
        day,
        address: address.to_string(),
        mapping,
    }
}
