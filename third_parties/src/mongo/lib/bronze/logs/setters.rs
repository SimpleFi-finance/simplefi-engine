use serde::de::DeserializeOwned;
use settings::load_settings;

use crate::mongo::{
    Mongo,
};

pub async fn save_logs<T: serde::Serialize + DeserializeOwned>(db: &Mongo, logs: Vec<T>) -> Result<(), Box<dyn std::error::Error>> {
    let global_settings = load_settings().unwrap();
    // todo add chain to collection name
    let logs_collection = db.collection::<T>(&global_settings.logs_bronze_collection_name);
    if logs.len() == 0 {
        return Ok(())
    }

    logs_collection.insert_many(logs, None).await?;

    Ok(())
}