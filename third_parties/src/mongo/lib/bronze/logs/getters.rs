use futures::{TryStreamExt};
use mongodb::{
    bson::doc,
    options::FindOptions
};
use serde::de::DeserializeOwned;
use settings::load_settings;

use crate::mongo::Mongo;

pub async fn get_logs<T: serde::Serialize + DeserializeOwned + Sync + Send + Unpin>(
    db: &Mongo,
    address: Option<String>,
    timestamp_from: Option<i64>,
    timestamp_to: Option<i64>,
) -> Result<Vec<T>, Box<dyn std::error::Error>> {

    let global_settings = load_settings().unwrap();
    let mut logs = Vec::new();
    let logs_collection = db.collection::<T>(&global_settings.logs_bronze_collection_name);
    let find_options = FindOptions::builder()
        .sort(doc! { "timestamp": 1 })
        .projection(doc!{"_id": 0})
        .build();

    let mut filter = doc!{};

    if address.is_some() {
        let address_filter = doc! {
                "$eq": address.unwrap()
            };
        
        filter.insert("address", address_filter);
    }

    if timestamp_from.is_some() {
        let timestamp_filter = doc! {
            "$gte": timestamp_from.unwrap(),
            "$lte": timestamp_to.unwrap_or(chrono::Utc::now().timestamp_micros())
        };
        
        filter.insert("timestamp", timestamp_filter);
    }

    let mut cursor = logs_collection.find(filter, find_options.clone()).await?;

    while let Some(log) = cursor.try_next().await? {
        logs.push(log);
    }

    Ok(logs)
}