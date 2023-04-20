use futures::stream::StreamExt;
use log::{ debug, error };
use mongodb::{ bson::{ doc }, error::Error as MongoError };
use std::vec;

use shared_types::mongo::abi::FactoryContractsCollection;
use third_parties::mongo::{MongoConfig, Mongo};

use crate::settings::load_settings;


///
/// Function check_contracts_from_factory
///
/// Check if the addresses are created by a factory contract
///
/// @param addresses: Vec<String> - Addresses to check
/// @return Vec<String> - Addresses that are created by a factory contract
///
pub async fn check_contracts_from_factory(addresses: &Vec<String> ) -> Result<Vec<String>, MongoError> {
    // Create a mongo connection with Mongo helper from shared_types

    let mysettings = load_settings().expect("Failed to load settings");
    let mongo_uri = mysettings.mongodb_uri;
    let mongodb_database_name = mysettings.mongodb_database_name;

    let mongo_config = MongoConfig {
        uri: mongo_uri,
        database: mongodb_database_name,
    };

    // Create a new MongoDB client
    let mongo = Mongo::new(&mongo_config).await.expect("Failed to create mongo Client");

    debug!("Mongo client created");

    let factory_address_collection = mongo.database.collection::<FactoryContractsCollection>("factory_contracts");

    let pipeline = vec![
        doc! {
            "$match": {
                "address": {
                    "$in": addresses
                }
            }
        },
        doc! {
            "$project": {
                "_id": 0,
            }
        }
    ];

    let options = None;

    let mut cursor = factory_address_collection.aggregate(pipeline, options).await.expect("Failed to query the aggregation to get the list of addresses created by factories");

    // Iterate over the results and store them in a vector of strings
    let mut results = Vec::new();

    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                if let Some(value) = document.get("address").and_then(|v| v.as_str()) {
                    debug!("value: {}", value.to_string());

                    results.push(value.to_string());
                }
            }
            Err(e) => error!("Error: {}", e),
        }
    }

    print!("results: {:?}", results);

    Ok(results)
}


#[cfg(test)]
mod tests {
    use super::*;
    use log::{ info };

    #[tokio::test]
    async fn test_check_contracts_from_factory() {
        let addresses = vec!["0xfe78cfe392a33486c87bb0570e94cc1076ca30c7".to_string(), "0xfe66a06310d805ce363fa67194807df74f5b0c18".to_string(), "pepito".to_string()];

        let results = check_contracts_from_factory(&addresses).await.unwrap();

        info!("output {:?}", results);

        assert_eq!(results.len(), 2);
    }
}

