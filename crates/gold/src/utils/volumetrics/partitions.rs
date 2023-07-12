use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

use polars::prelude::DataFrame;

use crate::{
    protocol_driver::{
        driver_traits::volumetric_methods::VolumetricMethods,
        protocol_driver::SupportedProtocolDrivers,
    },
    types::volumetrics::Volumetric,
    utils::{balance_strings::format_balance_string, big_number::add_big_from_strs},
};

pub fn df_parallel_partitions(
    dfs: Arc<Vec<DataFrame>>,
    driver: &SupportedProtocolDrivers,
) -> Vec<Volumetric> {
    match dfs.len() {
        x if x <= 50 => df_partitions_threaded(dfs, 1, driver),
        x if x < 100 => df_partitions_threaded(dfs, 2, driver),
        x if x < 150 => df_partitions_threaded(dfs, 3, driver),
        x if x < 250 => df_partitions_threaded(dfs, 4, driver),
        x if x < 500 => df_partitions_threaded(dfs, 5, driver),
        x if x < 1000 => df_partitions_threaded(dfs, 6, driver),
        x if x < 2500 => df_partitions_threaded(dfs, 7, driver),
        x if x < 5000 => df_partitions_threaded(dfs, 8, driver),
        x if x < 7500 => df_partitions_threaded(dfs, 10, driver),
        x if x < 10000 => df_partitions_threaded(dfs, 12, driver),
        _ => df_partitions_threaded(dfs, 15, driver),
    }
}

pub fn df_partitions_threaded(
    dfs: Arc<Vec<DataFrame>>,
    threads: usize,
    driver: &SupportedProtocolDrivers,
) -> Vec<Volumetric> {
    let chunk_size = dfs.len() / threads;

    let mut active_threads: Vec<JoinHandle<Vec<Volumetric>>> = vec![];

    for count in 0..threads {
        let thread_chunk_start = chunk_size * count;
        let thread_chunk_end = thread_chunk_start + chunk_size;

        let array_slice = dfs[thread_chunk_start..thread_chunk_end].to_vec();
        // let new_thread = thread::spawn(move || {
        //     array_slice
        //         .iter()
        //         .map(|df| driver.volumes_from_dataframe_slice(df))
        //         .collect::<Vec<Volumetric>>()
        // });
        let new_driver = driver.clone();
        let new_thread = thread::spawn(move || volumetric_from_df_slice(array_slice, new_driver));
        active_threads.push(new_thread);
    }

    let mut volumetrics: Vec<Vec<Volumetric>> = vec![];

    for thread in active_threads {
        let res = thread.join().unwrap();
        volumetrics.push(res);
    }

    volumetrics.concat()
}

fn volumetric_from_df_slice(
    dfs: Vec<DataFrame>,
    driver: SupportedProtocolDrivers,
) -> Vec<Volumetric> {
    let y: Vec<Volumetric> = dfs
        .iter()
        .map(|df| driver.volumes_from_dataframe_slice(df))
        .collect();
    y
}
