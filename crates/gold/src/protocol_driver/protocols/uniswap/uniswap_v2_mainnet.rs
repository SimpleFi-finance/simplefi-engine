use crate::{
    types::{protocols::ProtocolInfo, volumetrics::Volumes},
    utils::{balance_strings::format_balance_string, big_number::add_big_from_strs},
};
use bronze::mongo::evm::data_sets::logs::Log;
use chains_types::SupportedChains;
use polars::{
    prelude::{DataFrame, NamedFrom},
    series::Series,
};

// protocol info
pub fn get_protocol_info() -> ProtocolInfo {
    ProtocolInfo {
        name: String::from("UniswapV2_mainnet"),
        factory_address: String::from("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f"),
        chain: SupportedChains::EthereumMainnet,
        creation_log_name: String::from("PairCreated"),
        protocol_start_year: 2020,
    }
}

// volumetric methods
pub fn volumes_from_dataframe_slice(df: DataFrame) -> Volumes {
    let mut token_0_out_total = "0".to_string();
    let mut token_1_out_total = "0".to_string();
    let mut token_0_in_total = "0".to_string();
    let mut token_1_in_total = "0".to_string();
    let mut transfer_total = "0".to_string();
    let mut token_0_withdraw_total = "0".to_string();
    let mut token_1_withdraw_total = "0".to_string();
    let mut token_0_deposit_total = "0".to_string();
    let mut token_1_deposit_total = "0".to_string();

    for _index in 0..df.height() {
        let series = df
            .columns([
                "data1",
                "data2",
                "data3",
                "data4",
                "snapshot_timestamp",
                "snapshot_block",
                "hour_snapshot_timestamp",
                "hour_snapshot_block",
                "day_snapshot_timestamp",
                "day_snapshot_block",
                "log_type",
                "topic1",
                "topic2",
            ])
            .unwrap();
        let mut log_type_series = series[10].utf8().unwrap().into_iter();
        let mut topic_1_series = series[11].utf8().unwrap().into_iter();
        let mut topic_2_series = series[12].utf8().unwrap().into_iter();
        let mut data_1_series = series[0].utf8().unwrap().into_iter();
        let mut data_2_series = series[1].utf8().unwrap().into_iter();
        let mut data_3_series = series[2].utf8().unwrap().into_iter();
        let mut data_4_series = series[3].utf8().unwrap().into_iter();
        let event_type = log_type_series.next().unwrap().unwrap();
        let data_1 = data_1_series.next().unwrap();
        let data_2 = data_2_series.next().unwrap();
        let data_3 = data_3_series.next().unwrap();
        let data_4 = data_4_series.next().unwrap();
        let topic_1 = topic_1_series.next().unwrap();
        let topic_2 = topic_2_series.next().unwrap();
        match event_type {
            "Swap" => {
                match data_1 {
                    Some(x) => token_0_out_total = add_big_from_strs(&token_0_out_total, x),
                    _ => (),
                }
                match data_2 {
                    Some(x) => token_1_out_total = add_big_from_strs(&token_1_out_total, x),
                    _ => (),
                }
                match data_3 {
                    Some(x) => token_0_in_total = add_big_from_strs(&token_0_in_total, x),
                    _ => (),
                }
                match data_4 {
                    Some(x) => token_1_in_total = add_big_from_strs(&token_1_in_total, x),
                    _ => (),
                }
            }
            "Transfer" => match (topic_1, topic_2, data_1) {
                (Some(from_value), Some(to_value), Some(value)) => {
                    if from_value != "0x0000000000000000000000000000000000000000"
                        && to_value != "0x0000000000000000000000000000000000000000"
                    {
                        transfer_total = add_big_from_strs(value, &transfer_total)
                    }
                }
                _ => (),
            },
            "Burn" => match (data_1, data_2) {
                (Some(token_0_value), Some(token_1_value)) => {
                    token_0_withdraw_total =
                        add_big_from_strs(token_0_value, &token_0_withdraw_total);
                    token_1_withdraw_total =
                        add_big_from_strs(token_1_value, &token_1_withdraw_total);
                }
                _ => (),
            },
            "Mint" => match (data_1, data_2) {
                (Some(token_0_value), Some(token_1_value)) => {
                    token_0_deposit_total =
                        add_big_from_strs(token_0_value, &token_0_deposit_total);
                    token_1_deposit_total =
                        add_big_from_strs(token_1_value, &token_0_deposit_total);
                }
                _ => (),
            },
            _ => (),
        }
    }
    Volumes {
        swaps_in: vec![
            format!("token0|{}", token_0_in_total),
            format!("token1|{}", token_1_in_total),
        ],
        swaps_out: vec![
            format!("token0|{}", token_0_out_total),
            format!("token1|{}", token_1_out_total),
        ],
        withdrawal: vec![
            format_balance_string("token0", &token_0_withdraw_total),
            format_balance_string("token1", &token_1_withdraw_total),
        ],
        mint: vec![
            format_balance_string("token0", &token_0_deposit_total),
            format_balance_string("token1", &token_1_deposit_total),
        ],
        transfer: transfer_total,
    }
}

