use futures::TryStreamExt;
use mongodb::{
    bson::doc,
    options::FindOptions
};

use crate::mongo::Mongo;

use super::types::Log;

pub async fn get_logs(
    db: &Mongo,
    address: Option<String>,
    timestamp_from: Option<i64>,
    timestamp_to: Option<i64>,
) -> Result<Vec<Log>, Box<dyn std::error::Error>> {
    let mut logs = Vec::new();
    let logs_collection = db.collection::<Log>("logs_bronze");
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

#[cfg(test)]

mod tests {
    use crate::mongo::{MongoConfig};

    use super::*;

    #[tokio::test]
    async fn test_get_logs() {
        let config = MongoConfig {
            uri: "mongodb://localhost:27017".to_string(),
            database: "test".to_string(),
        };

        let db = Mongo::new(&config).await.unwrap();

        let logs = get_logs(&db, Some(String::from("thisisamockaddress")), None, None).await.unwrap();

        assert_ne!(logs.len(), 0);

        let logs = get_logs(&db, None, Some(1458128819358319), Some(1658128819358319)).await.unwrap();

        assert_ne!(logs.len(), 0);
    }
}