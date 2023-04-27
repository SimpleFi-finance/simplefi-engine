use chrono::Utc;
use log::debug;
use mongodb::{ Collection };

use shared_types::mongo::abi::AbiJSONCollection;

pub async fn save_abi_json(
    abis_collection: &Collection<AbiJSONCollection>,
    index: &u32,
    abi_binary: &String,
) {
    debug!("Saving ABI JSON called");

    let abi = abi_binary.clone();

    abis_collection
        .insert_one(
            AbiJSONCollection {
                timestamp: Utc::now().timestamp_millis() as i64,
                index: *index,
                abi: abi,
            },
            None,
        )
        .await
        .expect("Failed to insert document");

    debug!("ABI inserted {:?}", index)
}
