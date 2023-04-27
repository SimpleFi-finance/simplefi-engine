use bson::Binary;
use log::info;
use mongodb::{ bson::{ doc }, options::FindOneOptions, Collection };

use shared_types::mongo::abi::AbiCollection;

pub async fn get_index(collection: &Collection<AbiCollection>, abi: &Binary) -> (u32, bool) {
    info!("get_index called");

    let filter = doc! {"abi": abi};

    let options = FindOneOptions::builder().projection(doc! {
        "index": 1,
        "abi": 0,
        "_id": 0
    }).build();

    if let Ok(result) = collection.find_one(filter, options).await {
        if let Some(doc) = result {
            let index = doc.index;

            return (index, true)
        }
    }

    // If the document is not found, get the latest index in the collection
    let options = mongodb::options::FindOneOptions::builder()
        .projection(doc! {"index": 1, "abi": 0, "_id": 0})
        .sort(doc! {"_id": -1})
        .build();

    let filter = doc! {};

    if let Ok(result) = collection.find_one(filter, options).await {
        if let Some(doc) = result {
            let index = doc.index;

            return (index + 1, false)
        }
    }

    // If the collection is empty, return 0
    (0, false)
}
