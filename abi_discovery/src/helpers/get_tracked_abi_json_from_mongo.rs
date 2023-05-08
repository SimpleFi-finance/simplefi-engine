use mongodb::{ Collection, bson::doc };
use futures::stream::StreamExt;

use crate::settings::load_settings;
use shared_types::mongo::abi::{ ContractAbiCollection,  ContractWithAbiJSON, ContractWithAbiJSONDocument};
use third_parties::mongo::lib::abi_discovery::get_default_connection;

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
///    Ok(abis) => info!("abis: {:?}", abis),
///   Err(e) => error!("error: {:?}", e),
/// }
/// ```
///
/// # Panics
///
/// This function will panic if the mongodb_uri is not set in the settings file
///
pub async fn get_tracked_abi_json_from_mongo(
    addresses: Vec<String>,
) -> Result<Vec<ContractWithAbiJSON>, Box<dyn std::error::Error>> {
    let mysettings = load_settings().expect("Failed to load settings");

    let mongo = get_default_connection(&mysettings.mongodb_uri.as_str(), &mysettings.mongodb_database_name.as_str()).await;

    let contract_abi_collection: Collection<ContractAbiCollection> = mongo.collection(&mysettings.mongodb_contract_abi_collection.as_str());

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
    let mut results: Vec<ContractWithAbiJSON> = vec![];

    while let Some(doc) = cursor.next().await {
        let document = bson::from_document::<ContractWithAbiJSONDocument>(doc?)?;

        let result = ContractWithAbiJSON {
            timestamp: document.timestamp,
            address: document.address,
            abi: document.abi,
        };

        results.push(result);
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;

    #[tokio::test]
    async fn test_get_tracked_abi_json_from_mongo() {
        // TODO: implement tear up and down for these fixtures
        let addresses = vec![
            "0x6b175474e89094c44da98b954eedeac495271d0f".to_string(),
            "0x0000000000003f5e74c1ba8a66b48e6f3d71ae82".to_string(),
            "0x00000000009726632680fb29d3f7a9734e3010e2".to_string(),
        ];

        let abis = get_tracked_abi_json_from_mongo(addresses).await.unwrap();

        info!("abis len: {:?}", abis.len());
        info!("abis: {:?}", abis);

        assert!(abis.len() == 2);
        assert!(abis[0].address == "0x0000000000003f5e74c1ba8a66b48e6f3d71ae82");
    }

}
