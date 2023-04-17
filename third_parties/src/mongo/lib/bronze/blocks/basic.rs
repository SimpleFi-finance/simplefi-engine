// command to init collection in database with indexes

use mongodb::{options::IndexOptions, IndexModel, bson::doc};

use crate::mongo::{MongoConfig, Mongo};

use super::types::Block;

pub async fn blocks_db () -> Result<Mongo, Box<dyn std::error::Error>> {
    // todo get mongo settings from config file
    let blocks_db_config = MongoConfig {
        uri: String::from("mongodb://localhost:27017"),
        database: "blocks_bronze".to_string(),
    };

    let blocks_db = Mongo::new(&blocks_db_config)
        .await
        .expect("Failed to create mongo Client");

    Ok(blocks_db)
}

pub async fn init_blocks_bronze(db: &Mongo) -> Result<(), Box<dyn std::error::Error>> {
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
        .collection::<Block>("blocks_bronze")
        .create_index(unique_number, None)
        .await
        .expect("error creating block unique index!");

    blocks_db
        .collection::<Block>("blocks_bronze")
        .create_index(indexes_generic, None)
        .await
        .expect("error creating ts index!");
    
    Ok(())
}