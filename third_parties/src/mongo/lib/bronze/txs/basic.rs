// command to init collection in database with indexes

use mongodb::{options::IndexOptions, IndexModel, bson::doc};
use settings::load_settings;

use crate::mongo::{MongoConfig, Mongo};

use super::types::Tx;

pub async fn txs_db () -> Result<Mongo, Box<dyn std::error::Error>> {
    let global_settings = load_settings().unwrap();

    let txs_db_config = MongoConfig {
        uri: global_settings.mongodb_uri,
        database: global_settings.mongodb_database_name,
    };

    let txs_db = Mongo::new(&txs_db_config)
        .await
        .expect("Failed to create mongo Client");

    Ok(txs_db)
}

pub async fn init_txs_bronze(db: &Mongo) -> Result<(), Box<dyn std::error::Error>> {
    let txs_db = db;
    let global_settings = load_settings().unwrap();

    let unique_options = IndexOptions::builder().unique(true).build();
    let unique_number = IndexModel::builder()
        .keys(doc! {"hash": 1})
        .options(unique_options.clone())
        .build();

    let indexes_generic = IndexModel::builder()
        .keys(doc! {"timestamp": 1})
        .options(IndexOptions::builder().build())
        .build();

    let number_generic = IndexModel::builder()
        .keys(doc! {"block_number": 1})
        .options(IndexOptions::builder().build())
        .build();

    txs_db
        .collection::<Tx>(&global_settings.txs_bronze_collection_name)
        .create_index(unique_number, None)
        .await
        .expect("error creating block unique index!");

    txs_db
        .collection::<Tx>(&global_settings.txs_bronze_collection_name)
        .create_index(indexes_generic, None)
        .await
        .expect("error creating ts index!");
    
    txs_db
        .collection::<Tx>(&global_settings.txs_bronze_collection_name)
        .create_index(number_generic, None)
        .await
        .expect("error creating ts index!");

    Ok(())
}