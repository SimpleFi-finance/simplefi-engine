use chains_types::{SupportedChains};
use chains_types::common::chain::Info;
use futures::{TryStreamExt};
use mongodb::{
    bson::doc,
    options::FindOptions
};
use serde::de::DeserializeOwned;
use mongo_types::Mongo;

pub async fn get_logs<T: serde::Serialize + DeserializeOwned + Sync + Send + Unpin>(
    db: &Mongo,
    chain: SupportedChains,
    address: Option<String>,
    timestamp_from: Option<i64>,
    timestamp_to: Option<i64>,
) -> Result<Vec<T>, Box<dyn std::error::Error>> {

    let mut logs = Vec::new();

    let collection_name = chain.resolve_collection_name(
        &data_lake_types::SupportedDataTypes::Logs,
        &data_lake_types::SupportedDataLevels::Bronze,
    );

    let logs_collection = db.collection::<T>(&collection_name);
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