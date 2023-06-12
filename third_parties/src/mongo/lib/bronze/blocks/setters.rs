use serde::de::DeserializeOwned;
use settings::load_settings;

use crate::mongo::Mongo;

pub async fn save_blocks<T: serde::Serialize + DeserializeOwned + Sync + Send + Unpin>(db: &Mongo, blocks: Vec<T>) -> Result<(), Box<dyn std::error::Error>> {
    let global_settings = load_settings().unwrap();
    
    let blocks_collection = db.collection::<T>(&global_settings.blocks_bronze_collection_name);

    if blocks.len() == 0 {
        return Ok(())
    }

    blocks_collection.insert_many(blocks, None).await?;

    Ok(())
}