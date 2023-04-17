use mongodb::{ Collection, bson::doc };
use futures::stream::StreamExt;

use crate::settings::load_settings;
use shared_types::mongo::abi::{ContractWithAbi, ContractAbiCollection, ContractWithAbiDocument};
use third_parties::mongo::{MongoConfig, Mongo};

///
/// Function to get tracked abis from mongo
///
/// # Arguments
///
/// * `addresses` - Vec<String> of addresses to get abi for
///
/// # Returns
///
/// * `Result<Vec<ContractWithAbi>, Box<dyn std::error::Error>>` - Result of Vec<ContractWithAbi> or Error
///
/// # Example
///
/// ```
/// use abi_discovery::get_tracked_abi_from_mongo;
///
/// let addresses = vec!["0x.."];
///
/// let result = get_tracked_abi_from_mongo(addresses).await;
///
/// match result {
///    Ok(abis) => println!("abis: {:?}", abis),
///   Err(e) => println!("error: {:?}", e),
/// }
/// ```
///
/// # Panics
///
/// This function will panic if the mongodb_uri is not set in the settings file
///
pub async fn get_tracked_abi_from_mongo(
    addresses: Vec<String>,
) -> Result<Vec<ContractWithAbi>, Box<dyn std::error::Error>> {
    let settings = load_settings()?;

    let mongo_uri = settings.mongodb_uri;
    let mongo_engine_db = settings.mongodb_engine_db;

    let config = MongoConfig {
        uri: mongo_uri,
        database: mongo_engine_db,
    };

    let db = Mongo::new(&config).await?;

    let contract_abi_collection: Collection<ContractAbiCollection> = db.collection("contract-abi");
    let query = doc! { "address": { "$in": addresses }};

    let pipeline = vec![
        doc! {"$match": query},
        doc! {
            "$lookup": {
                "from": "abis",
                "localField": "index",
                "foreignField": "index",
                "as": "abis"
            }
        },
        doc! {
            "$unwind": "$abis"
        },
        doc! {
            "$project": {
                "timestamp": 1,
                "address": 1,
                "abi": "$abis.abi",
            }
        }
    ];

    let mut cursor = contract_abi_collection.aggregate(pipeline, None).await?;
    let mut results: Vec<ContractWithAbi> = vec![];

    while let Some(doc) = cursor.next().await {
        let document = bson::from_document::<ContractWithAbiDocument>(doc?)?;

        let result = ContractWithAbi {
            timestamp: document.timestamp,
            address: document.address,
            abi: document.abi.bytes.to_vec(),
        };

        results.push(result);
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_tracked_abi_from_mongo() {
        // TODO: implement tear up and down for these fixtures
        let addresses = vec![
            "0x6b175474e89094c44da98b954eedeac495271d0f".to_string(),
            "0x0000000000003f5e74c1ba8a66b48e6f3d71ae82".to_string(),
            "0x00000000009726632680fb29d3f7a9734e3010e2".to_string(),
        ];

        let abis = get_tracked_abi_from_mongo(addresses).await.unwrap();

        println!("abis len: {:?}", abis.len());
        println!("abis: {:?}", abis);

        assert!(abis.len() == 2);
        assert!(abis[0].address == "0x0000000000003f5e74c1ba8a66b48e6f3d71ae82");
    }

}