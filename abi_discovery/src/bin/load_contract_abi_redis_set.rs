use futures::StreamExt;
use log::{ info, debug };
use mongodb::bson::doc;
use tokio;

use abi_discovery::settings::load_settings;
use shared_utils::logger::init_logging;
use shared_types::mongo::abi::ContractAbiCollection;
use third_parties::mongo::lib::abi_discovery::get_default_connection;
use third_parties::redis::{add_to_set, connect};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging();

    let mysettings = load_settings().expect("Failed to load settings");

    // Redis connector
    let redis_uri = mysettings.redis_uri.to_string();
    let redis_tracked_addresses_set = mysettings.redis_tracked_addresses_set.to_string();

    let mut con = connect(redis_uri.as_str()).await.unwrap();

    // Mongo connector
    let mongo = get_default_connection(&mysettings.mongodb_uri.as_str(), &mysettings.mongodb_database_name.as_str()).await;

    let contracts = mongo.database.collection::<ContractAbiCollection>(&mysettings.mongodb_contract_abi_collection);

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
