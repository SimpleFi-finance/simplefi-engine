use chrono::{DateTime, Datelike, NaiveDateTime, TimeZone, Timelike, Utc};

pub fn is_same_day(
    timestamp_x: u64,
    timestamp_y: u64,
) -> bool {
    let x_date = Utc.timestamp_millis_opt(timestamp_x as i64).unwrap();

    let x_year = x_date.year();
    let x_month = x_date.month();
    let x_day = x_date.day();

    let y_date = Utc.timestamp_millis_opt(timestamp_y as i64).unwrap();

    let y_year = y_date.year();
    let y_month = y_date.month();
    let y_day = y_date.day();

    return y_year == x_year && y_month == x_month && y_day == x_day;
}

pub fn is_same_month(
    timestamp_x: u64,
    timestamp_y: u64,
) -> bool {
    let x_date = Utc.timestamp_millis_opt(timestamp_x as i64).unwrap();
    let y_date = Utc.timestamp_millis_opt(timestamp_y as i64).unwrap();

    let x_year = x_date.year();
    let x_month = x_date.month();

    let y_year = y_date.year();
    let y_month = y_date.month();

    return y_year == x_year && y_month == x_month;
}

pub fn is_same_week(
    timestamp_x: u64,
    timestamp_y: u64,
) -> bool {
    let x_date = Utc.timestamp_millis_opt(timestamp_x as i64).unwrap();
    let y_date = Utc.timestamp_millis_opt(timestamp_y as i64).unwrap();

    let x_year = x_date.year();
    let x_month = x_date.month();
    let x_week = x_date.iso_week().week();

    let y_year = y_date.year();
    let y_month = y_date.month();
    let y_week = y_date.iso_week().week();

    return y_year == x_year && y_month == x_month && x_week == y_week;
}

pub fn is_same_hour(
    timestamp_x: u64,
    timestamp_y: u64,
) -> bool {
    let x_date = Utc.timestamp_millis_opt(timestamp_x as i64).unwrap();

    let x_year = x_date.year();
    let x_month = x_date.month();
    let x_day = x_date.day();
    let x_hour = x_date.hour();

    let y_date = Utc.timestamp_millis_opt(timestamp_y as i64).unwrap();

    let y_year = y_date.year();
    let y_month = y_date.month();
    let y_day = y_date.day();
    let y_hour = y_date.hour();

    return y_year == x_year && y_month == x_month && y_day == x_day && y_hour == x_hour;
}

pub fn is_hour_timestamp(timestamp: u64) -> bool {
    let date = Utc.timestamp_millis_opt(timestamp as i64).unwrap();
    let min = date.minute();
    let sec = date.second();
    min == 0 && sec == 0
}
pub fn is_day_timestamp(timestamp: u64) -> bool {
    let date = Utc.timestamp_millis_opt(timestamp as i64).unwrap();
    let hour = date.hour();
    let min = date.minute();
    min == 0 && hour == 0
}

pub fn round_timestamp(
    nearest_minutes: u64,
    ts: &u64,
) -> u64 {
    let round_to = 1000 * 60 * nearest_minutes;
    let ceiled_div = ts / round_to + (ts & round_to != 0) as u64;
    ceiled_div * round_to
}

pub fn datetime_from_ts(ts: u64) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp_millis(ts as i64).unwrap();
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    datetime
}
