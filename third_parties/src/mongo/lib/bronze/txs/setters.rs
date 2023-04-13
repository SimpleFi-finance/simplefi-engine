use crate::mongo::Mongo;

use super::types::Tx;

pub async fn save_txs(db: &Mongo, txs: Vec<Tx>) -> Result<(), Box<dyn std::error::Error>> {
    let txs_collection = db.collection::<Tx>("txs_bronze");

    txs_collection.insert_many(txs, None).await?;

    Ok(())
}


#[cfg(test)]

mod tests {
    use crate::mongo::{
        MongoConfig,
        Mongo, lib::bronze::txs::mocks::get_mock_tx
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
        let txs_collection = db.collection::<Tx>("txs_bronze");
        // Insert a new user
        let mock_txs = get_mock_tx(&None, &None, Some(10));
        save_txs(&db, mock_txs).await.unwrap_or(());

        // Find the user we just inserted
        let find_options = FindOptions::builder()
            .sort(doc! { "timestamp": 1 })
            .projection(doc!{"_id": 0})
            .build();

        let mut txs_cursor = txs_collection
            .find(None, find_options)
            .await
            .unwrap();

        let mut txs = Vec::new();

        while let Some(block) = txs_cursor.try_next().await? {
            txs.push(block);
        }

        assert_ne!(txs.len(), 0);
        let tx_1 = txs[0].clone();

        let filter = doc! {
            "block_number": {
                "$eq": tx_1.block_number
            },
        };

        println!("filter: {:?}", filter);

        let tx_filter = txs_collection.find_one(filter, None).await?;
        assert_eq!(tx_filter.unwrap(), tx_1);

        Ok(())
    }
}