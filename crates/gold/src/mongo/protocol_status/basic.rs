// command to init collection in database with indexes

use mongo_types::{Mongo, MongoConfig};
use simplefi_engine_settings::load_settings;

pub async fn protocol_status_db() -> Result<Mongo, Box<dyn std::error::Error>> {
    let global_settings = load_settings().unwrap();

    let protocol_status_db_config = MongoConfig {
        uri: global_settings.mongodb_uri,
        database: global_settings.mongodb_database_name,
    };

    let protocol_status_db = Mongo::new(&protocol_status_db_config)
        .await
        .expect("Failed to create mongo Client");

    Ok(protocol_status_db)
}

pub async fn init_protocol_status_gold(db: &Mongo) -> Result<(), Box<dyn std::error::Error>> {
    let global_settings = load_settings().unwrap();

    let protocol_status = db;

    // let unique_options = IndexOptions::builder().unique(true).build();
    // let unique_number = IndexModel::builder()
    //     .keys(doc! {"number": 1})
    //     .options(unique_options.clone())
    //     .build();

    // let indexes_generic = IndexModel::builder()
    //     .keys(doc! {"timestamp": 1})
    //     .options(IndexOptions::builder().build())
    //     .build();

    // blocks_db
    //     .collection::<Block>(&global_settings.blocks_bronze_collection_name)
    //     .create_index(unique_number, None)
    //     .await
    //     .expect("error creating block unique index!");

    // blocks_db
    //     .collection::<Block>(&global_settings.blocks_bronze_collection_name)
    //     .create_index(indexes_generic, None)
    //     .await
    //     .expect("error creating ts index!");

    Ok(())
}
