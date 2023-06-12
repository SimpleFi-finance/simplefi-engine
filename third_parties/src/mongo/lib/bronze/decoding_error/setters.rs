use serde::de::DeserializeOwned;
use settings::load_settings;

use crate::mongo::Mongo;

pub async fn save_decoding_error<T: serde::Serialize + DeserializeOwned + Sync + Send + Unpin>(db: &Mongo, errors: Vec<T>) -> Result<(), Box<dyn std::error::Error>>  {

    let global_settings = load_settings().unwrap();

    let decoding_error_collection = db.collection::<T>(&global_settings.decoding_error_bronze_collection_name);
    if errors.len() == 0 {
        return Ok(())
    }
    decoding_error_collection.insert_many(errors, None).await.unwrap();

    Ok(())
}