use crate::mongo::{MongoConfig, Mongo};
use super::types::Block;

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

pub async fn save_blocks (blocks: Vec<Block>) -> Result<(), Box<dyn std::error::Error>> {
    let blocks_db = blocks_db().await?;
    let blocks_collection = blocks_db.collection::<Block>("blocks");

    blocks_collection.insert_many(blocks, None).await?;

    Ok(())
}