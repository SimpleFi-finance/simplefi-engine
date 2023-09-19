use polars::series::Series;

use super::date::round_timestamp;

pub fn match_timestamp_period(timestamp_series: &Series) -> (Vec<u64>, Vec<u64>, Vec<u64>) {
    let mut snap_timestamp_series: Vec<u64> = vec![];
    let mut hour_snap_timestamp_series: Vec<u64> = vec![];
    let mut day_snap_timestamp_series: Vec<u64> = vec![];

    let timestamps = timestamp_series.u64().unwrap().into_iter();

    for ts in timestamps {
        let t = ts.unwrap();
        snap_timestamp_series.push(round_timestamp(5, &t));
        hour_snap_timestamp_series.push(round_timestamp(60, &t));
        day_snap_timestamp_series.push(round_timestamp(60 * 24, &t));
    }

    (
        snap_timestamp_series,
        hour_snap_timestamp_series,
        day_snap_timestamp_series,
    )
}
