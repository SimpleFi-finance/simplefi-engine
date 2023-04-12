use crate::mongo::{
    Mongo,
};

use super::types::Block;

pub async fn save_blocks (db: &Mongo, blocks: Vec<Block>) -> Result<(), Box<dyn std::error::Error>> {
    let blocks_collection = db.collection::<Block>("blocks_bronze");

    blocks_collection.insert_many(blocks, None).await?;

    Ok(())
}

#[cfg(test)]

mod tests {
    use crate::mongo::MongoConfig;

    use super::*;
    use futures::TryStreamExt;
    use mongodb::{bson::doc, options::{IndexOptions, FindOptions}, IndexModel};

    #[tokio::test]
    async fn test_save_blocks() -> Result<(), Box<dyn std::error::Error>>{
        // Create a new MongoDB configuration
        let config = MongoConfig {
            uri: "mongodb://localhost:27017".to_string(),
            database: "test".to_string(),
        };

        // Create a new MongoDB client
        let mongo = Mongo::new(&config).await.unwrap();

        let unique_options = IndexOptions::builder().unique(true).build();
        let model = IndexModel::builder()
            .keys(doc! {"number": 1})
            .options(unique_options)
            .build();

        mongo
            .collection::<Block>("blocks_bronze")
            .create_index(model, None)
            .await
            .expect("error creating index!");

        // Get a handle to the "blocks" collection
        let blocks_collection = mongo.collection::<Block>("blocks_bronze");
        let mock_block = Block {
            timestamp: 150000000,
            year: 2018,
            month: 1,
            day: 1,
            number: 12345,
            hash: String::from("testhash"),
            parent_hash: String::from("parenthash"),
            uncles_hash: String::from("unclehash"),
            author: String::from("author"),
            state_root: String::from("stateroot"),
            transactions_root: String::from("transactionsroot"),
            receipts_root: String::from("receiptsroot"),
            gas_used: String::from("gasused"),
            gas_limit: String::from("gaslimit"),
            extra_data: String::from("extradata"),
            logs_bloom: String::from("logsbloom"),
            difficulty: String::from("difficulty"),
            total_difficulty: String::from("totaldifficulty"),
            seal_fields: vec![String::from("sealfields")],
            uncles: vec![String::from("uncles")],
            transactions: vec![String::from("tx1")],
            size: String::from("size"),
            mix_hash: String::from("mixhash"),
            nonce: String::from("nonce"),
            base_fee_per_gas: String::from("basefeepergas"),
        };
        // Insert a new user
        save_blocks(&mongo, vec![mock_block.clone()]).await.unwrap_or(());

        // Find the user we just inserted
        let find_options = FindOptions::builder()
            .sort(doc! { "timestamp": 1 })
            .projection(doc!{"_id": 0})
            .build();

        let filter = doc! {
            "number": {
                "$eq": 12345
            },
        };
        let mut blocks_cursor = blocks_collection
            .find(filter, find_options)
            .await
            .unwrap();

        let mut blocks = Vec::new();

        while let Some(block) = blocks_cursor.try_next().await? {
            blocks.push(block);
        }

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0], mock_block);
        Ok(())
    }
}