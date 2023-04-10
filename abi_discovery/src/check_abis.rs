use mongodb::{ bson::{ doc }, error::Error as MongoError };
use futures::stream::StreamExt;
use redis::RedisError;
use std::vec;

use shared_types::mongo::abi::FactoryContractsCollection;
use third_parties::{redis::{ connect, add_to_set, is_in_set }, mongo::{MongoConfig, Mongo}};
use settings::load_settings;

///
/// Function check_tracked_addresses
///
/// Check if the addresses are tracked
///
/// @param addresses: Vec<String> - Addresses to check
/// @return Vec<String> - Addresses that are tracked
///
pub async fn check_tracked_addresses(addresses: &[String]) -> Result<Vec<String>, RedisError> {
    let set_key = "tracked_addresses";
    let set_verify_key = "verify_addresses";

    let mut tracked_addresses = Vec::new();

    let settings = load_settings().expect("Failed to load settings");
    let redis_uri = settings.redis_uri.to_string();

    let mut con = connect(redis_uri.as_str()).await.unwrap();

    for address in addresses {
        if is_in_set(&mut con, set_key, address).await? {
            tracked_addresses.push(address.clone());
        } else {
            add_to_set(&mut con, set_verify_key, address).await?;
        }
    }

    Ok(tracked_addresses)
}

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
    // TODO: Create helper and pass just client and database and collection
    let mongo_config = MongoConfig {
        uri: "mongodb://localhost:27017".to_string(),
        database: "abi_discovery_2".to_string(),
    };

    // Create a new MongoDB client
    let mongo = Mongo::new(&mongo_config).await.expect("Failed to create mongo Client");

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
                    results.push(value.to_string());
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    print!("results: {:?}", results);

    Ok(results)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_tracked_addresses() {
        let redis_uri = "redis://localhost:6379/";

        let mut con = connect(redis_uri).await.unwrap();

        // generate 20 random ethereum addresses
        let mut addresses = Vec::new();
        for _ in 0..20 {
            let address = format!("0x{}", hex::encode(&rand::random::<[u8; 20]>()));
            addresses.push(address);
        }

        // add 10 of them to the tracked_addresses set
        for address in &addresses[0..10] {
            add_to_set(&mut con, "tracked_addresses", address).await.unwrap();
        }

        // check if the 20 addresses are in the tracked_addresses set
        let tracked_addresses = check_tracked_addresses(&addresses).await.unwrap();

        // check if the 10 addresses are in the tracked_addresses set
        assert_eq!(tracked_addresses.len(), 10);

        // check if the 10 addresses are in the tracked_addresses set
        for address in &addresses[0..10] {
            assert!(tracked_addresses.contains(address));
        }

        // check if the 10 addresses are in the verify_addresses set
        for address in &addresses[10..20] {
            assert!(is_in_set(&mut con, "verify_addresses", address).await.unwrap());
        }
    }

    #[tokio::test]
    async fn test_check_contracts_from_factory() {
        let addresses = vec!["0xfe78cfe392a33486c87bb0570e94cc1076ca30c7".to_string(), "0xfe66a06310d805ce363fa67194807df74f5b0c18".to_string(), "pepito".to_string()];

        let results = check_contracts_from_factory(&addresses).await.unwrap();

        println!("output {:?}", results);

        assert_eq!(results.len(), 2);
    }
}

