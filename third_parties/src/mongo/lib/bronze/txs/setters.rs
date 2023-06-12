use serde::de::DeserializeOwned;
use settings::load_settings;

use crate::mongo::Mongo;

pub async fn save_txs<T: serde::Serialize + DeserializeOwned + Sync + Send + Unpin>(db: &Mongo, txs: Vec<T>) -> Result<(), Box<dyn std::error::Error>> {
    let global_settings = load_settings().unwrap();
    
    let txs_collection = db.collection::<T>(&global_settings.txs_bronze_collection_name);
    if txs.len() == 0 {
        return Ok(())
    }
    txs_collection.insert_many(txs, None).await?;

    Ok(())
}