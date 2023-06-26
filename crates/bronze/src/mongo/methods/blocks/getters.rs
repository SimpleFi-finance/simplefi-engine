use chains_types::SupportedChains;
use log::debug;
use mongodb::{
    bson::doc,
    options::{FindOneOptions, FindOptions},
};
use serde::de::DeserializeOwned;

use chains_types::common::chain::Info;
use chrono::Utc;
use futures::stream::TryStreamExt;
use mongo_types::Mongo;

pub async fn get_blocks<T: serde::Serialize + DeserializeOwned + Sync + Send + Unpin>(
    db: &Mongo,
    chain: SupportedChains,
    timestamp_from: Option<i64>,
    timestamp_to: Option<i64>,
    blocknumber_from: Option<i64>,
    blocknumber_to: Option<i64>,
) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    // TODO: implement pagination

    let mut blocks = Vec::new();

    if blocknumber_from.is_some() && timestamp_from.is_some() {
        panic!("Only one bteween blocknumber_from and timestamp_from can be set");
    }

    let find_options = FindOptions::builder()
        .sort(doc! { "timestamp": -1 })
        .projection(doc! {"_id": 0})
        .build();

    let collection_name = chain.resolve_collection_name(
        &data_lake_types::SupportedDataTypes::Blocks,
        &data_lake_types::SupportedDataLevels::Bronze,
    );

    let blocks_collection = db.collection::<T>(&collection_name);

    if timestamp_from.is_some() {
        let ts_now = Utc::now().timestamp_micros();

        let doc = doc! {
            "timestamp": {
                "$gte": timestamp_from.unwrap(),
                "$lte": timestamp_to.unwrap_or(ts_now)
            },
        };

        let mut cursor = blocks_collection.find(doc, find_options.clone()).await?;

        while let Some(block) = cursor.try_next().await? {
            blocks.push(block);
        }
    }

    if blocknumber_from.is_some() {
        let filter = if blocknumber_to.is_some() {
            doc! {
                "number": {
                    "$gte": blocknumber_from.unwrap(),
                    "$lte": blocknumber_to.unwrap()
                }
            }
        } else {
            doc! {
                "number": {
                    "$gte": blocknumber_from.unwrap()
                }
            }
        };

        let mut cursor = blocks_collection.find(filter, find_options.clone()).await?;

        while let Some(block) = cursor.try_next().await? {
            blocks.push(block);
        }
    }

    if blocknumber_from.is_none() && timestamp_from.is_none() {
        let find_options = FindOneOptions::builder()
            .sort(doc! { "timestamp": -1 })
            .projection(doc! {"_id": 0})
            .build();

        let block = blocks_collection
            .find_one(None, find_options.clone())
            .await
            .unwrap();
        match block {
            Some(block) => {
                blocks.push(block);
            }
            None => {
                debug!("No blocks found");
            }
        }
    }

    Ok(blocks)
}

pub async fn get_block<T: serde::Serialize + DeserializeOwned + Sync + Send + Unpin>(
    db: &Mongo,
    chain: SupportedChains,
    block_number: Option<i64>,
    timestamp: Option<i64>,
) -> Result<Option<T>, Box<dyn std::error::Error>> {
    // TODO: implement filter logic

    if block_number.is_some() && timestamp.is_some() {
        panic!("Only one bteween block_number and timestamp can be set");
    }

    if block_number.is_none() && timestamp.is_none() {
        panic!("One between block_number and timestamp must be set");
    }

    let collection_name = chain.resolve_collection_name(
        &data_lake_types::SupportedDataTypes::Blocks,
        &data_lake_types::SupportedDataLevels::Bronze,
    );

    let blocks_collection = db.collection::<T>(&collection_name);
    let find_options = FindOneOptions::builder()
        .sort(doc! { "timestamp": 1 })
        .projection(doc! {"_id": 0})
        .build();

    if block_number.is_some() {
        let filter = doc! {
            "number": {
                "$gte": block_number,
            }
        };

        let block = blocks_collection
            .find_one(filter, find_options.clone())
            .await
            .unwrap();
        return Ok(block);
    }

    let filter = doc! {
        "timestamp": {
            "$gte": timestamp,
        }
    };

    let block = blocks_collection
        .find_one(filter, find_options.clone())
        .await
        .unwrap();
    return Ok(block);
}
