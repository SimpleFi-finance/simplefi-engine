use crate::types::{shared::Timeframe, volumetrics::Volumetric};
use crate::utils::volumetrics::amalgamate_volumetrics;
use mongo_types::Mongo;
use mongodb::bson::doc;
use simplefi_engine_settings::load_settings;

use super::{
    types::{VolumetricPeriodDaily, VolumetricPeriodFiveMin, VolumetricPeriodHourly},
    utils::{
        inserts::{prep_daily_insert, prep_five_min_insert, prep_hourly_insert},
        shared::{get_month_year_day, split_by_period},
    },
};

pub async fn insert_volumtrics_or_update_daily(
    db: &Mongo,
    address: &str,
    volumetrics: Vec<Volumetric>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (month, year, _) = get_month_year_day(volumetrics[0].timestamp);

    let global_settings = load_settings().unwrap();

    let collection = db.collection::<VolumetricPeriodDaily>(
        &global_settings.volumetrics_daily_gold_collection_name,
    );

    // let collection = db.collection::<VolumetricPeriodDaily>("volumetrics_daily");
    let grouped_volumetric = collection
        .find_one(
            doc! {"address": address, "year": year, "month": month},
            None,
        )
        .await?;
    let mut volumes_to_insert = volumetrics;

    match grouped_volumetric {
        Some(res) => {
            // check if need to update last entry
            let last_volume = &res.mapping[&res.mapping.len() - 1];
            let is_same_block =
                last_volume.volume.block_number == volumes_to_insert[0].block_number;

            if is_same_block {
                // update new entry to add stats from db entry
                volumes_to_insert[0] =
                    amalgamate_volumetrics(&last_volume.volume, &volumes_to_insert[0]);
            }

            let filtered_previous_entries = res
                .mapping
                .iter()
                .filter(|v| v.volume.block_number < volumes_to_insert[0].block_number)
                .map(|v| v.volume.clone())
                .collect();

            let periods_to_insert = split_by_period(
                [filtered_previous_entries, volumes_to_insert].concat(),
                &Timeframe::Daily,
            );

            let inserts = periods_to_insert
                .iter()
                .cloned()
                .map(|volumes| prep_daily_insert(volumes, address))
                .collect::<Vec<VolumetricPeriodDaily>>();

            // delete old entry
            // TODO make this more efficient
            let _ = collection
                .delete_one(
                    doc! {"address": address, "year": year, "month": month},
                    None,
                )
                .await;

            // insert new entries
            let _ = collection.insert_many(inserts, None).await;

            Ok(())
        }
        _ => {
            // no previous entry in this period, simple insert
            let entry_periods = split_by_period(volumes_to_insert, &Timeframe::Daily);

            let inserts = entry_periods
                .iter()
                .cloned()
                .map(|volumes| prep_daily_insert(volumes, address))
                .collect::<Vec<VolumetricPeriodDaily>>();
            let _ = collection.insert_many(inserts, None).await;

            Ok(())
        }
    }
}