// market creation
pub fn get_created_market_addresses(df: DataFrame) -> Vec<String> {
    todo!();
}

pub fn get_created_market_address(log: Log) -> String {
    todo!();
}

// snapshot methods

// log normalization
// TODO Complete log normalization once I've seen F's types for it
fn normalise_transfer_log(
    df_builder: &mut DataFrameSeries,
    log: Log,
) {
    // todo!()
}
fn normalise_creation_log(
    df_builder: &mut DataFrameSeries,
    log: Log,
) {
    // todo!()
}
fn normalise_swap_log(
    df_builder: &mut DataFrameSeries,
    log: Log,
) {
    // todo!()
}
fn normalise_mint_log(
    df_builder: &mut DataFrameSeries,
    log: Log,
) {
    // todo!()
}
fn normalise_burn_log(
    df_builder: &mut DataFrameSeries,
    log: Log,
) {
    // todo!()
}

struct DataFrameSeries {
    timestamp_series: Vec<Option<i64>>,
    year_series: Vec<Option<i64>>,
    month_series: Vec<Option<i64>>,
    day_series: Vec<Option<i64>>,
    address_series: Vec<Option<String>>,
    block_number_series: Vec<Option<i64>>,
    block_hash_series: Vec<Option<String>>,
    transaction_hash_series: Vec<Option<String>>,
    transaction_index_series: Vec<Option<i64>>,
    log_index_series: Vec<Option<i64>>,
    log_type_series: Vec<Option<String>>,
    topic1_series: Vec<Option<String>>,
    topic2_series: Vec<Option<String>>,
    topic3_series: Vec<Option<String>>,
    topic4_series: Vec<Option<String>>,
    data1_series: Vec<Option<String>>,
    data2_series: Vec<Option<String>>,
    data3_series: Vec<Option<String>>,
    data4_series: Vec<Option<String>>,
    data5_series: Vec<Option<String>>,
    data6_series: Vec<Option<String>>,
    data7_series: Vec<Option<String>>,
    data8_series: Vec<Option<String>>,
    data9_series: Vec<Option<String>>,
    data10_series: Vec<Option<String>>,
    removed_series: Vec<Option<bool>>,
    tx_log_index_series: Vec<Option<i64>>,
}

