use std::sync::Arc;

use polars::{
    lazy::dsl::{col, lit},
    prelude::{IntoLazy, LazyFrame, NamedFrom},
    series::Series,
};

use crate::{
    protocol_driver::protocol_driver::SupportedProtocolDrivers,
    types::volumetrics::Volumetric,
    utils::{
        period_timestamps::match_timestamp_period, volumetrics::partitions::df_parallel_partitions,
    },
};

pub async fn create_five_min_volumetrics(
    events: LazyFrame,
    driver: &SupportedProtocolDrivers,
) -> Vec<Volumetric> {
    let filtered_events = events
        .select([
            col("timestamp"),
            col("log_type"),
            col("topic1"),
            col("topic2"),
            col("topic3"),
            col("data1"),
            col("data2"),
            col("data3"),
            col("data4"),
        ])
        .filter(col("log_type").eq(lit("Sync")).not())
        .filter(col("log_type").eq(lit("Approval")).not())
        .sort("timestamp", Default::default())
        .collect()
        .unwrap();

    let (snapshot_timestamp_series, _, _) = match_timestamp_period(
        filtered_events
            .column("timestamp")
            .expect("expect to get timestamp series"),
    );

    let with_snapshot = filtered_events
        .lazy()
        .with_columns(vec![lit(Series::new(
            "snapshot_timestamp",
            snapshot_timestamp_series,
        ))])
        .collect()
        .unwrap();

    let partitioned_dfs = with_snapshot
        .partition_by_stable(["snapshot_timestamp"])
        .unwrap();

    df_parallel_partitions(Arc::new(partitioned_dfs), driver)
}