pub async fn insert_volumtrics_or_update_hourly(
    db: &Mongo,
    address: &str,
    volumetrics: Vec<Volumetric>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (month, year, _) = get_month_year_day(volumetrics[0].timestamp);

    let global_settings = load_settings().unwrap();

    let collection = db.collection::<VolumetricPeriodHourly>(
        &global_settings.volumetrics_hourly_gold_collection_name,
    );

    // let collection = db.collection::<VolumetricPeriodHourly>("volumetrics_hourly");
    let grouped_volumetric = collection
        .find_one(
            doc! {"address": address, "year": year, "month": month},
            None,
        )
        .await?;
    let mut volumes_to_insert = volumetrics;

    match grouped_volumetric {
        Some(res) => {
            // check if need to update last entry
            let last_volume_mapping = &res.mapping[&res.mapping.len() - 1];
            let is_same_block = last_volume_mapping.mapping[last_volume_mapping.mapping.len() - 1]
                .block_number
                == volumes_to_insert[0].block_number;

            if is_same_block {
                // update new entry to add stats from db entry
                volumes_to_insert[0] = amalgamate_volumetrics(
                    &last_volume_mapping.mapping[last_volume_mapping.mapping.len() - 1],
                    &volumes_to_insert[0],
                );
            }

            let filtered_previous_entries = res
                .mapping
                .iter()
                .filter(|v| {
                    let last_item = &v.mapping[&v.mapping.len() - 1];
                    last_item.block_number < volumes_to_insert[0].block_number
                })
                .map(|v| v.mapping.clone())
                .flatten()
                .collect();

            let periods_to_insert = split_by_period(
                [filtered_previous_entries, volumes_to_insert].concat(),
                &Timeframe::Hourly,
            );

            let inserts = periods_to_insert
                .iter()
                .cloned()
                .map(|volumes| prep_hourly_insert(volumes, address))
                .collect::<Vec<VolumetricPeriodHourly>>();

            // delete old entry
            // TODO make this more efficient
            let _ = collection
                .delete_one(
                    doc! {"address": address, "year": year, "month": month},
                    None,
                )
                .await;

            // insert new entries
            let _ = collection.insert_many(inserts, None).await;

            Ok(())
        }
        _ => {
            // no previous entry in this period, simple insert
            let entry_periods = split_by_period(volumes_to_insert, &Timeframe::Hourly);

            let inserts = entry_periods
                .iter()
                .cloned()
                .map(|volumes| prep_hourly_insert(volumes, address))
                .collect::<Vec<VolumetricPeriodHourly>>();
            let _ = collection.insert_many(inserts, None).await;

            Ok(())
        }
    }
}
pub async fn insert_volumtrics_or_update_five(
    db: &Mongo,
    address: &str,
    volumetrics: Vec<Volumetric>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (month, year, day) = get_month_year_day(volumetrics[0].timestamp);

    let global_settings = load_settings().unwrap();

    let collection = db.collection::<VolumetricPeriodFiveMin>(
        &global_settings.volumetrics_five_minute_gold_collection_name,
    );

    // let collection = db.collection::<VolumetricPeriodFiveMin>("volumetrics_five");
    let grouped_volumetric = collection
        .find_one(
            doc! {"address": address, "year": year, "month": month, "day": day},
            None,
        )
        .await?;
    let mut volumes_to_insert = volumetrics;

    match grouped_volumetric {
        Some(res) => {
            let last_volume = &res.mapping[&res.mapping.len() - 1].volume;
            // check if need to update last entry
            let is_same_block = last_volume.block_number == volumes_to_insert[0].block_number;

            if is_same_block {
                // update new entry to add stats from db entry
                volumes_to_insert[0] = amalgamate_volumetrics(&last_volume, &volumes_to_insert[0]);
            }

            let filtered_previous_entries = res
                .mapping
                .iter()
                .filter(|v| v.volume.block_number < volumes_to_insert[0].block_number)
                .map(|v| v.volume.clone())
                .collect();

            let periods_to_insert = split_by_period(
                [filtered_previous_entries, volumes_to_insert].concat(),
                &Timeframe::FiveMinute,
            );

            let inserts = periods_to_insert
                .iter()
                .cloned()
                .map(|volumes| prep_five_min_insert(volumes, address))
                .collect::<Vec<VolumetricPeriodFiveMin>>();

            // delete old entry
            // TODO make this more efficient
            let _ = collection
                .delete_one(
                    doc! {"address": address, "year": year, "month": month},
                    None,
                )
                .await;

            // insert new entries
            let _ = collection.insert_many(inserts, None).await;

            Ok(())
        }
        _ => {
            // no previous entry in this period, simple insert
            let entry_periods = split_by_period(volumes_to_insert, &Timeframe::FiveMinute);

            let inserts = entry_periods
                .iter()
                .cloned()
                .map(|volumes| prep_five_min_insert(volumes, address))
                .collect::<Vec<VolumetricPeriodFiveMin>>();
            let _ = collection.insert_many(inserts, None).await;

            Ok(())
        }
    }
}
