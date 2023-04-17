use crate::mongo::Mongo;

use super::types::Block;

pub async fn save_blocks (db: &Mongo, blocks: Vec<Block>) -> Result<(), Box<dyn std::error::Error>> {
    let blocks_collection = db.collection::<Block>("blocks_bronze");
    blocks_collection.insert_many(blocks, None).await?;

    Ok(())
}

#[cfg(test)]

mod tests {
    use crate::mongo::{
        MongoConfig,
        Mongo
    };

    use super::*;
    use futures::TryStreamExt;
    use mongodb::{bson::doc, options::FindOptions};

    #[tokio::test]
    async fn test_save_blocks() -> Result<(), Box<dyn std::error::Error>>{
        // Create a new MongoDB configuration
        let config = MongoConfig {
            uri: "mongodb://localhost:27017".to_string(),
            database: "test".to_string(),
        };

        // Create a new MongoDB client
        let db = Mongo::new(&config).await.unwrap();

        // Get a handle to the "blocks" collection
        let blocks_collection = db.collection::<Block>("blocks_bronze");
        let mock_block = Block {
            timestamp: 150000000,
            year: Some(2018),
            month: Some(1),
            day: Some(1),
            number: 12345,
            hash: Some(String::from("testhash")),
            parent_hash: Some(String::from("parenthash")),
            state_root: Some(String::from("stateroot")),
            transactions_root: Some(String::from("transactionsroot")),
            receipts_root: Some(String::from("receiptsroot")),
            gas_used: 4,
            gas_limit: 4,
            extra_data: Some(String::from("extradata")),
            logs_bloom: Some(String::from("logsbloom")),
            difficulty: 3,
            mix_hash: Some(String::from("mixhash")),
            nonce: 3,
            base_fee_per_gas: 3,
            miner: Some(String::from("miner")),
            uncles_hash: Some(String::from("sha3uncles")),
            withdrawals_root: Some(String::from("withdrawalsroot")),
        };
        // Insert a new user
        save_blocks(&db, vec![mock_block.clone()]).await.unwrap_or(());

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