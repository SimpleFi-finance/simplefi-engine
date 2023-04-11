use crate::mongo::{MongoConfig, Mongo};
use mongodb::{bson::doc, options::FindOptions};
use super::types::Block;
use futures::stream::TryStreamExt;

async fn blocks_db () -> Result<Mongo, Box<dyn std::error::Error>> {
    let blocks_db_config = MongoConfig {
        uri: String::from("mongodb://localhost:27017"),
        database: "blocks_bronze".to_string(),
    };

    let blocks_db = Mongo::new(&blocks_db_config)
        .await
        .expect("Failed to create mongo Client");

    Ok(blocks_db)
}


pub async fn get_blocks_by_timestamp(
    timestamp_from: Option<i64>,
    timestamp_to: Option<i64>,
) -> Result<Vec<Block>, Box<dyn std::error::Error>> {

    // todo implement pagination
    // todo implement conditional filtering

    let blocks_db = blocks_db().await?;
    let blocks_collection = blocks_db.collection::<Block>("blocks");

    let filter = doc! {
        "timestamp": {
            "$gte": timestamp_from.unwrap(),
            "$lte": timestamp_to.unwrap()
        },
    };

    let find_options = FindOptions::builder()
        .sort(doc! { "timestamp": 1 })
        .projection(doc!{"_id": 0})
        .build();

    let mut cursor = blocks_collection.find(filter, find_options).await?;
    let mut blocks = Vec::new();

    while let Some(block) = cursor.try_next().await? {
        blocks.push(block);
    }

    Ok(blocks)
}