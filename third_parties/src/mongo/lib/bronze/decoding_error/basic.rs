// command to init collection in database with indexes

use mongodb::{options::IndexOptions, IndexModel, bson::doc};
use settings::load_settings;

use crate::mongo::{MongoConfig, Mongo};

use super::types::DecodingError;

pub async fn decoding_error_db () -> Result<Mongo, Box<dyn std::error::Error>> {

    let global_settings = load_settings().unwrap();

    let decoding_error_db_config = MongoConfig {
        uri: global_settings.mongodb_uri,
        database: global_settings.mongodb_database_name,
    };

    let error_db = Mongo::new(&decoding_error_db_config)
        .await
        .expect("Failed to create mongo Client");

    Ok(error_db)
}

pub async fn init_decoding_error_bronze(db: &Mongo) -> Result<(), Box<dyn std::error::Error>> {
    
    let global_settings = load_settings().unwrap();
    
    let decoding_error_db = db;

    let timestamp_index = IndexModel::builder()
        .keys(doc! {"timestamp": 1})
        .options(IndexOptions::builder().build())
        .build();

    let contract_index = IndexModel::builder()
        .keys(doc! {"contract_address": 1})
        .options(IndexOptions::builder().build())
        .build();

    decoding_error_db
        .collection::<DecodingError>(&global_settings.decoding_error_bronze_collection_name)
        .create_index(timestamp_index, None)
        .await
        .expect("error creating timestamp index!");

    decoding_error_db
        .collection::<DecodingError>(&global_settings.decoding_error_bronze_collection_name)
        .create_index(contract_index, None)
        .await
        .expect("error creating unique index!");

    Ok(())
}