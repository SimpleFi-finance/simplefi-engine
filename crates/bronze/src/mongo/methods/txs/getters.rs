use chains_types::SupportedChains;
use chains_types::common::chain::Info;
use mongodb::{bson::doc, options::{FindOptions}};
use serde::de::DeserializeOwned;
use futures::stream::TryStreamExt;
use chrono::Utc;
use mongo_types::Mongo;


pub async fn get_txs<T: serde::Serialize + DeserializeOwned + Sync + Send + Unpin>(
    db: &Mongo,
    chain: SupportedChains,
    address: Option<String>,
    timestamp_from: Option<i64>,
    timestamp_to: Option<i64>,
    blocknumber_from: Option<i64>,
    blocknumber_to: Option<i64>,
)  -> Result<Vec<T>, Box<dyn std::error::Error>> {

    let mut txs = Vec::new();

    if address.is_none() && timestamp_from.is_none() && timestamp_to.is_none() && blocknumber_from.is_none() && blocknumber_to.is_none() {
        panic!("At least one filter must be set");
    }

    if timestamp_from.is_some() && blocknumber_from.is_some() {
        panic!("Only one between timestamp_from and blocknumber_from can be set");
    }

    let find_options = FindOptions::builder()
        .sort(doc! { "timestamp": 1 })
        .projection(doc!{"_id": 0})
        .build();

    let collection_name = chain.resolve_collection_name(
        &data_lake_types::SupportedDataTypes::Transactions,
        &data_lake_types::SupportedDataLevels::Bronze,
    );

    let txs_collection = db.collection::<T>(&collection_name);

    let mut filter = doc!{};

    if address.is_some() {

        let value = doc!{"$or": [
            doc!{"from": address.clone().unwrap()}, 
            doc!{"to": address.clone().unwrap() }
        ]};
        filter = value;
    }

    if blocknumber_from.is_some() {
        let blocknumber_doc = doc! {
            "$gte": blocknumber_from.unwrap(),
            "$lte": blocknumber_to.unwrap_or(999999999999999999)
        };
        filter.insert("block_number", blocknumber_doc);
    }

    if timestamp_from.is_some() {
        let ts_now = Utc::now().timestamp_micros();
        let ts_doc = doc! {
            "$gte": timestamp_from.unwrap(),
            "$lte": timestamp_to.unwrap_or(ts_now)
        };
        filter.insert("timestamp", ts_doc);
    }

    let mut cursor = txs_collection.find(filter, find_options.clone()).await?;

    while let Some(tx) = cursor.try_next().await? {
        txs.push(tx);
    }

    Ok(txs)
}