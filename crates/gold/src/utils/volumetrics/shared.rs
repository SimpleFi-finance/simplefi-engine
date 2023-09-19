use crate::types::{shared::Timeframe, volumetrics::Volumetric};
use crate::utils::date::{is_same_day, is_same_month};
use chrono::{Datelike, TimeZone, Timelike, Utc};

pub fn is_same_period(
    timestamp_a: u64,
    timestamp_b: u64,
    timeframe: &Timeframe,
) -> bool {
    match timeframe {
        Timeframe::FiveMinute => is_same_day(timestamp_a, timestamp_b),
        _ => is_same_month(timestamp_a, timestamp_b),
    }
}

pub fn get_month_year_day_hour_minute(ts: &u64) -> (u32, u32, u32, u32, u32) {
    let utc = Utc.timestamp_millis_opt(ts.clone() as i64).unwrap();
    let year = utc.year() as u32;
    let month = utc.month();
    let day = utc.day();
    let hour = utc.hour();
    let minute = utc.minute();

    (month, year, day, hour, minute)
}

pub fn split_by_period(
    volumes: Vec<Volumetric>,
    timeframe: &Timeframe,
) -> Vec<Vec<Volumetric>> {
    volumes.iter().cloned().fold(vec![], |mut acc, volume| {
        if acc.len() != 0 {
            let acc_len = acc.len();
            let most_recent_period: &mut Vec<Volumetric> = &mut acc[acc_len - 1];
            let most_recent_volume = &most_recent_period[most_recent_period.len() - 1];

            if is_same_period(
                volume.timestamp * 1000,
                most_recent_volume.timestamp * 1000,
                timeframe,
            ) {
                most_recent_period.push(volume);

                return acc;
            }
        }
        // first entry or new period
        acc.push(vec![volume]);

        return acc;
    })
}
