pub mod create_five_min_volumetrics;
pub mod partitions;
use super::balance_strings::merge_bal_string_vecs;
use super::big_number::add_big_from_strs;
use crate::types::shared::Timeframe;
use crate::types::volumetrics::Volumetric;
use std::collections::HashMap;
pub mod aggregate_volumetrics;
pub mod process_volumetrics;

pub fn amalgamate_volumetrics(
    volume_a: &Volumetric,
    volume_b: &Volumetric,
) -> Volumetric {
    Volumetric {
        timestamp: volume_a.timestamp,
        swaps_out: merge_bal_string_vecs(&volume_a.swaps_out, &volume_b.swaps_out),
        swaps_in: merge_bal_string_vecs(&volume_a.swaps_in, &volume_b.swaps_in),
        withdrawal: merge_bal_string_vecs(&volume_a.withdrawal, &volume_b.withdrawal),
        mint: merge_bal_string_vecs(&volume_a.mint, &volume_b.mint),
        transfer: add_big_from_strs(&volume_a.transfer, &volume_b.transfer),
    }
}

pub fn amalgamate_volumetrics_vecs(
    volumes_a: Vec<Volumetric>,
    volumes_b: Vec<Volumetric>,
) -> Vec<Volumetric> {
    let mut hashmap: HashMap<u64, Volumetric> = HashMap::new();

    volumes_a.iter().for_each(|v| {
        hashmap.insert(v.timestamp, v.clone());
    });

    volumes_b.iter().for_each(|v| {
        let existing = hashmap.get(&v.timestamp);

        match existing {
            Some(e) => {
                hashmap.insert(e.timestamp, amalgamate_volumetrics(v, e));
            }
            None => {
                hashmap.insert(v.timestamp, v.clone());
            }
        }
    });

    hashmap.values().cloned().collect()
}

pub fn split_by_period(
    timeframe: &Timeframe,
    volumes: Vec<Volumetric>,
) -> Vec<Vec<Volumetric>> {
    let mut hmap: HashMap<u64, Vec<Volumetric>> = HashMap::new();

    for volume in volumes {
        // let existing = hmap.get()

        let mut existing = hmap.get(&volume.timestamp);

        match existing {
            Some(ex) => {
                ex.push(volume);
            }
            _ => {
                hmap.insert(volume.timestamp, vec![volume]);
            }
        }
    }

    hmap.values().cloned().collect()
}
