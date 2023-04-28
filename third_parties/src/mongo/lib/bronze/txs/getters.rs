use crate::mongo::{Mongo};
use mongodb::{bson::doc, options::{FindOptions}};
use super::types::Tx;
use futures::stream::TryStreamExt;
use chrono::Utc;


pub async fn get_txs (
    db: &Mongo,
    address: Option<String>,
    timestamp_from: Option<i64>,
    timestamp_to: Option<i64>,
    blocknumber_from: Option<i64>,
    blocknumber_to: Option<i64>,
)  -> Result<Vec<Tx>, Box<dyn std::error::Error>> {
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

    let txs_collection = db.collection::<Tx>("txs_bronze");

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

#[cfg(test)]

mod tests {
    use crate::mongo::MongoConfig;

    use super::*;

    #[tokio::test]

    async fn test_get_txs() -> Result<(), Box<dyn std::error::Error>> {
        let config = MongoConfig {
            uri: "mongodb://localhost:27017".to_string(),
            database: "test".to_string(),
        };

        let mongo = Mongo::new(&config).await.unwrap();

        let txs = get_txs(
            &mongo, 
            Some(String::from("from1")), 
            None, 
            None, 
            None, 
            None
        ).await.unwrap();

        assert_ne!(txs.len(), 0);

        let txs = get_txs(
            &mongo, 
            Some(String::from("from")),
            None, 
            None, 
            None, 
            None
        ).await.unwrap();

        let tx_1 = txs[0].clone();

        let txs = get_txs(
            &mongo, 
            None,
            Some(tx_1.timestamp), 
            None, 
            None, 
            None
        ).await.unwrap();
        assert_ne!(txs.len(), 0);

        let txs = get_txs(
            &mongo, 
            None,
            None, 
            None,
            Some(tx_1.block_number),
            None
        ).await.unwrap();

        assert_ne!(txs.len(), 0);

        Ok(())
    } 
}