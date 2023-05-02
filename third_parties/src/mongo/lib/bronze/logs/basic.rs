// command to init collection in database with indexes

use mongodb::{options::IndexOptions, IndexModel, bson::doc};
use settings::load_settings;

use crate::mongo::{MongoConfig, Mongo};

use super::types::Log;

pub async fn logs_db () -> Result<Mongo, Box<dyn std::error::Error>> {
    let global_settings = load_settings().unwrap();

    let logs_db_config = MongoConfig {
        uri: global_settings.mongodb_uri,
        database: global_settings.mongodb_database_name,
    };

    let logs_db = Mongo::new(&logs_db_config)
        .await
        .expect("Failed to create mongo Client");

    Ok(logs_db)
}

pub async fn init_logs_bronze(db: &Mongo) -> Result<(), Box<dyn std::error::Error>> {
    let global_settings = load_settings().unwrap();
    let logs_db = db;

    let unique_options = IndexOptions::builder().unique(true).build();
    let unique_index = IndexModel::builder()
        .keys(doc! {
            "transaction_hash": 1, 
            "transaction_index": 1, 
            "log_index": 1
        })
        .options(unique_options.clone())
        .build();

    let timestamp_index = IndexModel::builder()
        .keys(doc! {"timestamp": 1})
        .options(IndexOptions::builder().build())
        .build();

    let block_index = IndexModel::builder()
        .keys(doc! {"block_number": 1})
        .options(IndexOptions::builder().build())
        .build();

    let address_index = IndexModel::builder()
        .keys(doc! {"address": 1})
        .options(IndexOptions::builder().build())
        .build();

    logs_db
        .collection::<Log>(&global_settings.logs_bronze_collection_name)
        .create_index(timestamp_index, None)
        .await
        .expect("error creating timestamp index!");
    logs_db
        .collection::<Log>(&global_settings.logs_bronze_collection_name)
        .create_index(unique_index, None)
        .await
        .expect("error creating unique index!");

    logs_db
        .collection::<Log>(&global_settings.logs_bronze_collection_name)
        .create_index(block_index, None)
        .await
        .expect("error creating logs index!");

    logs_db
        .collection::<Log>(&global_settings.logs_bronze_collection_name)
        .create_index(address_index, None)
        .await
        .expect("error creating address index!");
    
    Ok(())
}