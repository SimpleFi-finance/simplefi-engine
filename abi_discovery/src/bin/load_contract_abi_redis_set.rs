use futures::StreamExt;
use log::{ info, debug };
use shared_utils::logger::init_logging;
use tokio;

use abi_discovery::settings::load_settings;
use mongodb::bson::doc;
use shared_types::mongo::abi::{ContractAbiCollection };
use third_parties::{mongo::{MongoConfig, Mongo}, redis::{add_to_set, connect}};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mysettings = load_settings().expect("Failed to load settings");
    init_logging();
    // Redis connector
    let redis_uri = mysettings.redis_uri.to_string();
    let redis_tracked_addresses_set = mysettings.redis_tracked_addresses_set.to_string();

    let mut con = connect(redis_uri.as_str()).await.unwrap();

    // Mongo connector
    let mongo_uri = mysettings.mongodb_uri;
    let mongodb_database_name = mysettings.mongodb_database_name;
    let contract_abi_collection = mysettings.mongodb_contract_abi_collection;

    let mongo_config = MongoConfig {
        uri: mongo_uri,
        database: mongodb_database_name,
    };

    let mongo = Mongo::new(&mongo_config).await.expect("Failed to create mongo Client");
    let contracts = mongo.database.collection::<ContractAbiCollection>(&contract_abi_collection);

    let mut skip = 0;

    info!("Starting to load contract indexes from MongoDB to Redis Set (verify_addresses) ...");

    loop {
        let options= mongodb::options::FindOptions::builder()
            .limit(500)
            .skip(skip)
            .build();
        let query = doc! {"flag": "Verified" };

        let cursor = contracts.find(query, options).await.expect("Failed to execute find");
        let results = cursor.collect::<Vec<_>>().await;

        if results.is_empty() {
            break;
        }

        debug!("Found {} contract indexes. Skipping {}", results.len(), skip);

        for result in results {
            match result {
                Ok(address) => {
                    let _: () = add_to_set(&mut con, &redis_tracked_addresses_set, &address.address).await.expect("Failed to add to set");
                }
                Err(error_msg) => {
                    println!("Error: {}", error_msg);
                }
            }
        }

        debug!("Finished adding contract indexes to Redis Set (redis_tracked_addresses_set setting)");

        skip += 500;

    }

    Ok(())
}
