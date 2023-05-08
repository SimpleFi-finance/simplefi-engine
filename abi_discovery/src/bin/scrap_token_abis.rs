use futures::StreamExt;
use log::{debug, info, error};
use mongodb::bson::doc;
use shared_utils::logger::init_logging;
use std::error::Error;

use abi_discovery::{
    settings::load_settings,
    types::TokensCollection,
};
use third_parties::{
    mongo::lib::abi_discovery::get_default_connection,
    broker::{publish_rmq_message, create_rmq_channel}
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_logging();

    info!("Starting scrap token abis process...");

    let mysettings = load_settings().expect("Failed to load settings");

    let rabbit_uri = mysettings.rabbit_mq_url.to_string();
    let queue_name = mysettings.rabbit_exchange_name.to_string();
    let exchange_name = format!("{}_exchange", queue_name);
    let routing_key = String::from("abi_discovery");
    let etherscan_keys = mysettings.etherscan_api_keys;

    debug!("Setting rabbit mq connection");
    debug!("Rabbit URI: {}", rabbit_uri);
    debug!("Rabbit Queue Name: {}", queue_name);
    debug!("Rabbit Exchange Name: {}", exchange_name);
    debug!("Router key: {}", routing_key);
    debug!("Etherscan keys: {:?}", etherscan_keys);

    let channel = create_rmq_channel(&mysettings.rabbit_mq_url)
        .await
        .expect("Failed to create channel");

    let mongo = get_default_connection(&mysettings.mongodb_uri.as_str(), &mysettings.mongodb_database_name.as_str()).await;

    let core_data_collection = mongo.collection::<TokensCollection>("tokens");

    let mut skip = 0;

    info!("Starting to load contract indexes from MongoDB to Redis Set (verify_addresses) ...");

    loop {
        let query = doc! {
            "chain": "ethereum",
        };

        let options = mongodb::options::FindOptions::builder()
            .limit(500)
            .skip(skip)
            .build();

        let cursor = core_data_collection
            .find(query, options)
            .await
            .expect("Failed to execute find");

        let results = cursor.collect::<Vec<_>>().await;

        if results.is_empty() {
            break;
        }

        debug!("Found {} contract indexes. Skipping {}", results.len(), skip);

        for result in results {
            match result {
                Ok(token) => {
                    debug!("Token address: {:?}", token.address);

                    let address = token.address;

                    publish_rmq_message(&exchange_name, &routing_key, &address.as_bytes(), &channel)
                                .await
                                .expect("Failed to publish message");

                    continue;
                }
                Err(error_msg) => {
                    error!("Error: {}", error_msg);
                }
            }
        }

        skip += 500;
    }

    Ok(())
}
