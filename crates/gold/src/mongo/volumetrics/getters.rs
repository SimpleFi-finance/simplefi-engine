use crate::mongo::Mongo;
use mongodb::bson::doc;
use settings::load_settings;
use shared_types::gold::{shared::Timeframe, volumetrics::Volumetric};

// TODO

pub async fn get_latest_volumetric(
    db: &Mongo,
    address: &str,
    timeframe: Timeframe,
) -> Result<Volumetric, Box<dyn std::error::Error>> {
    let global_settings = load_settings().unwrap();

    match timeframe {
        Timeframe::Daily => {
            let global_settings = load_settings().unwrap();

            let collection = db.collection::<VolumetricPeriodDaily>(
                &global_settings.volumetrics_daily_gold_collection_name,
            );
        }
        Timeframe::Hourly => {
            let global_settings = load_settings().unwrap();

            let collection = db.collection::<VolumetricPeriodHourly>(
                &global_settings.volumetrics_hourly_gold_collection_name,
            );
        }
        _ => {
            // Five Minute

            let global_settings = load_settings().unwrap();

            let collection = db.collection::<VolumetricPeriodFiveMin>(
                &global_settings.volumetrics_five_minute_gold_collection_name,
            );
        }
    }
    // let collection = db.collection::()
    let collection = get_collection(&timeframe, db);
    let filters = doc! {"address": address.to_string()};
    let grouped_snaps = collection.find_one(filters, None).await?;
    match grouped_snaps {
        Some(volumes) => {
            let volume = volumes
                .volumetrics
                .last()
                .expect("expect grouped volumetrics to contain at least one volume");

            Ok(volume.clone())
        }
        _ => panic!("no volumetrics found for address {}", address),
    }
}

pub async fn get_period_volumetrics(
    db: &Database,
    address: &str,
    timeframe: Timeframe,
) -> Result<Vec<Volumetric>, Box<dyn std::error::Error>> {
    let collection = get_collection(&timeframe, db);
    let filters = doc! {"address": address};
    let grouped_snaps = collection.find_one(filters, None).await?;

    match grouped_snaps {
        Some(volumes) => {
            let volumes = volumes.volumetrics;
            Ok(volumes)
        }
        _ => panic!("no volumetrics found for address {}", address),
    }
}
