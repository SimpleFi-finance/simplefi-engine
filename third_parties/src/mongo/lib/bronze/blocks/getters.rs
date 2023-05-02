use crate::mongo::{Mongo};
use mongodb::{bson::doc, options::{FindOptions, FindOneOptions}};
use settings::load_settings;
use super::types::Block;
use futures::stream::TryStreamExt;
use chrono::Utc;

pub async fn get_blocks(
    db: &Mongo,
    timestamp_from: Option<i64>,
    timestamp_to: Option<i64>,
    blocknumber_from: Option<i64>,
    blocknumber_to: Option<i64>,
) -> Result<Vec<Block>, Box<dyn std::error::Error>> {
    let global_settings = load_settings().unwrap();
    // todo implement pagination

    let mut blocks = Vec::new();

    if blocknumber_from.is_some() && timestamp_from.is_some() {
        panic!("Only one bteween blocknumber_from and timestamp_from can be set");
    }

    let find_options = FindOptions::builder()
        .sort(doc! { "timestamp": -1 })
        .projection(doc!{"_id": 0})
        .build();

    // todo load settings
    let blocks_collection = db.collection::<Block>(&global_settings.blocks_bronze_collection_name);
    
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
        
        println!("blocknumber_from: {:?}", &blocknumber_from);
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
            .projection(doc!{"_id": 0})
            .build();

        let block = blocks_collection.find_one(None, find_options.clone()).await.unwrap();
        match block {
            Some(block) => {
                blocks.push(block);
            }
            None => {
                println!("No blocks found");
            }
        }
    }

    Ok(blocks)
}

pub async fn get_block(
    db: &Mongo,
    block_number: Option<i64>,
    timestamp: Option<i64>,
) -> Result<Option<Block>, Box<dyn std::error::Error>> {
    
    let global_settings = load_settings().unwrap();
    // todo implement filter logic

    if block_number.is_some() && timestamp.is_some() {
        panic!("Only one bteween block_number and timestamp can be set");
    }

    if block_number.is_none() && timestamp.is_none() {
        panic!("One bteween block_number and timestamp must be set");
    }

    let blocks_collection = db.collection::<Block>(&global_settings.blocks_bronze_collection_name);
    let find_options = FindOneOptions::builder()
        .sort(doc!{ "timestamp": 1 })
        .projection(doc!{"_id": 0})
        .build();

    if block_number.is_some() {

        let filter = doc! {
            "number": {
                "$gte": block_number,
            }
        };

        let block= blocks_collection.find_one(filter, find_options.clone()).await.unwrap();
        return Ok(block);
    }

    let filter = doc! {
        "timestamp": {
            "$gte": timestamp,
        }
    };

    let block = blocks_collection.find_one(filter, find_options.clone()).await.unwrap();
    return Ok(block);
}

#[cfg(test)]
mod tests {
    use crate::mongo::MongoConfig;

    use super::*;

    #[tokio::test]
    async fn test_get_blocks() -> Result<(), Box<dyn std::error::Error>> {
        let config = MongoConfig {
            uri: "mongodb://localhost:27017".to_string(),
            database: "test".to_string(),
        };

        // Create a new MongoDB client
        let mongo = Mongo::new(&config).await.unwrap();
        
        let blocks = get_blocks(&mongo, None, None, Some(1234), Some(12345)).await.unwrap();
        assert_eq!(blocks.len(), 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_block() -> Result<(), Box<dyn std::error::Error>> {
        let config = MongoConfig {
            uri: "mongodb://localhost:27017".to_string(),
            database: "test".to_string(),
        };

        // Create a new MongoDB client
        let mongo = Mongo::new(&config).await.unwrap();
        
        let block = get_block(&mongo, Some(12345), None).await.unwrap();
        assert_ne!(block, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_block_none() -> Result<(), Box<dyn std::error::Error>> {
        let config = MongoConfig {
            uri: "mongodb://localhost:27017".to_string(),
            database: "test".to_string(),
        };

        // Create a new MongoDB client
        let mongo = Mongo::new(&config).await.unwrap();
        
        let block = get_block(&mongo, Some(12346), None).await.unwrap();
        assert_eq!(block, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_block_ts() -> Result<(), Box<dyn std::error::Error>> {
        let config = MongoConfig {
            uri: "mongodb://localhost:27017".to_string(),
            database: "test".to_string(),
        };

        // Create a new MongoDB client
        let mongo = Mongo::new(&config).await.unwrap();
        
        let block = get_block(&mongo, None, Some(150000000)).await.unwrap();
        assert_ne!(block, None);
        Ok(())
    }
}