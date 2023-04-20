use bson::Binary;
use chrono::Utc;
use log::info;
use mongodb::{ Collection };

use shared_types::mongo::abi::AbiCollection;

pub async fn save_abi(
    abis_collection: &Collection<AbiCollection>,
    index: &u32,
    abi_binary: &Binary,
) {
    let abi = abi_binary.clone();

    abis_collection
        .insert_one(
            AbiCollection {
                timestamp: Utc::now().timestamp_millis() as i64,
                index: *index,
                abi: abi,
            },
            None,
        )
        .await
        .expect("Failed to insert document");

    info!("ABI inserted {:?}", index)
}
