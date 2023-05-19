use settings::load_settings;
use crate::mongo::{MongoConfig, Mongo};

pub async fn mongo_db () -> Result<Mongo, Box<dyn std::error::Error>> {
    let global_settings = load_settings().unwrap();

    let db_config = MongoConfig {
        uri: global_settings.mongodb_uri,
        database: global_settings.mongodb_database_name,
    };

    let mongo = Mongo::new(&db_config)
        .await
        .expect("Failed to create mongo Client");

    Ok(mongo)
}