pub fn normalize_logs(logs: Vec<Log>) -> DataFrame {
    let creation_log = "PairCreated".to_string();
    let transfer_log = "Transfer".to_string();
    let swap_log = "Swap".to_string();
    let mint_log = "Mint".to_string();
    let burn_log = "Burn".to_string();

    let mut df_builder = DataFrameSeries {
        timestamp_series: vec![],
        year_series: vec![],
        month_series: vec![],
        day_series: vec![],
        address_series: vec![],
        block_number_series: vec![],
        block_hash_series: vec![],
        transaction_hash_series: vec![],
        transaction_index_series: vec![],
        log_index_series: vec![],
        log_type_series: vec![],
        topic1_series: vec![],
        topic2_series: vec![],
        topic3_series: vec![],
        topic4_series: vec![],
        data1_series: vec![],
        data2_series: vec![],
        data3_series: vec![],
        data4_series: vec![],
        data5_series: vec![],
        data6_series: vec![],
        data7_series: vec![],
        data8_series: vec![],
        data9_series: vec![],
        data10_series: vec![],
        removed_series: vec![],
        tx_log_index_series: vec![],
    };

    for log in logs {
        let cloned_log = log.clone();

        df_builder.timestamp_series.push(Some(log.timestamp));
        df_builder.year_series.push(Some(log.year as i64));
        df_builder.month_series.push(Some(log.month as i64));
        df_builder.day_series.push(Some(log.day as i64));
        df_builder.address_series.push(log.address);
        df_builder.block_hash_series.push(Some(log.block_hash));
        df_builder.block_number_series.push(Some(log.block_number));
        df_builder
            .transaction_hash_series
            .push(log.transaction_hash);
        df_builder
            .transaction_index_series
            .push(Some(log.transaction_index));
        df_builder
            .transaction_index_series
            .push(Some(log.transaction_index));
        df_builder
            .transaction_index_series
            .push(Some(log.transaction_index));
        df_builder.log_index_series.push(Some(log.log_index));
        df_builder.log_type_series.push(log.log_type.clone());
        df_builder.removed_series.push(Some(log.removed));
        df_builder
            .tx_log_index_series
            .push(Some(log.transaction_log_index));

        match log.log_type {
            Some(l) if l == creation_log => normalise_creation_log(&mut df_builder, cloned_log),
            Some(l) if l == transfer_log => normalise_transfer_log(&mut df_builder, cloned_log),
            Some(l) if l == swap_log => normalise_swap_log(&mut df_builder, cloned_log),
            Some(l) if l == mint_log => normalise_mint_log(&mut df_builder, cloned_log),
            Some(l) if l == burn_log => normalise_burn_log(&mut df_builder, cloned_log),

            _ => continue,
        };
    }

    DataFrame::new(vec![
        Series::new("timestamp", df_builder.timestamp_series),
        Series::new("year", df_builder.year_series),
        Series::new("month", df_builder.month_series),
        Series::new("day", df_builder.day_series),
        Series::new("address", df_builder.address_series),
        Series::new("block_number", df_builder.block_number_series),
        Series::new("block_hash", df_builder.block_hash_series),
        Series::new("transaction_hash", df_builder.transaction_hash_series),
        Series::new("transaction_index", df_builder.transaction_index_series),
        Series::new("log_index", df_builder.log_index_series),
        Series::new("log_type", df_builder.log_type_series),
        Series::new("topic1", df_builder.topic1_series),
        Series::new("topic2", df_builder.topic2_series),
        Series::new("topic3", df_builder.topic3_series),
        Series::new("topic4", df_builder.topic4_series),
        Series::new("data1", df_builder.data1_series),
        Series::new("data2", df_builder.data2_series),
        Series::new("data3", df_builder.data3_series),
        Series::new("data4", df_builder.data4_series),
        Series::new("data5", df_builder.data5_series),
        Series::new("data6", df_builder.data6_series),
        Series::new("data7", df_builder.data7_series),
        Series::new("data8", df_builder.data8_series),
        Series::new("data9", df_builder.data9_series),
        Series::new("data10", df_builder.data10_series),
        Series::new("removed", df_builder.removed_series),
        Series::new("tx_log_index", df_builder.tx_log_index_series),
    ])
    .unwrap()
}
