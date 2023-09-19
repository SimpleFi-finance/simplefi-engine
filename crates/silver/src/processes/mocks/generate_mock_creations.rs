use polars::{
    prelude::{DataFrame, NamedFrom},
    series::Series,
};

use crate::mongo::volumetrics::utils::shared::get_month_year_day_hour_minute;

pub async fn generate_mock_dataframe(
    // factory_address: &String,
    // year: i32,
    from: i64,
    to: i64,
) -> DataFrame {
    let timestamps: Vec<i64> = vec![
        1639601385625,
        1641415785625,
        1643230185625,
        1645044585625,
        1646858985625,
        1648673385625,
        1650487785625,
        1652302185625,
        1654116585625,
        1655930985625,
        1657745385625,
        1659559785625,
        1661374185625,
        1663188585625,
        1665002985625,
        1666817385625,
        1668631785625,
        1670446185625,
        1672260585625,
        1674074985625,
        1675889385625,
        1677703785625,
        1679518185625,
        1681332585625,
        1683146985625,
    ];
    let mut timestamp_series: Vec<Option<i64>> = vec![];
    let mut year_series: Vec<Option<i64>> = vec![];
    let mut month_series: Vec<Option<i64>> = vec![];
    let mut day_series: Vec<Option<i64>> = vec![];
    let mut address_series: Vec<String> = vec![];
    let mut block_number_series: Vec<Option<i64>> = vec![];
    let mut block_hash_series: Vec<Option<String>> = vec![];
    let mut transaction_hash_series: Vec<Option<String>> = vec![];
    let mut transaction_index_series: Vec<Option<i64>> = vec![];
    let mut log_index_series: Vec<Option<i64>> = vec![];
    let mut log_type_series: Vec<Option<String>> = vec![];
    let mut topic1_series: Vec<Option<String>> = vec![];
    let mut topic2_series: Vec<Option<String>> = vec![];
    let mut topic3_series: Vec<Option<String>> = vec![];
    let mut topic4_series: Vec<Option<String>> = vec![];
    let mut data1_series: Vec<Option<String>> = vec![];
    let mut data2_series: Vec<Option<String>> = vec![];
    let mut data3_series: Vec<Option<String>> = vec![];
    let mut data4_series: Vec<Option<String>> = vec![];
    let mut data5_series: Vec<Option<String>> = vec![];
    let mut data6_series: Vec<Option<String>> = vec![];
    let mut data7_series: Vec<Option<String>> = vec![];
    let mut data8_series: Vec<Option<String>> = vec![];
    let mut data9_series: Vec<Option<String>> = vec![];
    let mut data10_series: Vec<Option<String>> = vec![];
    let mut removed_series: Vec<Option<bool>> = vec![];
    let mut tx_log_index_series: Vec<Option<i64>> = vec![];

    for i in 0..24 {
        let ts = timestamps[i] as u64;
        let (month, year, day, _, _) = get_month_year_day_hour_minute(&ts);
        timestamp_series.push(Some(timestamps[i]));
        year_series.push(Some(year as i64));
        month_series.push(Some(month as i64));
        day_series.push(Some(day as i64));
        address_series.push("test".to_string());
        block_number_series.push(Some(i as i64));
        block_hash_series.push(Some("asdas".to_string()));
        transaction_hash_series.push(Some("asdas".to_string()));
        transaction_index_series.push(Some(1));
        log_index_series.push(Some(i as i64));
        log_type_series.push(Some("PairCreated".to_string()));
        topic1_series.push(Some("token_1".to_string()));
        topic2_series.push(Some("token_2".to_string()));
        topic3_series.push(None);
        topic4_series.push(None);
        data1_series.push(Some(format!("market{}", i)));
        data2_series.push(None);
        data3_series.push(None);
        data4_series.push(None);
        data5_series.push(None);
        data6_series.push(None);
        data7_series.push(None);
        data8_series.push(None);
        data9_series.push(None);
        data10_series.push(None);
        removed_series.push(None);
        tx_log_index_series.push(None);
    }

    DataFrame::new(vec![
        Series::new("timestamp", timestamp_series),
        Series::new("year", year_series),
        Series::new("month", month_series),
        Series::new("day", day_series),
        Series::new("address", address_series),
        Series::new("block_number", block_number_series),
        Series::new("block_hash", block_hash_series),
        Series::new("transaction_hash", transaction_hash_series),
        Series::new("transaction_index", transaction_index_series),
        Series::new("log_index", log_index_series),
        Series::new("log_type", log_type_series),
        Series::new("topic1", topic1_series),
        Series::new("topic2", topic2_series),
        Series::new("topic3", topic3_series),
        Series::new("topic4", topic4_series),
        Series::new("data1", data1_series),
        Series::new("data2", data2_series),
        Series::new("data3", data3_series),
        Series::new("data4", data4_series),
        Series::new("data5", data5_series),
        Series::new("data6", data6_series),
        Series::new("data7", data7_series),
        Series::new("data8", data8_series),
        Series::new("data9", data9_series),
        Series::new("data10", data10_series),
        Series::new("removed", removed_series),
        Series::new("tx_log_index", tx_log_index_series),
    ])
    .unwrap()
}
