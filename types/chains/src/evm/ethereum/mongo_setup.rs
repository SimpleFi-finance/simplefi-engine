// command to init collection in database with indexes

use mongodb::{options::IndexOptions, IndexModel, bson::doc};
use serde::de::DeserializeOwned;
use mongo_types::Mongo;

pub async fn init_blocks_bronze<T: serde::Serialize + DeserializeOwned>(db: &Mongo, collection_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let blocks_db = db;

    let unique_options = IndexOptions::builder().unique(true).build();
    let unique_number = IndexModel::builder()
        .keys(doc! {"number": 1})
        .options(unique_options.clone())
        .build();

    let indexes_generic = IndexModel::builder()
        .keys(doc! {"timestamp": 1})
        .options(IndexOptions::builder().build())
        .build();

    blocks_db
        .collection::<T>(collection_name)
        .create_index(unique_number, None)
        .await
        .expect("error creating block unique index!");

    blocks_db
        .collection::<T>(collection_name)
        .create_index(indexes_generic, None)
        .await
        .expect("error creating ts index!");
    
    Ok(())
}

pub async fn init_decoding_error_bronze<T: serde::Serialize + DeserializeOwned + Sync + Send + Unpin>(db: &Mongo, collection_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    
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
        .collection::<T>(collection_name)
        .create_index(timestamp_index, None)
        .await
        .expect("error creating timestamp index!");

    decoding_error_db
        .collection::<T>(collection_name)
        .create_index(contract_index, None)
        .await
        .expect("error creating unique index!");

    Ok(())
}

pub async fn init_logs_bronze<T: serde::Serialize + DeserializeOwned>(db: &Mongo, collection_name: &String) -> Result<(), Box<dyn std::error::Error>> {
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
        .collection::<T>(collection_name)
        .create_index(timestamp_index, None)
        .await
        .expect("error creating timestamp index!");
    logs_db
        .collection::<T>(collection_name)
        .create_index(unique_index, None)
        .await
        .expect("error creating unique index!");

    logs_db
        .collection::<T>(collection_name)
        .create_index(block_index, None)
        .await
        .expect("error creating logs index!");

    logs_db
        .collection::<T>(collection_name)
        .create_index(address_index, None)
        .await
        .expect("error creating address index!");
    
    Ok(())
}

pub async fn init_txs_bronze<T: serde::Serialize + DeserializeOwned + Sync + Send + Unpin>(db: &Mongo, collection_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let txs_db = db;

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
        .collection::<T>(collection_name)
        .create_index(unique_number, None)
        .await
        .expect("error creating block unique index!");

    txs_db
        .collection::<T>(collection_name)
        .create_index(indexes_generic, None)
        .await
        .expect("error creating ts index!");
    
    txs_db
        .collection::<T>(collection_name)
        .create_index(number_generic, None)
        .await
        .expect("error creating ts index!");

    Ok(())
}