use crate::mongo::{
    Mongo,
};

use super::types::Log;

pub async fn save_logs(db: &Mongo, logs: Vec<Log>) -> Result<(), Box<dyn std::error::Error>> {
    let logs_collection = db.collection::<Log>("logs_bronze");

    logs_collection.insert_many(logs, None).await?;

    Ok(())
}

#[cfg(test)]

mod tests {
    use crate::mongo::{
        MongoConfig, 
        Mongo, 
        lib::bronze::logs::{
            basic::init_logs_bronze, 
            mocks::get_mock_logs,
            setters::save_logs, 
            types::Log
        }
    };

    use futures::TryStreamExt;
    use mongodb::{
        bson::doc, options::FindOptions
    };

    #[tokio::test]
    async fn init_db() {
        let config = MongoConfig {
            uri: "mongodb://localhost:27017".to_string(),
            database: "test".to_string(),
        };

        // Create a new MongoDB client
        let mongo = Mongo::new(&config).await.unwrap();

        init_logs_bronze(&mongo).await.unwrap();
    }
    #[tokio::test]
    async fn test_save_logs() -> Result<(), Box<dyn std::error::Error>> {
        let config = MongoConfig {
            uri: "mongodb://localhost:27017".to_string(),
            database: "test".to_string(),
        };

        let db = Mongo::new(&config).await?;

        let logs = get_mock_logs(&None, &None, Some(10));

        save_logs(&db, logs).await.unwrap_or(());

        let logs_collection = db.collection::<Log>("logs_bronze");

        let find_options = FindOptions::builder()
            .sort(doc! { "timestamp": 1 })
            .projection(doc!{"_id": 0})
            .build();
    
        let filter = doc! {
            "address": {
                "$eq": "thisisamockaddress"
            },
        };

        let mut logs_data = Vec::new();

        let mut cursor = logs_collection.find(filter, find_options).await?;
        
        while let Some(log) = cursor.try_next().await? {
            logs_data.push(log);
        }

        assert_eq!(logs_data.len(), 10);
        
        Ok(())
    }
}